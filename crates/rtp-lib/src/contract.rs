use crate::{ContractError, StorageKey};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::UnorderedMap,
    AccountId, PanicOnDefault, Promise,
};
use rtp_contract_common::{
    get_bank_id, get_partnership_id, DealStatus, PaymentConfirmation, Payments, RtpEvent, Trade,
    TradeDetails,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    factory: AccountId,
    bank: String,
    trades: UnorderedMap<String, Trade>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(factory: AccountId, bank: String) -> Self {
        Self {
            factory,
            bank,
            trades: UnorderedMap::new(StorageKey::Trades),
        }
    }

    #[handle_result]
    pub fn perform_trade(&mut self, trade_details: TradeDetails) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        self.trades.insert(
            trade_details.trade_id.clone(),
            Trade {
                bank: self.bank.clone(),
                trade_details: trade_details.clone(),
                deal_status: DealStatus::Pending,
                payments: Payments::default(),
            },
        );
        let partnership_id =
            get_partnership_id(self.bank.clone(), trade_details.counterparty.clone());

        let event: RtpEvent = RtpEvent::SendTrade {
            partnership_id,
            bank_id: get_bank_id(&self.bank),
            trade: trade_details,
        };
        event.emit();

        Ok(())
    }

    #[handle_result]
    pub fn settle_trade(
        &mut self,
        trade_id: String,
        deal_status: DealStatus,
    ) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        let trade = self
            .trades
            .get_mut(&trade_id)
            .ok_or(ContractError::InvalidTradeId)?;
        trade.deal_status = deal_status;

        Ok(())
    }

    #[handle_result]
    pub fn confirm_payment(
        &mut self,
        trade_id: String,
        confirmation: PaymentConfirmation,
    ) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        let trade = self
            .trades
            .get_mut(&trade_id)
            .ok_or(ContractError::InvalidTradeId)?;
        match confirmation {
            PaymentConfirmation::Credit => trade.payments.credit = true,
            PaymentConfirmation::Debit => trade.payments.debit = true,
        }
        let partnership_id =
            get_partnership_id(self.bank.clone(), trade.trade_details.counterparty.clone());

        let event = RtpEvent::ConfirmPayment {
            partnership_id,
            bank_id: get_bank_id(&self.bank),
            trade_id: trade.trade_details.trade_id.clone(),
            confirmation,
        };
        event.emit();

        Ok(())
    }

    #[handle_result]
    pub fn delete_account(&mut self) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        Promise::new(env::current_account_id())
            .delete_account(self.factory.clone())
            .as_return();

        Ok(())
    }

    #[handle_result]
    pub fn get_trade(&self, trade_id: String) -> Result<Trade, ContractError> {
        let trade = self
            .trades
            .get(&trade_id)
            .ok_or(ContractError::InvalidTradeId)?;
        Ok(trade.clone())
    }
}

use crate::{ContractError, StorageKey};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    store::UnorderedMap,
    AccountId, PanicOnDefault, Promise,
};
use rtp_contract_common::{DealStatus, RtpEventBindgen, Trade, TradeDetails};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    factory: AccountId,
    partnership_id: String,
    bank_a: String,
    bank_b: String,
    trades: UnorderedMap<String, Trade>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(factory: AccountId, partnership_id: String, bank_a: String, bank_b: String) -> Self {
        Self {
            factory,
            partnership_id,
            bank_a,
            bank_b,
            trades: UnorderedMap::new(StorageKey::Trades),
        }
    }

    #[handle_result]
    pub fn perform_trade(
        &mut self,
        bank: String,
        trade_details: TradeDetails,
    ) -> Result<(), ContractError> {
        if env::predecessor_account_id() != self.factory {
            return Err(ContractError::NotFactory);
        }

        if bank != self.bank_a && bank != self.bank_b {
            return Err(ContractError::InvalidBank);
        }

        if let Some(trade) = self.trades.get_mut(&trade_details.trade_id) {
            if bank == self.bank_a {
                trade.trade_a = Some(trade_details.clone());
            } else {
                trade.trade_b = Some(trade_details.clone());
            }
        } else {
            self.trades.insert(
                trade_details.trade_id.clone(),
                if bank == self.bank_a {
                    Trade {
                        trade_a: Some(trade_details.clone()),
                        trade_b: None,
                        deal_status: DealStatus::Pending,
                    }
                } else {
                    Trade {
                        trade_a: None,
                        trade_b: Some(trade_details.clone()),
                        deal_status: DealStatus::Pending,
                    }
                },
            );
        }

        let event: RtpEventBindgen = RtpEventBindgen::SendTrade {
            partnership_id: self.partnership_id.clone(),
            bank,
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
        if trade.trade_a.is_none() || trade.trade_b.is_none() {
            return Err(ContractError::TradeIncomplete);
        }

        trade.deal_status = deal_status.clone();

        let event = RtpEventBindgen::SettleTrade {
            partnership_id: self.partnership_id.clone(),
            trade_id,
            deal_status,
        };
        event.emit();

        Ok(())
    }

    #[handle_result]
    pub fn remove_partnership(&mut self) -> Result<(), ContractError> {
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

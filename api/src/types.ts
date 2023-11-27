export type BatchData = {
  block_height: number;
  timestamp: number;
  events: Event[];
};

export type TradeDetails = {
  trade_id: string;
  timestamp: number;
  side: 'Buy' | 'Sell';
  counterparty: string;
} & Record<
  string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  any
>;

export type SendTradeData = {
  partnership_id: string;
  bank_id: string;
  trade: TradeDetails;
};

export type ConfirmPaymentData = {
  partnership_id: string;
  bank_id: string;
  trade_id: string;
  confirmation: PaymentConfirmation;
};

export type DealStatus =
  | { status: 'Pending' }
  | {
      status: 'Confirmed' | 'Rejected' | 'Executed';
      message: string;
    };

export type Payments = {
  credit: boolean;
  debit: boolean;
};

export type Event =
  | {
      event: 'new_bank';
      data: {
        bank: string;
        bank_id: string;
      };
    }
  | {
      event: 'send_trade';
      data: SendTradeData;
    }
  | {
      event: 'settle_trade';
      data: {
        partnership_id: string;
        trade_id: string;
        deal_status: DealStatus;
      };
    }
  | {
      event: 'confirm_payment';
      data: ConfirmPaymentData;
    };

export type Trade = {
  bank: string;
  trade_details: TradeDetails;
  deal_status: DealStatus;
  payments: Payments;
};

export type PaymentConfirmation = 'Credit' | 'Debit';

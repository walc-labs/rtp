export type BatchData = {
  block_height: number;
  timestamp: number;
  events: Event[];
};

export type TradeDetails = {
  trade_id: string;
  event_timestamp: number;
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

export type MatchingStatus =
  | { status: 'Pending' | 'Error' }
  | {
      status: 'Confirmed' | 'Rejected';
      message: string;
    };

export type PaymentStatus =
  | { status: 'Pending' | 'Error' }
  | {
      status: 'Confirmed' | 'Rejected';
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
      event: 'set_matching_status';
      data: {
        partnership_id: string;
        trade_id: string;
        matching_status: MatchingStatus;
      };
    }
  | {
      event: 'confirm_payment';
      data: ConfirmPaymentData;
    }
  | {
      event: 'set_payment_status';
      data: {
        partnership_id: string;
        trade_id: string;
        payment_status: PaymentStatus;
      };
    };

export type Trade = {
  bank: string;
  trade_details: TradeDetails;
  matching_status: MatchingStatus;
  payment_status: PaymentStatus;
  payments: Payments;
};

export type PaymentConfirmation = 'Credit' | 'Debit';

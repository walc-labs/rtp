export type BatchData = {
  block_height: number;
  timestamp: number;
  events: Event[];
};

export type TradeDetails = {
  trade_id: string;
  timestamp: number;
  side: 'Buy' | 'Sell';
} & Record<
  string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  any
>;

export type SendTradeData = {
  partnership_id: string;
  bank: string;
  trade: TradeDetails;
};

export type DealStatus =
  | { status: 'Pending' }
  | {
      status: 'Confirmed' | 'Rejected' | 'Executed';
      message: string;
    };

export type Event =
  | {
      event: 'new_partnership';
      data: {
        partnership_id: string;
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
    };

export type Trade = {
  trade_a: TradeDetails;
  trade_b: TradeDetails;
  deal_status: DealStatus;
};

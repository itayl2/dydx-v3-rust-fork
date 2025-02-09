use std::any::Any;
use std::cmp;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use chrono::DateTime;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal::prelude::ToPrimitive;
use strum_macros::{Display, EnumString};

pub type OrdersResponse = Vec<OrderResponseObject>;

const QUOTE_QUANTUMS_ATOMIC_RESOLUTION: i64 = -6;
const USDC_ATOMIC_RESOLUTION: i64 = -6;

#[cfg(not(feature = "backtest"))]
pub type CreateOrderResponse = InternalApiResponse;
#[cfg(feature = "backtest")]
pub type CreateOrderResponse = OrderResponseObject;

#[cfg(not(feature = "backtest"))]
pub type CancelOrderResponse = InternalApiResponse;
#[cfg(feature = "backtest")]
pub type CancelOrderResponse = OrderResponseObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSizeParams {
    pub market: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSizeObject {
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceStepParams {
    pub market: String,
    pub price: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceStepObject {
    pub price: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiOrderParams {
    pub market: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: Decimal,
    pub price: Decimal,
    #[serde(skip_serializing, default)]
    pub intended_price: Decimal,
    pub time_in_force: APITimeInForce,
    pub client_id: String,
    pub reduce_only: bool,
    pub good_til_block_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_order_trigger_subticks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<String>,
}

impl ApiOrderParams {
    pub fn set_size(&mut self, size: Decimal) {
        self.size = size;
    }
    
    pub fn get_size(&self) -> Decimal {
        self.size
    }

    pub fn get_price(&self) -> Decimal {
        self.price
    }

    pub fn has_stop_price(&self) -> bool {
        self.conditional_order_trigger_subticks.is_some()
    }

    pub fn get_market(&self) -> String {
        self.market.clone()
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.clone()
    }
    
    pub fn get_intended_price(&self) -> Decimal {
        self.intended_price
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    pub market: String,
    pub client_id: String,
    pub order_type: OrderType,
    pub good_til_block_time: Option<i64>,
    pub good_til_block: Option<i64>,
    #[cfg(feature = "backtest")]
    pub order_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalApiResponse {
    pub hash: String,
    pub code: i64,
    pub raw_log: String,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: OrderResponseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseObject {
    pub id: String,
    pub subaccount_id: Option<String>,
    pub client_id: String,
    pub clob_pair_id: String,
    pub side: OrderSide,
    pub size: Decimal,
    pub total_filled: Option<Decimal>,
    pub price: Decimal,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub order_flags: String,
    pub good_til_block: Option<String>,
    pub good_til_block_time: Option<String>,
    pub created_at_height: Option<String>,
    pub client_metadata: String,
    pub trigger_price: Option<Decimal>,
    pub time_in_force: APITimeInForce,
    pub status: APIOrderStatus,
    pub post_only: bool,
    pub ticker: String,
    pub updated_at: Option<String>,
    pub updated_at_height: Option<String>,
    pub subaccount_number: i32,
}

impl OrderResponseObject {
    pub fn get_total_filled(&self) -> Decimal {
        match self.total_filled.as_ref() {
            Some(total_filled) => total_filled.clone(),
            None => Decimal::ZERO,
        }
    }

    // example value: "2021-01-05T16:33:43.163Z"
    pub fn get_updated_timestamp(&self) -> Option<i64> {
        let updated_at = match self.updated_at.as_ref() {
            Some(updated_at) => updated_at,
            None => return None,
        };

        match DateTime::parse_from_rfc3339(&updated_at) {
            Ok(datetime) => Some(datetime.timestamp_millis()),
            Err(error) => {
                eprintln!("Failed to parse updated_at timestamp: {error:?}, updated_at: {updated_at:?}");
                Some(0)
            }
        }
    }

    pub fn get_client_id(&self) -> String {
        self.client_id.clone()
    }

    pub fn get_raw_order_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_order_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_market(&self) -> String {
        self.ticker.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
}

impl OrderType {
    pub fn requires_trigger_price(&self) -> bool {
        match self {
            OrderType::Limit => false,
            OrderType::Market => false,
            OrderType::StopLimit => true,
            OrderType::StopMarket => true,
            OrderType::TakeProfit => true,
            OrderType::TakeProfitMarket => true,
        }
    }

    pub fn get_stop_values() -> Vec<OrderType> {
        vec![
            OrderType::StopLimit,
            OrderType::StopMarket,
        ]
    }

    pub fn get_take_profit_values() -> Vec<OrderType> {
        vec![
            OrderType::TakeProfit,
            OrderType::TakeProfitMarket,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq, Hash)]
pub enum APITimeInForce {
    GTT,
    IOC,
    FOK,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum APIOrderStatus {
    Open,
    Filled,
    Canceled,
    BestEffortCanceled,
    Untriggered,
    BestEffortOpened,
}

impl APIOrderStatus {
    pub fn first() -> Self {
        Self::BestEffortOpened
    }

    pub fn get_verb(&self) -> String {
        match self {
            Self::Untriggered => "created",
            Self::Open => "opened",
            Self::BestEffortOpened => "opened",
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::BestEffortCanceled => "canceled",
        }.to_string()
    }

    pub fn is_open(&self) -> bool {
        Self::get_open_statuses().contains(self)
    }

    pub fn get_canceled_statuses() -> Vec<Self> {
        vec![Self::Canceled, Self::BestEffortCanceled]
    }

    pub fn get_open_statuses() -> Vec<Self> {
        vec![Self::BestEffortOpened, Self::Open, Self::Untriggered]
    }

    pub fn get_closed_statuses() -> Vec<Self> {
        vec![Self::Filled, Self::Canceled, Self::BestEffortCanceled]
    }

    pub fn get_step_number(&self) -> f64 {
        match self {
            Self::BestEffortOpened => 0.0,
            Self::Untriggered => 1.0,
            Self::Open => 1.0,
            Self::BestEffortCanceled => 1.5,
            Self::Canceled => 2.0,
            Self::Filled => 2.0,
        }
    }

    pub fn is_filled(&self) -> bool {
        match self {
            Self::Filled => true,
            _ => false,
        }
    }

    pub fn is_canceled(&self) -> bool {
        match self {
            Self::Canceled => true,
            Self::BestEffortCanceled => true,
            _ => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_canceled() || self.is_filled()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressResponse {
    pub subaccounts: Vec<SubaccountResponseObject>,
    pub total_trading_rewards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountResponseInnerObject {
    pub address: String,
    pub subaccount_number: i32,
    pub equity: Decimal,
    pub free_collateral: Decimal,
    pub open_perpetual_positions: PerpetualPositionsMap,
    pub asset_positions: AssetPositionsMap,
    pub margin_enabled: bool,
    pub updated_at_height: String,
    pub latest_processed_block_height: String,
}

impl SubaccountResponseInnerObject {
    pub fn get_total_unrealized_pnl(&self) -> Decimal {
        self.open_perpetual_positions
            .values()
            .map(|position| position.unrealized_pnl)
            .sum()
    }

    pub fn get_total_value(&self) -> Decimal {
        self.open_perpetual_positions
            .values()
            .map(|position| position.get_value())
            .sum()
    }

    pub fn get_open_position_leverage(&self) -> Decimal {
        let total_asset_value = self.get_total_value();
        let total_unrealized_pnl = self.get_total_unrealized_pnl();
        (total_asset_value + total_unrealized_pnl) / self.equity
    }

    pub fn get_equity(&self) -> Decimal {
        self.equity
    }

    pub fn get_free_collateral(&self) -> Decimal {
        self.free_collateral
    }

    pub fn get_all_positions(&self) -> Vec<PerpetualPositionResponseObject> {
        self.open_perpetual_positions.values().cloned().collect()
    }

    pub fn get_position(&self, symbol: &str) -> Option<PerpetualPositionResponseObject> {
        self.open_perpetual_positions.get(symbol).cloned()
    }

    pub fn get_position_size(&self, symbol: &str) -> Option<Decimal> {
        self.get_position(symbol).map(|position| position.size)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountResponseObject {
    pub subaccount: SubaccountResponseInnerObject,
}

impl Default for SubaccountResponseObject {
    fn default() -> Self {
        Self {
            subaccount: SubaccountResponseInnerObject::default(),
        }
    }
}

impl Default for SubaccountResponseInnerObject {
    fn default() -> Self {
        Self {
            address: String::default(),
            subaccount_number: i32::default(),
            equity: Decimal::ZERO,
            free_collateral: Decimal::ZERO,
            open_perpetual_positions: PerpetualPositionsMap::default(),
            asset_positions: AssetPositionsMap::default(),
            margin_enabled: false,
            updated_at_height: String::default(),
            latest_processed_block_height: String::default(),
        }
    }
}

impl SubaccountResponseInnerObject {
    pub fn get_asset_balance(&self, asset: &str) -> Decimal {
        match self.asset_positions.get(asset) {
            Some(position) => position.size.clone(),
            None => Decimal::ZERO,
        }
    }

    pub fn get_quote_balance(&self) -> Decimal {
        match self.asset_positions.get("USDC") {
            Some(position) => position.size.clone(),
            None => Decimal::ZERO,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountWebSocketObject {
    pub address: String,
    pub subaccount_number: i32,
    pub equity: Decimal,
    pub free_collateral: Decimal,
    pub open_perpetual_positions: PerpetualPositionsMap,
    pub asset_positions: AssetPositionsMap,
    pub margin_enabled: bool,
}

pub type PerpetualPositionsMap = HashMap<String, PerpetualPositionResponseObject>;
pub type AssetPositionsMap = HashMap<String, AssetPositionResponseObject>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualPositionResponseObject {
    pub market: String,
    pub status: PerpetualPositionStatus,
    pub side: PositionSide,
    pub size: Decimal,
    pub max_size: Decimal,
    pub entry_price: Decimal,
    pub realized_pnl: Decimal,
    pub created_at: String,
    pub created_at_height: String,
    pub sum_open: Decimal,
    pub sum_close: Decimal,
    pub net_funding: Decimal,
    pub unrealized_pnl: Decimal,
    pub closed_at: Option<String>,
    pub exit_price: Option<Decimal>,
    pub subaccount_number: i32,
}

impl PerpetualPositionResponseObject {
    pub fn get_value(&self) -> Decimal {
        self.size * self.get_entry_price()
    }

    pub fn get_size(&self) -> Decimal {
        self.size
    }

    pub fn get_unrealized_pnl(&self) -> Decimal {
        self.unrealized_pnl
    }

    pub fn is_short(&self) -> bool {
        self.side == PositionSide::SHORT
    }
    
    pub fn is_long(&self) -> bool {
        self.side == PositionSide::LONG
    }

    pub fn get_market(&self) -> String {
        self.market.clone()
    }

    pub fn get_side(&self) -> PositionSide {
        self.side.clone()
    }
    
    pub fn get_entry_price(&self) -> Decimal {
        self.entry_price
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum PerpetualPositionStatus {
    OPEN,
    CLOSED,
    LIQUIDATED,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum PositionSide {
    LONG,
    SHORT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionResponseObject {
    pub symbol: String,
    pub side: PositionSide,
    pub size: Decimal,
    pub asset_id: String,
    pub subaccount_number: i32,
}

/**
export interface ResponseWithBody extends express.Response {
  body: unknown,x
}
 **/
pub struct ResponseWithBody {
    pub body: Box<dyn Any>,
}

/**
export enum RequestMethod {
  DELETE = 'DELETE',
  GET = 'GET',
  POST = 'POST',
  PUT = 'PUT',
}
 **/
#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
pub enum RequestMethod {
    DELETE,
    GET,
    POST,
    PUT,
}

/**
export interface PaginationResponse {
  pageSize?: number,
  totalResults?: number,
  offset?: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

/**
export interface ParentSubaccountResponse {
  address: string,
  parentSubaccountNumber: number,
  equity: string,
  freeCollateral: string,
  childSubaccounts: SubaccountResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountResponse {
    pub address: String,
    pub parent_subaccount_number: u64,
    pub equity: Decimal,
    pub free_collateral: Decimal,
    pub child_subaccounts: Vec<SubaccountResponseObject>,
}

/**
export type SubaccountById = {[id: string]: SubaccountFromDatabase};
 **/
pub type SubaccountById = HashMap<String, SubaccountFromDatabase>;

/**
export interface TimeResponse {
  iso: IsoString,
  epoch: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeResponse {
    pub iso: String,  // Assuming IsoString is just a type alias for string
    pub epoch: f64,   // Using f64 as it can represent larger numbers than i64
}

/**
export interface PerpetualPositionResponse {
  positions: PerpetualPositionResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionResponse {
    pub positions: Vec<PerpetualPositionResponseObject>,
}

/**
export interface PerpetualPositionWithFunding extends PerpetualPositionFromDatabase {
  unsettledFunding: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualPositionWithFunding {
    #[serde(flatten)]
    pub base: PerpetualPositionFromDatabase,
    pub unsettled_funding: String,
}

/**
export interface AssetPositionResponse {
  positions: AssetPositionResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPositionResponse {
    pub positions: Vec<AssetPositionResponseObject>,
}

/**
export interface FillResponse extends PaginationResponse {
  fills: FillResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct FillResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub fills: Vec<FillResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FillResponseSinglePage {
    pub fills: Vec<FillResponseObject>,
}

/**
export interface FillResponseObject {
  id: string,
  side: OrderSide,
  liquidity: Liquidity,
  type: FillType,
  market: string,
  marketType: MarketType,
  price: string,
  size: string,
  fee: string,
  affiliateRevShare: string,
  createdAt: IsoString,
  createdAtHeight: string,
  orderId?: string,
  clientMetadata?: string,
  subaccountNumber: number,
}
 **/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillResponseObject {
    pub id: String,
    pub side: OrderSide,
    pub liquidity: Liquidity,
    #[serde(rename = "type")]
    pub fill_type: FillType,
    pub market: String,
    pub market_type: MarketType,
    pub price: Decimal,
    pub size: Decimal,
    pub fee: Decimal,
    pub affiliate_rev_share: Option<String>,
    pub created_at: String,
    pub created_at_height: String,
    pub order_id: Option<String>,
    pub client_metadata: Option<String>,
    pub subaccount_number: i32,
}

impl FillResponseObject {
    pub fn get_order_id(&self) -> String {
        self.order_id.clone().unwrap_or_default()
    }

    pub fn get_market(&self) -> String {
        self.market.clone()
    }

    pub fn get_price_or_zero(&self) -> Decimal {
        self.price
    }
}

/**
export interface TransferResponse extends PaginationResponse {
  transfers: TransferResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub transfers: Vec<TransferResponseObject>,
}

/**
export interface TransferResponseObject {
  id: string,
  sender: {
    address: string,
    subaccountNumber?: number,
  },
  recipient: {
    address: string,
    subaccountNumber?: number,
  },
  size: string,
  createdAt: string,
  createdAtHeight: string,
  symbol: string,
  type: TransferType,
  transactionHash: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponseObject {
    pub id: String,
    pub sender: TransferParty,
    pub recipient: TransferParty,
    pub size: String,
    pub created_at: String,
    pub created_at_height: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferParty {
    pub address: String,
    pub subaccount_number: Option<f64>,
}

/**
export interface ParentSubaccountTransferResponse extends PaginationResponse {
  transfers: TransferResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParentSubaccountTransferResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub transfers: Vec<TransferResponseObject>,
}

/**
export interface ParentSubaccountTransferResponseObject {
  id: string,
  sender: {
    address: string,
    parentSubaccountNumber?: number,
  },
  recipient: {
    address: string,
    parentSubaccountNumber?: number,
  },
  size: string,
  createdAt: string,
  createdAtHeight: string,
  symbol: string,
  type: TransferType,
  transactionHash: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountTransferResponseObject {
    pub id: String,
    pub sender: ParentTransferParty,
    pub recipient: ParentTransferParty,
    pub size: String,
    pub created_at: String,
    pub created_at_height: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTransferParty {
    pub address: String,
    pub parent_subaccount_number: Option<f64>,
}

/**
export interface TransferBetweenResponse extends PaginationResponse {
  transfersSubset: TransferResponseObject[],
  totalNetTransfers: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub transfers_subset: Vec<TransferResponseObject>,
    pub total_net_transfers: String,
}

/**
export interface HistoricalPnlResponse extends PaginationResponse {
  historicalPnl: PnlTicksResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPnlResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub historical_pnl: Vec<PnlTicksResponseObject>,
}

/**
export interface PnlTicksResponseObject {
  id: string,
  subaccountId: string,
  equity: string,
  totalPnl: string,
  netTransfers: string,
  createdAt: string,
  blockHeight: string,
  blockTime: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnlTicksResponseObject {
    pub id: String,
    pub subaccount_id: String,
    pub equity: Decimal,
    pub total_pnl: String,
    pub net_transfers: String,
    pub created_at: String,
    pub block_height: String,
    pub block_time: String,
}

/**
export interface TradeResponse extends PaginationResponse {
  trades: TradeResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponse {
    #[serde(flatten)]
    pub pagination: PaginationResponse,
    pub trades: Vec<TradeResponseObject>,
}

/**
export interface TradeResponseObject {
  id: string,
  side: OrderSide,
  size: string,
  price: string,
  type: TradeType,
  createdAt: IsoString,
  createdAtHeight: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponseObject {
    pub id: String,
    pub side: OrderSide,
    pub size: String,
    pub price: String,
    #[serde(rename = "type")]
    pub trade_type: TradeType,
    pub created_at: String,
    pub created_at_height: String,
}

/**
export interface HeightResponse {
  height: string,
  time: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct HeightResponse {
    pub height: String,
    pub time: String,
}

/**
export type AssetById = {[assetId: string]: AssetFromDatabase};
 **/
pub type AssetById = HashMap<String, AssetFromDatabase>;

/**
export interface MarketAndType {
  marketType: MarketType,
  market: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketAndType {
    pub market_type: MarketType,
    pub market: String,
}

/**
export type MarketAndTypeByClobPairId = {[clobPairId: string]: MarketAndType};
 **/
pub type MarketAndTypeByClobPairId = HashMap<String, MarketAndType>;

/**
export enum MarketType {
  PERPETUAL = 'PERPETUAL',
  SPOT = 'SPOT',
}
 **/
#[derive(Debug, Serialize, Deserialize, Display, EnumString, Clone, PartialEq, Eq, Hash)]
pub enum MarketType {
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "SPOT")]
    Spot,
}

pub type PerpetualMarketResponseMap = HashMap<String, PerpetualMarketResponseObject>;

/**
export interface PerpetualMarketResponse {
  markets: {
    [ticker: string]: PerpetualMarketResponseObject,
  },
}
 **/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerpetualMarketResponse {
    pub markets: PerpetualMarketResponseMap,
}

/**
export interface PerpetualMarketResponseObject {
  clobPairId: string,
  ticker: string,
  status: PerpetualMarketStatus,
  oraclePrice: string,
  priceChange24H: string,
  volume24H: string,
  trades24H: number,
  nextFundingRate: string,
  initialMarginFraction: string,
  maintenanceMarginFraction: string,
  openInterest: string,
  atomicResolution: number,
  quantumConversionExponent: number,
  tickSize: string,
  stepSize: string,
  stepBaseQuantums: number,
  subticksPerTick: number,
  marketType: PerpetualMarketType,
  openInterestLowerCap?: string,
  openInterestUpperCap?: string,
  baseOpenInterest: string,
}
 **/
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualMarketResponseObject {
    pub clob_pair_id: String,
    pub ticker: String,
    pub status: PerpetualMarketStatus,
    pub oracle_price: Decimal,
    pub price_change24H: Decimal,
    pub volume24H: Decimal,
    pub trades24H: i64,
    pub next_funding_rate: Decimal,
    pub initial_margin_fraction: Decimal,
    pub maintenance_margin_fraction: Decimal,
    pub open_interest: Decimal,
    pub atomic_resolution: i64,
    pub quantum_conversion_exponent: i64,
    pub min_order_size: Decimal,
    pub tick_size: Decimal,
    pub step_size: Decimal,
    pub step_base_quantums: i64,
    pub subticks_per_tick: i64,
    pub market_type: PerpetualMarketType,
    pub open_interest_lower_cap: Option<String>,
    pub open_interest_upper_cap: Option<String>,
    pub base_open_interest: Decimal,
    pub step_scale: usize,
}

impl<'de> Deserialize<'de> for PerpetualMarketResponseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a temporary struct that matches the JSON structure
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct TempObject {
            clob_pair_id: String,
            ticker: String,
            status: PerpetualMarketStatus,
            oracle_price: Decimal,
            price_change24H: Decimal,
            volume24H: Decimal,
            trades24H: i64,
            next_funding_rate: Decimal,
            initial_margin_fraction: Decimal,
            maintenance_margin_fraction: Decimal,
            open_interest: Decimal,
            atomic_resolution: i64,
            quantum_conversion_exponent: i64,
            tick_size: Decimal,
            step_size: Decimal,
            step_base_quantums: i64,
            subticks_per_tick: i64,
            market_type: PerpetualMarketType,
            open_interest_lower_cap: Option<String>,
            open_interest_upper_cap: Option<String>,
            base_open_interest: Decimal,
        }

        // Deserialize into the temporary struct
        let temp = TempObject::deserialize(deserializer)?;

        let mut perpetual_market_response_object = PerpetualMarketResponseObject {
            clob_pair_id: temp.clob_pair_id,
            ticker: temp.ticker,
            status: temp.status,
            oracle_price: temp.oracle_price,
            price_change24H: temp.price_change24H,
            volume24H: temp.volume24H,
            trades24H: temp.trades24H,
            next_funding_rate: temp.next_funding_rate,
            initial_margin_fraction: temp.initial_margin_fraction,
            maintenance_margin_fraction: temp.maintenance_margin_fraction,
            open_interest: temp.open_interest,
            atomic_resolution: temp.atomic_resolution,
            quantum_conversion_exponent: temp.quantum_conversion_exponent,
            min_order_size: Decimal::ZERO,
            step_scale: temp.step_size.to_f64().unwrap().recip().log10().round() as usize,
            tick_size: temp.tick_size,
            step_size: temp.step_size,
            step_base_quantums: temp.step_base_quantums,
            subticks_per_tick: temp.subticks_per_tick,
            market_type: temp.market_type,
            open_interest_lower_cap: temp.open_interest_lower_cap,
            open_interest_upper_cap: temp.open_interest_upper_cap,
            base_open_interest: temp.base_open_interest,
        };
        perpetual_market_response_object.min_order_size = perpetual_market_response_object.quantums_to_size(perpetual_market_response_object.step_base_quantums);
        Ok(perpetual_market_response_object)
    }
}

impl PerpetualMarketResponseObject {
    pub fn quantums_to_size(&self, quantums: i64) -> Decimal {
        // let atomic_resolution = match atomic_resolution > 0 {
        //     true => -self.atomic_resolution,
        //     false => self.atomic_resolution,
        // };
        let quantums_decimal = Decimal::from(quantums);
        let factor = Decimal::TEN.powi(self.atomic_resolution);
        quantums_decimal * factor
    }

    pub fn get_maintenance_margin_requirement(&self, position_size: Decimal, price: Option<Decimal>) -> Decimal {
        let price = match price {
            Some(price) => price,
            None => self.oracle_price,
        };
        (position_size * price * self.maintenance_margin_fraction).abs()
    }

    pub fn get_initial_margin_requirement(&self, position_size: Decimal, price: Option<Decimal>) -> Decimal {
        let price = match price {
            Some(price) => price,
            None => self.oracle_price,
        };
        (position_size * price * self.initial_margin_fraction).abs()
    }

    fn round_down(x: Decimal, base: i64) -> Decimal {
        let base_dec = Decimal::from(base);
        // Compute floor(x / base) * base
        (x / base_dec).floor() * base_dec
    }

    pub fn size_to_quantums(&self, size: Decimal) -> i64 {
        // Step 1: Compute raw_quantums = size × 10^( -atomicResolution )
        let factor = Decimal::new(10, 0).powi(-self.atomic_resolution);
        let raw_quantums = size * factor;

        // Step 2: Round down raw_quantums to the nearest multiple of stepBaseQuantums
        let quantums_dec = Self::round_down(raw_quantums, self.step_base_quantums);

        // Step 3: Convert quantums to integer
        let quantums_int = quantums_dec.to_i64().unwrap();

        // Step 4: Ensure quantums is at least stepBaseQuantums
        cmp::max(quantums_int, self.step_base_quantums)
    }

    pub fn calculate_subticks(&self, price: Decimal) -> i64 {
        // Step 1: Compute exponent = atomicResolution - quantumConversionExponent - QUOTE_QUANTUMS_ATOMIC_RESOLUTION
        let exponent = self.atomic_resolution - self.quantum_conversion_exponent - QUOTE_QUANTUMS_ATOMIC_RESOLUTION;

        // Step 2: Compute raw_subticks = price × 10^exponent
        let factor = Decimal::new(10, 0).powi(exponent);
        let raw_subticks = price * factor;

        // Step 3: Round down raw_subticks to the nearest multiple of subticksPerTick
        let subticks_dec = Self::round_down(raw_subticks, self.subticks_per_tick);

        // Step 4: Convert subticks to integer
        let subticks_int = subticks_dec.to_i64().unwrap();

        // Step 5: Ensure subticks is at least subticksPerTick
        cmp::max(subticks_int, self.subticks_per_tick)
    }

    pub fn round_order_size(&self, size: Decimal) -> Decimal {
        let quantums = self.size_to_quantums(size);
        self.quantums_to_size(quantums).normalize()
    }

    pub fn get_order_price(&self, price: Decimal) -> Decimal {
        let subticks = self.calculate_subticks(price);
        self.subticks_to_price(subticks).normalize()
    }

    pub fn subticks_to_price(&self, subticks: i64) -> Decimal {
        let quantum_conversion_exponent = match self.quantum_conversion_exponent > 0 {
            true => -self.quantum_conversion_exponent,
            false => self.quantum_conversion_exponent,
        };
        // Step 1: Compute exponent = atomicResolution - quantumConversionExponent - USDC_ATOMIC_RESOLUTION
        let exponent = self.atomic_resolution - quantum_conversion_exponent - USDC_ATOMIC_RESOLUTION;

        // Step 2: Compute factor = 10^exponent
        let base = Decimal::new(10, 0);
        let factor = base.powi(exponent);

        // Step 3: Compute price = subticks / factor
        let subticks_dec = Decimal::from(subticks);
        subticks_dec / factor
    }
}

pub type PriceLevel = Vec<String>;

/**
export interface OrderbookResponseObject {
  bids: OrderbookResponsePriceLevel[],
  asks: OrderbookResponsePriceLevel[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookResponseObject {
    pub bids: Vec<OrderbookResponsePriceLevel>,
    pub asks: Vec<OrderbookResponsePriceLevel>,
}

/**
export interface OrderbookResponsePriceLevel {
  price: string,
  size: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookResponsePriceLevel {
    pub price: String,
    pub size: String,
}

/**
export type RedisOrderMap = { [orderId: string]: RedisOrder };
 **/
pub type RedisOrderMap = HashMap<String, RedisOrder>;

/**
export type PostgresOrderMap = { [orderId: string]: OrderFromDatabase };
 **/
pub type PostgresOrderMap = HashMap<String, OrderFromDatabase>;

/**
export interface CandleResponse {
  candles: CandleResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct CandleResponse {
    pub candles: Vec<CandleResponseObject>,
}

/**
export interface CandleResponseObject extends Omit<CandleFromDatabase, CandleColumns.id> {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleResponseObject {
    pub started_at: String,
    pub ticker: String,
    pub resolution: CandleResolution,
    pub low: Decimal,
    pub high: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub base_token_volume: String,
    pub usd_volume: String,
    pub trades: i64,
    pub starting_open_interest: String,
    pub open_interest: String,
    pub closing_open_interest: String,
    pub ordering_bookmark: Option<String>,
}

/**
export interface SparklineResponseObject {
  [ticker: string]: string[],
}
 **/
pub type SparklineResponseObject = HashMap<String, Vec<String>>;

/**
export enum SparklineTimePeriod {
  ONE_DAY = 'ONE_DAY',
  SEVEN_DAYS = 'SEVEN_DAYS',
}
 **/
#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
pub enum SparklineTimePeriod {
    #[serde(rename = "ONE_DAY")]
    OneDay,
    #[serde(rename = "SEVEN_DAYS")]
    SevenDays,
}

/**
export interface HistoricalFundingResponse {
  historicalFunding: HistoricalFundingResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingResponse {
    pub historical_funding: Vec<HistoricalFundingResponseObject>,
}

/**
export interface HistoricalFundingResponseObject {
  ticker: string,
  rate: string,
  price: string,
  effectiveAt: IsoString,
  effectiveAtHeight: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingResponseObject {
    pub ticker: String,
    pub rate: String,
    pub price: String,
    pub effective_at: String,
    pub effective_at_height: String,
}

/**
export interface AddressRequest {
  address: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressRequest {
    pub address: String,
}

/**
export interface SubaccountRequest extends AddressRequest {
  subaccountNumber: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountRequest {
    pub address: String,
    pub subaccount_number: i32,
}

/**
export interface ParentSubaccountRequest extends AddressRequest {
  parentSubaccountNumber: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
}

/**
export interface PaginationRequest {
  page?: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<f64>,
}

/**
export interface LimitRequest {
  limit: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct LimitRequest {
    pub limit: f64,
}

/**
export interface TickerRequest {
  ticker?: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct TickerRequest {
    pub ticker: Option<String>,
}

/**
interface CreatedBeforeRequest {
  createdBeforeOrAtHeight?: number,
  createdBeforeOrAt?: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBeforeRequest {
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
}

/**
export interface LimitAndCreatedBeforeRequest extends LimitRequest, CreatedBeforeRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitAndCreatedBeforeRequest {
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
}

/**
export interface LimitAndEffectiveBeforeRequest extends LimitRequest {
  effectiveBeforeOrAtHeight?: number,
  effectiveBeforeOrAt?: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitAndEffectiveBeforeRequest {
    pub limit: f64,
    pub effective_before_or_at_height: Option<f64>,
    pub effective_before_or_at: Option<String>,
}

/**
export interface LimitAndCreatedBeforeAndAfterRequest extends LimitAndCreatedBeforeRequest {
  createdOnOrAfterHeight?: number,
  createdOnOrAfter?: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitAndCreatedBeforeAndAfterRequest {
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub created_on_or_after_height: Option<f64>,
    pub created_on_or_after: Option<String>,
}

/**
export interface PerpetualPositionRequest extends SubaccountRequest, LimitAndCreatedBeforeRequest {
  status: PerpetualPositionStatus[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualPositionRequest {
    pub address: String,
    pub subaccount_number: i32,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub status: Vec<PerpetualPositionStatus>,
}

/**
export interface ParentSubaccountPerpetualPositionRequest extends ParentSubaccountRequest,
  LimitAndCreatedBeforeRequest {
  status: PerpetualPositionStatus[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountPerpetualPositionRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub status: Vec<PerpetualPositionStatus>,
}

/**
export interface AssetPositionRequest extends SubaccountRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionRequest {
    pub address: String,
    pub subaccount_number: i32,
}

/**
export interface ParentSubaccountAssetPositionRequest extends ParentSubaccountRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountAssetPositionRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
}

/**
export interface TransferRequest
  extends SubaccountRequest, LimitAndCreatedBeforeRequest, PaginationRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub address: String,
    pub subaccount_number: i32,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub page: Option<f64>,
}

/**
export interface ParentSubaccountTransferRequest
  extends ParentSubaccountRequest, LimitAndCreatedBeforeRequest, PaginationRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountTransferRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub page: Option<f64>,
}

/**
export interface TransferBetweenRequest extends CreatedBeforeRequest {
  sourceAddress: string,
  sourceSubaccountNumber: number,
  recipientAddress: string,
  recipientSubaccountNumber: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenRequest {
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub source_address: String,
    pub source_subaccount_number: f64,
    pub recipient_address: String,
    pub recipient_subaccount_number: f64,
}

// ... (previous conversions)

/**
export interface FillRequest
  extends SubaccountRequest, LimitAndCreatedBeforeRequest, PaginationRequest {
  market: string,
  marketType: MarketType,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillRequest {
    pub address: String,
    pub subaccount_number: i32,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub page: Option<f64>,
    pub market: String,
    pub market_type: MarketType,
}

/**
export interface ParentSubaccountFillRequest
  extends ParentSubaccountRequest, LimitAndCreatedBeforeRequest, PaginationRequest {
  market: string,
  marketType: MarketType,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountFillRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub page: Option<f64>,
    pub market: String,
    pub market_type: MarketType,
}

/**
export interface TradeRequest extends LimitAndCreatedBeforeRequest, PaginationRequest {
  ticker: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeRequest {
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub page: Option<f64>,
    pub ticker: String,
}

/**
export interface PerpetualMarketRequest extends LimitRequest, TickerRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualMarketRequest {
    pub limit: f64,
    pub ticker: Option<String>,
}

/**
export interface PnlTicksRequest
  extends SubaccountRequest, LimitAndCreatedBeforeAndAfterRequest, PaginationRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnlTicksRequest {
    pub address: String,
    pub subaccount_number: i32,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub created_on_or_after_height: Option<f64>,
    pub created_on_or_after: Option<String>,
    pub page: Option<f64>,
}

/**
export interface ParentSubaccountPnlTicksRequest
  extends ParentSubaccountRequest, LimitAndCreatedBeforeAndAfterRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountPnlTicksRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
    pub limit: f64,
    pub created_before_or_at_height: Option<f64>,
    pub created_before_or_at: Option<String>,
    pub created_on_or_after_height: Option<f64>,
    pub created_on_or_after: Option<String>,
}

/**
export interface OrderbookRequest {
  ticker: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookRequest {
    pub ticker: String,
}

/**
export interface GetOrderRequest {
  orderId: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    pub order_id: String,
}

/**
export interface ListOrderRequest extends SubaccountRequest, LimitRequest, TickerRequest {
  side?: OrderSide,
  type?: OrderType,
  status?: OrderStatus[],
  goodTilBlockBeforeOrAt?: number,
  goodTilBlockTimeBeforeOrAt?: IsoString,
  returnLatestOrders?: boolean,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrderRequest {
    pub address: String,
    pub subaccount_number: i32,
    pub limit: f64,
    pub ticker: Option<String>,
    pub side: Option<OrderSide>,
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
    pub status: Option<Vec<APIOrderStatus>>,
    pub good_til_block_before_or_at: Option<f64>,
    pub good_til_block_time_before_or_at: Option<String>,
    pub return_latest_orders: Option<bool>,
}

/**
export interface ParentSubaccountListOrderRequest
  extends ParentSubaccountRequest, LimitRequest, TickerRequest {
  side?: OrderSide,
  type?: OrderType,
  status?: OrderStatus[],
  goodTilBlockBeforeOrAt?: number,
  goodTilBlockTimeBeforeOrAt?: IsoString,
  returnLatestOrders?: boolean,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubaccountListOrderRequest {
    pub address: String,
    pub parent_subaccount_number: f64,
    pub limit: f64,
    pub ticker: Option<String>,
    pub side: Option<OrderSide>,
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
    pub status: Option<Vec<APIOrderStatus>>,
    pub good_til_block_before_or_at: Option<f64>,
    pub good_til_block_time_before_or_at: Option<String>,
    pub return_latest_orders: Option<bool>,
}

/**
export interface CandleRequest extends LimitRequest {
  ticker: string,
  resolution: CandleResolution,
  fromISO?: IsoString,
  toISO?: IsoString,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleRequest {
    pub limit: f64,
    pub ticker: String,
    pub resolution: CandleResolution,
    pub from_iso: Option<String>,
    pub to_iso: Option<String>,
}

/**
export interface SparklinesRequest {
  timePeriod: SparklineTimePeriod,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SparklinesRequest {
    pub time_period: SparklineTimePeriod,
}

/**
export interface HistoricalFundingRequest extends LimitAndEffectiveBeforeRequest {
  ticker: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalFundingRequest {
    pub limit: f64,
    pub effective_before_or_at_height: Option<f64>,
    pub effective_before_or_at: Option<String>,
    pub ticker: String,
}

/**
export interface RegisterTokenRequest {
  address: string,
  token: string,
  language: string,
  message: string,
  timestamp: number,
  signedMessage: string,
  pubKey: string,
  walletIsKeplr: boolean,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTokenRequest {
    pub address: String,
    pub token: String,
    pub language: String,
    pub message: String,
    pub timestamp: f64,
    pub signed_message: String,
    pub pub_key: String,
    pub wallet_is_keplr: bool,
}

/**
export interface Risk {
  initial: Big,
  maintenance: Big,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct Risk {
    pub initial: String,  // Using String as Big.js doesn't have a direct Rust equivalent
    pub maintenance: String,
}

/**
export interface ComplianceResponse {
  restricted: boolean,
  reason?: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceResponse {
    pub restricted: bool,
    pub reason: Option<String>,
}

/**
export interface ComplianceRequest extends AddressRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceRequest {
    pub address: String,
}

/**
export interface SetComplianceStatusRequest extends AddressRequest {
  status: ComplianceStatus,
  reason?: ComplianceReason,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct SetComplianceStatusRequest {
    pub address: String,
    pub status: ComplianceStatus,
    pub reason: Option<ComplianceReason>,
}

/**
export enum BlockedCode {
  GEOBLOCKED = 'GEOBLOCKED',
  COMPLIANCE_BLOCKED = 'COMPLIANCE_BLOCKED',
}
 **/
#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
pub enum BlockedCode {
    #[serde(rename = "GEOBLOCKED")]
    Geoblocked,
    #[serde(rename = "COMPLIANCE_BLOCKED")]
    ComplianceBlocked,
}

/**
export interface ComplianceV2Response {
  status: ComplianceStatus,
  reason?: ComplianceReason,
  updatedAt?: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceV2Response {
    pub status: ComplianceStatus,
    pub reason: Option<ComplianceReason>,
    pub updated_at: Option<String>,
}

/**
export interface HistoricalTradingRewardAggregationRequest extends AddressRequest, LimitRequest {
  period: TradingRewardAggregationPeriod,
  startingBeforeOrAt: IsoString,
  startingBeforeOrAtHeight: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradingRewardAggregationRequest {
    pub address: String,
    pub limit: f64,
    pub period: TradingRewardAggregationPeriod,
    pub starting_before_or_at: String,
    pub starting_before_or_at_height: String,
}

/**
export interface HistoricalTradingRewardAggregationsResponse {
  rewards: HistoricalTradingRewardAggregation[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTradingRewardAggregationsResponse {
    pub rewards: Vec<HistoricalTradingRewardAggregation>,
}

/**
export interface HistoricalTradingRewardAggregation {
  tradingReward: string,
  startedAt: IsoString,
  startedAtHeight: string,
  endedAt?: IsoString,
  endedAtHeight?: string,
  period: TradingRewardAggregationPeriod,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTradingRewardAggregation {
    pub trading_reward: String,
    pub started_at: String,
    pub started_at_height: String,
    pub ended_at: Option<String>,
    pub ended_at_height: Option<String>,
    pub period: TradingRewardAggregationPeriod,
}

/**
export interface HistoricalBlockTradingRewardRequest extends AddressRequest, LimitRequest {
  startingBeforeOrAt: IsoString,
  startingBeforeOrAtHeight: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBlockTradingRewardRequest {
    pub address: String,
    pub limit: f64,
    pub starting_before_or_at: String,
    pub starting_before_or_at_height: String,
}

/**
export interface HistoricalBlockTradingRewardsResponse {
  rewards: HistoricalBlockTradingReward[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBlockTradingRewardsResponse {
    pub rewards: Vec<HistoricalBlockTradingReward>,
}

/**
export interface HistoricalBlockTradingReward {
  tradingReward: string,
  createdAt: IsoString,
  createdAtHeight: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBlockTradingReward {
    pub trading_reward: String,
    pub created_at: String,
    pub created_at_height: String,
}

/**
export interface TraderSearchResponse {
  result?: TraderSearchResponseObject,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct TraderSearchResponse {
    pub result: Option<TraderSearchResponseObject>,
}

/**
export interface TraderSearchRequest {
  searchParam: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraderSearchRequest {
    pub search_param: String,
}

/**
export interface TraderSearchResponseObject {
  address: string,
  subaccountNumber: number,
  subaccountId: string,
  username: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraderSearchResponseObject {
    pub address: String,
    pub subaccount_number: i32,
    pub subaccount_id: String,
    pub username: String,
}

/**
export interface VaultHistoricalPnl {
  ticker: string,
  historicalPnl: PnlTicksResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultHistoricalPnl {
    pub ticker: String,
    pub historical_pnl: Vec<PnlTicksResponseObject>,
}

/**
export interface MegavaultHistoricalPnlResponse {
  megavaultPnl: PnlTicksResponseObject[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MegavaultHistoricalPnlResponse {
    pub megavault_pnl: Vec<PnlTicksResponseObject>,
}

/**
export interface VaultsHistoricalPnlResponse {
  vaultsPnl: VaultHistoricalPnl[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultsHistoricalPnlResponse {
    pub vaults_pnl: Vec<VaultHistoricalPnl>,
}

/**
export interface VaultPosition {
  ticker: string,
  assetPosition: AssetPositionResponseObject,
  perpetualPosition?: PerpetualPositionResponseObject,
  equity: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultPosition {
    pub ticker: String,
    pub asset_position: AssetPositionResponseObject,
    pub perpetual_position: Option<PerpetualPositionResponseObject>,
    pub equity: Decimal,
}

/**
export interface MegavaultPositionResponse {
  positions: VaultPosition[],
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct MegavaultPositionResponse {
    pub positions: Vec<VaultPosition>,
}

/**
export interface MegavaultHistoricalPnlRequest {
  resolution: PnlTickInterval,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct MegavaultHistoricalPnlRequest {
    pub resolution: PnlTickInterval,
}

/**
export interface VaultsHistoricalPnlRequest extends MegavaultHistoricalPnlRequest {}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultsHistoricalPnlRequest {
    pub resolution: PnlTickInterval,
}

/**
export interface AffiliateMetadataRequest {
  address: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateMetadataRequest {
    pub address: String,
}

/**
export interface AffiliateAddressRequest {
  referralCode: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateAddressRequest {
    pub referral_code: String,
}

/**
export interface AffiliateSnapshotRequest {
  addressFilter?: string[],
  limit?: number,
  offset?: number,
  sortByAffiliateEarning?: boolean,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateSnapshotRequest {
    pub address_filter: Option<Vec<String>>,
    pub limit: Option<f64>,
    pub offset: Option<f64>,
    pub sort_by_affiliate_earning: Option<bool>,
}

/**
export interface AffiliateTotalVolumeRequest {
  address: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateTotalVolumeRequest {
    pub address: String,
}

/**
export interface AffiliateMetadataResponse {
  referralCode: string,
  isVolumeEligible: boolean,
  isAffiliate: boolean,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateMetadataResponse {
    pub referral_code: String,
    pub is_volume_eligible: bool,
    pub is_affiliate: bool,
}

/**
export interface AffiliateAddressResponse {
  address: string,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateAddressResponse {
    pub address: String,
}

/**
export interface AffiliateSnapshotResponse {
  affiliateList: AffiliateSnapshotResponseObject[],
  total: number,
  currentOffset: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateSnapshotResponse {
    pub affiliate_list: Vec<AffiliateSnapshotResponseObject>,
    pub total: f64,
    pub current_offset: f64,
}

/**
export interface AffiliateSnapshotResponseObject {
  affiliateAddress: string,
  affiliateReferralCode: string,
  affiliateEarnings: number,
  affiliateReferredTrades: number,
  affiliateTotalReferredFees: number,
  affiliateReferredUsers: number,
  affiliateReferredNetProtocolEarnings: number,
  affiliateReferredTotalVolume: number,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateSnapshotResponseObject {
    pub affiliate_address: String,
    pub affiliate_referral_code: String,
    pub affiliate_earnings: f64,
    pub affiliate_referred_trades: f64,
    pub affiliate_total_referred_fees: f64,
    pub affiliate_referred_users: f64,
    pub affiliate_referred_net_protocol_earnings: f64,
    pub affiliate_referred_total_volume: f64,
}

/**
export interface AffiliateTotalVolumeResponse {
  totalVolume: number | null,
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateTotalVolumeResponse {
    pub total_volume: Option<f64>,
}

/**
export interface SubaccountFromDatabase extends IdBasedModelFromDatabase {
  address: string,
  subaccountNumber: number,
  updatedAt: IsoString,
  updatedAtHeight: string,
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountFromDatabase {
    pub id: String,
    pub address: String,
    pub subaccount_number: i32,
    pub updated_at: String,
    pub updated_at_height: String,
}

/**
export interface PerpetualPositionFromDatabase extends IdBasedModelFromDatabase {
  subaccountId: string,
  perpetualId: string,
  side: PositionSide,
  status: PerpetualPositionStatus,
  size: string,
  maxSize: string,
  entryPrice: string,
  exitPrice?: string,
  sumOpen: string,
  sumClose: string,
  createdAt: IsoString,
  closedAt?: IsoString,
  createdAtHeight: string,
  closedAtHeight?: string,
  openEventId: Buffer,
  closeEventId?: Buffer,
  lastEventId: Buffer,
  settledFunding: string,
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetualPositionFromDatabase {
    pub id: String,
    pub subaccount_id: String,
    pub perpetual_id: String,
    pub side: PositionSide,
    pub status: PerpetualPositionStatus,
    pub size: String,
    pub max_size: String,
    pub entry_price: String,
    pub exit_price: Option<String>,
    pub sum_open: String,
    pub sum_close: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub created_at_height: String,
    pub closed_at_height: Option<String>,
    pub open_event_id: Vec<u8>,
    pub close_event_id: Option<Vec<u8>>,
    pub last_event_id: Vec<u8>,
    pub settled_funding: String,
}

/**
export enum Liquidity {
  TAKER = 'TAKER',
  MAKER = 'MAKER',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum Liquidity {
    TAKER,
    MAKER,
}

/**
export enum FillType {
  LIMIT = 'LIMIT',
  LIQUIDATED = 'LIQUIDATED',
  LIQUIDATION = 'LIQUIDATION',
  DELEVERAGED = 'DELEVERAGED',
  OFFSETTING = 'OFFSETTING',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum FillType {
    LIMIT,
    LIQUIDATED,
    LIQUIDATION,
    DELEVERAGED,
    OFFSETTING,
}

/**
export enum TransferType {
  TRANSFER_IN = 'TRANSFER_IN',
  TRANSFER_OUT = 'TRANSFER_OUT',
  DEPOSIT = 'DEPOSIT',
  WITHDRAWAL = 'WITHDRAWAL',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum TransferType {
    TRANSFER_IN,
    TRANSFER_OUT,
    DEPOSIT,
    WITHDRAWAL,
}

/**
export enum TradeType {
  LIMIT = 'LIMIT',
  LIQUIDATED = 'LIQUIDATED',
  DELEVERAGED = 'DELEVERAGED',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum TradeType {
    LIMIT,
    LIQUIDATED,
    DELEVERAGED,
}

/**
export interface AssetFromDatabase {
  id: string,
  symbol: string,
  atomicResolution: number,
  hasMarket: boolean,
  marketId?: number,
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetFromDatabase {
    pub id: String,
    pub symbol: String,
    pub atomic_resolution: i32,
    pub has_market: bool,
    pub market_id: Option<i32>,
}

/**
export enum PerpetualMarketStatus {
  ACTIVE = 'ACTIVE',
  PAUSED = 'PAUSED',
  CANCEL_ONLY = 'CANCEL_ONLY',
  POST_ONLY = 'POST_ONLY',
  INITIALIZING = 'INITIALIZING',
  FINAL_SETTLEMENT = 'FINAL_SETTLEMENT',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum PerpetualMarketStatus {
    ACTIVE,
    PAUSED,
    CANCEL_ONLY,
    POST_ONLY,
    INITIALIZING,
    FINAL_SETTLEMENT,
}

/**
export enum PerpetualMarketType {
  CROSS = 'CROSS',
  ISOLATED = 'ISOLATED',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, PartialEq, Eq, Hash)]
pub enum PerpetualMarketType {
    CROSS,
    ISOLATED,
}

/**
// Note: The exact structure of RedisOrder is not provided in the postgres_types files.
// This is a placeholder and may need to be updated with the correct fields.
 **/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisOrder {
    // Fields to be added based on actual definition
}

/**
export interface OrderFromDatabase extends IdBasedModelFromDatabase {
  subaccountId: string,
  clientId: string,
  clobPairId: string,
  side: OrderSide,
  size: string,
  totalFilled: string,
  price: string,
  type: OrderType,
  status: OrderStatus,
  timeInForce: TimeInForce,
  reduceOnly: boolean,
  orderFlags: string,
  updatedAt: IsoString,
  updatedAtHeight: string,
  goodTilBlock?: string,
  goodTilBlockTime?: string,
  createdAtHeight?: string,
  clientMetadata: string,
  triggerPrice?: string,
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFromDatabase {
    pub id: String,
    pub subaccount_id: String,
    pub client_id: String,
    pub clob_pair_id: String,
    pub side: OrderSide,
    pub size: String,
    pub total_filled: String,
    pub price: String,
    #[serde(rename = "type")]
    pub type_field: OrderType,
    pub status: APIOrderStatus,
    pub time_in_force: APITimeInForce,
    pub reduce_only: bool,
    pub order_flags: String,
    pub updated_at: String,
    pub updated_at_height: String,
    pub good_til_block: Option<String>,
    pub good_til_block_time: Option<String>,
    pub created_at_height: Option<String>,
    pub client_metadata: String,
    pub trigger_price: Option<String>,
}

/**
export enum CandleResolution {
  ONE_DAY = '1DAY',
  FOUR_HOURS = '4HOURS',
  ONE_HOUR = '1HOUR',
  THIRTY_MINS = '30MINS',
  FIFTEEN_MINS = '15MINS',
  FIVE_MINS = '5MINS',
  ONE_MIN = '1MIN',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum CandleResolution {
    #[serde(rename = "1DAY")]
    OneDay,
    #[serde(rename = "4HOURS")]
    FourHours,
    #[serde(rename = "1HOUR")]
    OneHour,
    #[serde(rename = "30MINS")]
    ThirtyMins,
    #[serde(rename = "15MINS")]
    FifteenMins,
    #[serde(rename = "5MINS")]
    FiveMins,
    #[serde(rename = "1MIN")]
    OneMin,
}


/**
export enum ComplianceStatus {
  COMPLIANT = 'COMPLIANT',
  FIRST_STRIKE_CLOSE_ONLY = 'FIRST_STRIKE_CLOSE_ONLY',
  FIRST_STRIKE = 'FIRST_STRIKE',
  CLOSE_ONLY = 'CLOSE_ONLY',
  BLOCKED = 'BLOCKED',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum ComplianceStatus {
    COMPLIANT,
    FIRST_STRIKE_CLOSE_ONLY,
    FIRST_STRIKE,
    CLOSE_ONLY,
    BLOCKED,
}

/**
export enum ComplianceReason {
  MANUAL = 'MANUAL',
  US_GEO = 'US_GEO',
  CA_GEO = 'CA_GEO',
  GB_GEO = 'GB_GEO',
  SANCTIONED_GEO = 'SANCTIONED_GEO',
  COMPLIANCE_PROVIDER = 'COMPLIANCE_PROVIDER',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum ComplianceReason {
    MANUAL,
    US_GEO,
    CA_GEO,
    GB_GEO,
    SANCTIONED_GEO,
    COMPLIANCE_PROVIDER,
}

/**
export enum TradingRewardAggregationPeriod {
  DAILY = 'DAILY',
  WEEKLY = 'WEEKLY',
  MONTHLY = 'MONTHLY',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum TradingRewardAggregationPeriod {
    DAILY,
    WEEKLY,
    MONTHLY,
}

/**
export enum PnlTickInterval {
  hour = 'hour',
  day = 'day',
}
 **/
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum PnlTickInterval {
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
}
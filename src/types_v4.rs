use std::any::Any;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

pub type OrdersResponse = Vec<OrderResponseObject>;

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
    pub size: String,
    pub price: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    pub market: String,
    pub client_id: String,
    pub good_til_block_time: Option<i64>,
    pub good_til_block: Option<i64>,
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
    pub size: String,
    pub total_filled: Option<String>,
    pub price: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub order_flags: String,
    pub good_til_block: Option<String>,
    pub good_til_block_time: Option<String>,
    pub created_at_height: Option<String>,
    pub client_metadata: String,
    pub trigger_price: Option<String>,
    pub time_in_force: APITimeInForce,
    pub status: APIOrderStatus,
    pub post_only: bool,
    pub ticker: String,
    pub updated_at: Option<String>,
    pub updated_at_height: Option<String>,
    pub subaccount_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TrailingStop,
    TakeProfit,
    TakeProfitMarket,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum APITimeInForce {
    GTT,
    FOK,
    IOC,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq)]
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
    pub fn get_open_statuses() -> Vec<Self> {
        vec![Self::BestEffortOpened, Self::Open, Self::Untriggered]
    }

    pub fn get_closed_statuses() -> Vec<Self> {
        vec![Self::Filled, Self::Canceled, Self::BestEffortCanceled]
    }

    pub fn get_step_number(&self) -> usize {
        match self {
            Self::Untriggered => 0,
            Self::BestEffortOpened => 1,
            Self::Open => 1,
            Self::Filled => 2,
            Self::Canceled => 2,
            Self::BestEffortCanceled => 2,
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
    pub equity: String,
    pub free_collateral: String,
    pub open_perpetual_positions: PerpetualPositionsMap,
    pub asset_positions: AssetPositionsMap,
    pub margin_enabled: bool,
    pub updated_at_height: String,
    pub latest_processed_block_height: String,
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
            equity: String::default(),
            free_collateral: String::default(),
            open_perpetual_positions: PerpetualPositionsMap::default(),
            asset_positions: AssetPositionsMap::default(),
            margin_enabled: false,
            updated_at_height: String::default(),
            latest_processed_block_height: String::default(),
        }
    }
}

impl SubaccountResponseInnerObject {
    pub fn get_quote_balance(&self) -> String {
        match self.asset_positions.get("USDC") {
            Some(position) => position.size.clone(),
            None => "0.0".to_string(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountWebSocketObject {
    pub address: String,
    pub subaccount_number: i32,
    pub equity: String,
    pub free_collateral: String,
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
    pub size: String,
    pub max_size: String,
    pub entry_price: String,
    pub realized_pnl: String,
    pub created_at: String,
    pub created_at_height: String,
    pub sum_open: String,
    pub sum_close: String,
    pub net_funding: String,
    pub unrealized_pnl: String,
    pub closed_at: Option<String>,
    pub exit_price: Option<String>,
    pub subaccount_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum PerpetualPositionStatus {
    OPEN,
    CLOSED,
    LIQUIDATED,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, Eq, PartialEq)]
pub enum PositionSide {
    LONG,
    SHORT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPositionResponseObject {
    pub symbol: String,
    pub side: PositionSide,
    pub size: String,
    pub asset_id: String,
    pub subaccount_number: i32,
}

/**
export interface ResponseWithBody extends express.Response {
  body: unknown,
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
    pub equity: String,
    pub free_collateral: String,
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
    pub price: String,
    pub size: String,
    pub fee: String,
    pub affiliate_rev_share: Option<String>,
    pub created_at: String,
    pub created_at_height: String,
    pub order_id: Option<String>,
    pub client_metadata: Option<String>,
    pub subaccount_number: i32,
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
    pub equity: String,
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
#[derive(Debug, Serialize, Deserialize, Display, EnumString, Clone)]
pub enum MarketType {
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "SPOT")]
    Spot,
}

/**
export interface PerpetualMarketResponse {
  markets: {
    [ticker: string]: PerpetualMarketResponseObject,
  },
}
 **/
#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualMarketResponse {
    pub markets: HashMap<String, PerpetualMarketResponseObject>,
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpetualMarketResponseObject {
    pub clob_pair_id: String,
    pub ticker: String,
    pub status: PerpetualMarketStatus,
    pub oracle_price: String,
    pub price_change24H: String,
    pub volume24H: String,
    pub trades24H: i64,
    pub next_funding_rate: String,
    pub initial_margin_fraction: String,
    pub maintenance_margin_fraction: String,
    pub open_interest: String,
    pub atomic_resolution: i64,
    pub quantum_conversion_exponent: i64,
    pub tick_size: String,
    pub step_size: String,
    pub step_base_quantums: i64,
    pub subticks_per_tick: i64,
    pub market_type: PerpetualMarketType,
    pub open_interest_lower_cap: Option<String>,
    pub open_interest_upper_cap: Option<String>,
    pub base_open_interest: String,
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
    pub low: String,
    pub high: String,
    pub open: String,
    pub close: String,
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
    pub equity: String,
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
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
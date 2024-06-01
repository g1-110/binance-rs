use crate::model::{string_or_float, string_or_float_opt};
use serde::{Deserialize, Serialize};

fn default_float() -> Option<f64> {
    Some(0.0)
}

fn default_string() -> Option<String> {
    Some("".to_string())
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub avg_price: Option<f64>,
    #[serde(alias = "clientOrderId", alias = "newClientStrategyId")]
    pub client_order_id: String,
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub cum_base: Option<f64>,
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub executed_qty: Option<f64>,
    #[serde(alias = "orderId", alias = "strategyId")]
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(default = "default_string")]
    pub orig_type: Option<String>, // maybe change this to order type enum
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: String,          // maybe change this to order side enum
    pub position_side: String, // maybe change this to position side enum
    #[serde(alias = "strategyStatus")]
    pub status: String, // maybe change this to order status enum
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub stop_price: Option<f64>,
    pub symbol: String,
    #[serde(alias = "bookTime", default)]
    pub time: u64,
    pub time_in_force: String, // maybe change this to time in force enum
    #[serde(alias = "type", alias = "strategyType")]
    pub order_type: String,
    pub update_time: u64,
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub activate_price: Option<f64>,
    #[serde(with = "string_or_float_opt", default = "default_float")]
    pub price_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionRisk {
    pub symbol: String,
    #[serde(default = "default_string")]
    pub side: Option<String>,
    pub position_side: String,
    pub leverage: String,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float", rename = "notionalValue")]
    pub notional: f64,
    #[serde(with = "string_or_float", rename = "maxQty")]
    pub max_quantity: f64,
    #[serde(with = "string_or_float")]
    pub break_even_price: f64,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,

    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    #[serde(with = "string_or_float", rename = "uniMMR")]
    pub uni_mmr: f64,
    #[serde(with = "string_or_float")]
    pub account_equity: f64,
    #[serde(with = "string_or_float")]
    pub actual_equity: f64,
    #[serde(with = "string_or_float")]
    pub account_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub account_maint_margin: f64,
    pub account_status: String,
    #[serde(with = "string_or_float")]
    pub virtual_max_withdraw_amount: f64,
    #[serde(with = "string_or_float")]
    pub total_available_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_margin_open_loss: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub total_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_asset: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_borrowed: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_free: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_interest: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_locked: f64,
    #[serde(with = "string_or_float")]
    pub um_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "umUnrealizedPNL")]
    pub um_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub cm_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "cmUnrealizedPNL")]
    pub cm_unrealized_pnl: f64,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub negative_balance: f64,
}

use crate::account::OrderSide;
use crate::api::{PortfolioMargin, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::Empty;
use crate::util::build_signed_request;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Display;

use super::model::{AccountBalance, AccountInformation, Order};
// use crate::binance::portfolio_margin::model::{AccountBalance, AccountInformation, Order};

use super::model::PositionRisk;

#[derive(Clone)]
pub struct PortfolioMarginAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl Display for PositionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Both => write!(f, "BOTH"),
            Self::Long => write!(f, "LONG"),
            Self::Short => write!(f, "SHORT"),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::Stop => write!(f, "STOP"),
            Self::StopMarket => write!(f, "STOP_MARKET"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            Self::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl Display for WorkingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MarkPrice => write!(f, "MARK_PRICE"),
            Self::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[allow(clippy::all)]
#[derive(Deserialize, Clone, Debug)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
            Self::GTX => write!(f, "GTX"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ResponseType {
    Ack,
    Result,
}
impl Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ack => write!(f, "ACK"),
            Self::Result => write!(f, "RESULT"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SelfTradePreventionMode {
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}
impl Display for SelfTradePreventionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpireTaker => write!(f, "EXPIRE_TAKER"),
            Self::ExpireMaker => write!(f, "EXPIRE_MAKER"),
            Self::ExpireBoth => write!(f, "EXPIRE_BOTH"),
        }
    }
}
#[derive(Clone, Debug)]
pub struct OrderRequest {
    pub new_client_order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub qty: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<f64>,
    pub response_type: Option<ResponseType>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}
impl OrderRequest {
    pub fn with_defaults(
        new_client_order_id: String, symbol: String, side: OrderSide, order_type: OrderType,
    ) -> Self {
        Self {
            new_client_order_id,
            symbol,
            side,
            order_type,
            position_side: None,
            time_in_force: None,
            qty: None,
            reduce_only: None,
            price: None,
            stop_price: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            response_type: None,
            self_trade_prevention_mode: None,
        }
    }
}

impl PortfolioMarginAccount {
    pub fn cancel_order_with_client_id<S>(
        &self, symbol: S, order: Order, market: S,
    ) -> Result<Order>
    where
        S: Into<String>,
    {
        let endpoint;
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        if ["LIMIT", "MARKET"].contains(&order.order_type.as_str()) {
            parameters.insert("origClientOrderId".into(), order.client_order_id);
            if market.into() == "inverse".to_string() {
                endpoint = API::PortfolioMargin(PortfolioMargin::OrderCM);
            } else {
                endpoint = API::PortfolioMargin(PortfolioMargin::OrderUM);
            }
        } else {
            parameters.insert("newClientStrategyId".into(), order.client_order_id);
            if market.into() == "inverse".to_string() {
                endpoint = API::PortfolioMargin(PortfolioMargin::ConditionalOrderCM);
            } else {
                endpoint = API::PortfolioMargin(PortfolioMargin::ConditionalOrderUM);
            }
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed(endpoint, Some(request))
    }

    // Custom order for for professional traders
    pub fn post_order(&self, order_request: OrderRequest, market: String) -> Result<Order> {
        let order_params = self.build_order(order_request.clone());
        println!("{:?}", order_params);
        let request = build_signed_request(order_params, self.recv_window)?;

        if market == "inverse" {
            if [OrderType::Limit, OrderType::Market].contains(&order_request.order_type) {
                self.client
                    .post_signed(API::PortfolioMargin(PortfolioMargin::OrderCM), request)
            } else {
                self.client.post_signed(
                    API::PortfolioMargin(PortfolioMargin::ConditionalOrderCM),
                    request,
                )
            }
        } else {
            if [OrderType::Limit, OrderType::Market].contains(&order_request.order_type) {
                self.client
                    .post_signed(API::PortfolioMargin(PortfolioMargin::OrderUM), request)
            } else {
                self.client.post_signed(
                    API::PortfolioMargin(PortfolioMargin::ConditionalOrderUM),
                    request,
                )
            }
        }
    }

    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order.symbol);
        parameters.insert("side".into(), order.side.to_string());

        // parameters.insert("type".into(), order.order_type.to_string());

        if [OrderType::Limit, OrderType::Market].contains(&order.order_type) {
            parameters.insert("type".into(), order.order_type.to_string());
            parameters.insert("newClientOrderId".into(), order.new_client_order_id);
        } else {
            parameters.insert("strategyType".into(), order.order_type.to_string());
            parameters.insert("newClientStrategyId".into(), order.new_client_order_id);
        }

        if let Some(position_side) = order.position_side {
            parameters.insert("positionSide".into(), position_side.to_string());
        }
        if let Some(time_in_force) = order.time_in_force {
            parameters.insert("timeInForce".into(), time_in_force.to_string());
        }
        if let Some(qty) = order.qty {
            parameters.insert("quantity".into(), qty.to_string());
        }
        if let Some(reduce_only) = order.reduce_only {
            parameters.insert("reduceOnly".into(), reduce_only.to_string().to_uppercase());
        }
        if let Some(price) = order.price {
            parameters.insert("price".into(), price.to_string());
        }
        if let Some(stop_price) = order.stop_price {
            parameters.insert("stopPrice".into(), stop_price.to_string());
        }
        if let Some(activation_price) = order.activation_price {
            parameters.insert("activationPrice".into(), activation_price.to_string());
        }
        if let Some(callback_rate) = order.callback_rate {
            parameters.insert("callbackRate".into(), callback_rate.to_string());
        }
        if let Some(working_type) = order.working_type {
            parameters.insert("workingType".into(), working_type.to_string());
        }
        if let Some(price_protect) = order.price_protect {
            parameters.insert(
                "priceProtect".into(),
                price_protect.to_string().to_uppercase(),
            );
        }
        if let Some(response_type) = order.response_type {
            parameters.insert("newOrderRespType".into(), response_type.to_string());
        }
        parameters
    }

    pub fn get_position_information<S>(&self, symbol: S, market: S) -> Result<Vec<PositionRisk>>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        if market.into() == "inverse" {
            self.client.get_signed(
                API::PortfolioMargin(PortfolioMargin::PositionRiskCM),
                Some(request),
            )
        } else {
            self.client.get_signed(
                API::PortfolioMargin(PortfolioMargin::PositionRiskUM),
                Some(request),
            )
        }
    }

    pub fn get_account_information(&self) -> Result<AccountInformation> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(
            API::PortfolioMargin(PortfolioMargin::Account),
            Some(request),
        )
    }

    pub fn get_account_balance(&self) -> Result<Vec<AccountBalance>> {
        let parameters = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(
            API::PortfolioMargin(PortfolioMargin::Balance),
            Some(request),
        )
    }

    // pub fn change_initial_leverage<S>(
    //     &self,
    //     symbol: S,
    //     leverage: u8,
    // ) -> Result<ChangeLeverageResponse>
    // where
    //     S: Into<String>,
    // {
    //     let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    //     parameters.insert("symbol".into(), symbol.into());
    //     parameters.insert("leverage".into(), leverage.to_string());

    //     let request = build_signed_request(parameters, self.recv_window)?;
    //     self.client
    //         .post_signed(API::Futures(Futures::ChangeInitialLeverage), request)
    // }

    pub fn cancel_all_open_orders<S>(&self, symbol: S, market: S) -> Result<()>
    where
        S: Into<String>,
    {
        let orders_canceled;
        let conditional_orders_canceled;
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        if market.into() == "inverse" {
            orders_canceled = self
                .client
                .delete_signed::<Empty>(
                    API::PortfolioMargin(PortfolioMargin::CancelAllOpenOrdersCM),
                    Some(request.clone()),
                )
                .map(|_| ());
            conditional_orders_canceled = self
                .client
                .delete_signed::<Empty>(
                    API::PortfolioMargin(PortfolioMargin::CancelAllConditionalOpenOrdersCM),
                    Some(request.clone()),
                )
                .map(|_| ());
        } else {
            orders_canceled = self
                .client
                .delete_signed::<Empty>(
                    API::PortfolioMargin(PortfolioMargin::CancelAllOpenOrdersUM),
                    Some(request.clone()),
                )
                .map(|_| ());
            conditional_orders_canceled = self
                .client
                .delete_signed::<Empty>(
                    API::PortfolioMargin(PortfolioMargin::CancelAllConditionalOpenOrdersUM),
                    Some(request.clone()),
                )
                .map(|_| ());
        }

        if orders_canceled.is_err() {
            return Err(orders_canceled.err().unwrap());
        } else if conditional_orders_canceled.is_err() {
            return Err(conditional_orders_canceled.err().unwrap());
        } else {
            Ok(())
        }
    }

    pub fn get_all_open_orders<S>(&self, symbol: S, market: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        if market.into() == "inverse" {
            let mut regular_orders: Vec<Order> = self
                .client
                .get_signed(
                    API::PortfolioMargin(PortfolioMargin::OpenOrdersCM),
                    Some(request),
                )
                .unwrap();
            parameters = BTreeMap::new();

            let request = build_signed_request(parameters, self.recv_window)?;

            let mut conditional_orders: Vec<Order> = self
                .client
                .get_signed(
                    API::PortfolioMargin(PortfolioMargin::ConditionalOpenOrdersCM),
                    Some(request),
                )
                .unwrap();

            regular_orders.append(&mut conditional_orders);
            Ok(regular_orders)
        } else {
            let a = self.client.get_signed(
                API::PortfolioMargin(PortfolioMargin::OpenOrdersUM),
                Some(request),
            );
            println!("{:#?}", a);
            a
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub portfolio_margin_rest_api_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_cm_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub recv_window: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com/ws".into(),

            portfolio_margin_rest_api_endpoint: "https://papi.binance.com".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_cm_rest_api_endpoint: "https://dapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com/ws".into(),

            recv_window: 5000,
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        Self::default()
            .set_rest_api_endpoint("https://testnet.binance.vision")
            .set_ws_endpoint("wss://testnet.binance.vision/ws")
            .set_portfolio_margin_rest_api_endpoint("https://testnet.binance.vision")
            .set_futures_rest_api_endpoint("https://testnet.binancefuture.com")
            .set_futures_cm_rest_api_endpoint("https://testnet.binancefuture.com")
            .set_futures_ws_endpoint("https://testnet.binancefuture.com/ws")
    }

    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }
    pub fn set_portfolio_margin_rest_api_endpoint<T: Into<String>>(
        mut self, portfolio_margin_rest_api_endpoint: T,
    ) -> Self {
        self.portfolio_margin_rest_api_endpoint = portfolio_margin_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_rest_api_endpoint<T: Into<String>>(
        mut self, futures_rest_api_endpoint: T,
    ) -> Self {
        self.futures_rest_api_endpoint = futures_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_cm_rest_api_endpoint<T: Into<String>>(
        mut self, futures_cm_rest_api_endpoint: T,
    ) -> Self {
        self.futures_cm_rest_api_endpoint = futures_cm_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_ws_endpoint<T: Into<String>>(mut self, futures_ws_endpoint: T) -> Self {
        self.futures_ws_endpoint = futures_ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}

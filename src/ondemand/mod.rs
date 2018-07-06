use errors::*;
use reqwest::header::{Headers, UserAgent};
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::collections::BTreeMap;

const API_ENDPOINT: &'static str = "https://ondemand.websol.barchart.com";

pub struct Client {
    http_client: HttpClient,
    api_key: String,
    api_endpoint: String,
}

impl Client {
    // pub fn debug(&self, debug:&bool){
    //self.debug = debug;
    // }

    pub fn new(api_key: Option<String>, api_endpoint: Option<String>) -> Client {
        Client {
            http_client: HttpClient::new(),
            api_key: api_key.unwrap_or_else(|| "".into()),
            api_endpoint: api_endpoint.unwrap_or_else(|| API_ENDPOINT.into()),
        }
    }

    pub fn quote(&self, symbols: &str, fields: Option<&str>) -> Result<(Value)> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("symbols".into(), symbols.into());
        params.insert("fields".into(), fields.unwrap_or_else(|| "").to_string());

        self.get("getQuote", &params)
    }

    pub fn equities_by_exchange(&self, exchange: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("exchange".into(), exchange.into());

        self.get("getEquitiesByExchange", &params)
    }

    pub fn futures_by_exchange(&self, exchange: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("exchange".into(), exchange.into());

        self.get("getFuturesByExchange", &params)
    }

    pub fn futures_options(&self, root: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("root".into(), root.into());

        self.get("getFuturesOptions", &params)
    }

    pub fn special_options(&self, root: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("root".into(), root.into());

        self.get("getSpecialOptions", &params)
    }

    pub fn equity_options(
        &self,
        underlying_symbols: &str,
        inputs: Option<&str>,
    ) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("underlying_symbols".into(), underlying_symbols.into());

        self.get("getEquityOptions", &params)
    }

    pub fn equity_options_intraday(
        &self,
        underlying_symbols: &str,
        inputs: Option<&str>,
    ) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("underlying_symbols".into(), underlying_symbols.into());

        self.get("getEquityOptionsIntraday", &params)
    }

    pub fn equity_options_history(&self, symbol: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("symbol".into(), symbol.into());

        self.get("getEquityOptionsHistory", &params)
    }

    pub fn options_screener(&self, inputs: Option<&str>) -> Result<(Value)> {
        let params = self.to_betree(&self.option_to_string(inputs));

        self.get("getOptionsScreener", &params)
    }

    pub fn ameribor_rate(&self) -> Result<(Value)> {
        let params = self.to_betree("");

        self.get("getAmeriborRate", &params)
    }

    pub fn quote_eod(&self, symbols: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("symbols".into(), symbols.into());

        self.get("getQuoteEod", &params)
    }

    pub fn history(&self, symbol: &str, _type: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("symbol".into(), symbol.into());
        params.insert("type".into(), _type.into());

        self.get("getHistroy", &params)
    }

    pub fn close_price(&self, symbols: &str, inputs: Option<&str>) -> Result<(Value)> {
        let mut params = self.to_betree(&self.option_to_string(inputs));
        params.insert("symbols".into(), symbols.into());

        self.get("getClosePrice", &params)
    }

    pub fn get(&self, api_name: &str, params: &BTreeMap<String, String>) -> Result<(Value)> {
        let url = format!(
            "{}/{}.json?apikey={}&{}",
            self.api_endpoint,
            api_name,
            self.api_key,
            self.build_request(params)
        );
        println!("Going after: {}", url);
        let mut resp = self
            .http_client
            .get(url.as_str())
            .headers(self.build_headers())
            .send()?;

        Ok(resp.json()?)
    }

    pub fn option_to_string(&self, o: Option<&str>) -> String {
        o.unwrap_or_else(|| "").to_string()
    }

    pub fn to_betree(&self, params: &str) -> BTreeMap<String, String> {
        let mut result: BTreeMap<String, String> = BTreeMap::new();

        if params.is_empty() == true {
            return result;
        }

        let entry = params.split(",");

        for s in entry {
            let mut kv = s.split("=");
            result.insert(
                kv.next().unwrap().to_string(),
                kv.next().unwrap().to_string(),
            );
        }

        result
    }

    fn build_request(&self, parameters: &BTreeMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{}={}&", key, value);
            request.push_str(param.as_ref());
        }
        request.pop();

        request
    }

    fn build_headers(&self) -> Headers {
        let mut custom_headers = Headers::new();
        custom_headers.set(UserAgent::new("barchart-ondemand-client-rust"));

        custom_headers
    }
}

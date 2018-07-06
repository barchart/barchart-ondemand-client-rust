# A Rust Client for Barchart OnDemand

A Simple Rust client for accessing market data from the Barchart OnDemand APIs.

## Install

Add this to your Cargo.toml file

```
[dependencies]
ondemand = { git = "https://github.com/barchart/barchart-ondemand-client-rust.git" }
```

### Get Market Data for Apple and Exelon

```rust
extern crate ondemand;

use ondemand::OnDemandClient;

let client = OnDemandClient::new(Some("YOUR_API_KEY".into()), None);

let q =  c.quote("AAPL,EXC", None);
println!("Raw Quote Resp: \n{:?}", q);
```

#### Outputs

```rust
Ok(Object({"results": Array([Object({"close": Number(187.97), "dayCode": String("6"), "dollarVolume": Number(2834727397.8281), "flag": String("s"), "high": Number(188.43), "lastPrice": Number(187.97), "low": Number(185.2), "mode": String("i"), "name": String("Apple Inc"), "netChange": Number(2.57), "numTrades": Number(79941), "open": Number(185.42), "percentChange": Number(1.39), "previousVolume": Number(16600199), "serverTimestamp": String("2018-07-06T17:21:10-05:00"), "symbol": String("AAPL"), "tradeTimestamp": String("2018-07-06T16:15:01-05:00"), "unitCode": String("2"), "volume": Number(17444363)})]), "status": Object({"code": Number(200), "message": String("Success.")})}))
```

### Generic call via API name

```rust
let mut params: BTreeMap<String, String> = BTreeMap::new();

params.insert("symbols".into(), "^BTCUSD".into());

let respone = c.get("getCrypto", &params);
let mut result  = respone.ok().unwrap();
let quotes = result["results"].as_array_mut().unwrap();

for(i, quote) in quotes.iter().enumerate(){
    println!("Quote: {} : {}", quote["symbol"], quote["lastPrice"]);
}
```
extern crate ondemand;

use std::collections::BTreeMap;
use ondemand::OnDemandClient;

fn main(){
    let c = OnDemandClient::new(Some("CHANGE_ME".into()), None);

    let q =  c.quote("AAPL", None);
    println!("Raw Quote Resp: \n{:?}", q);
   
    let cp = c.close_price("AAPL", Some("date=2018-01-15,splits=1,dividends=1"));
    println!("Close Price: \n{:?}", cp);

    let h = c.history("AAPL", "minutes", None);
    println!("History: \n{:?}", h);

    let ee = c.quote_eod("AAPL", Some("exchange=NASDAQ"));
    println!("Quote EOD: \n{:?}", ee);

    // let so = c.special_options("BCD", None);
    // println!("so: \n{:?}", so);

    // let eq = c.get_equities_by_exchange("BATS", Some("fields=symbol"));
    // println!("eq: \n{:?}", eq);

    let mut get_quote_params: BTreeMap<String, String> = BTreeMap::new();
    get_quote_params.insert("symbols".into(), "AAPL,EXC".into());

    let gen = c.get("getQuote", &get_quote_params);
    let mut result  = gen.ok().unwrap();
    let quotes = result["results"].as_array_mut().unwrap();
    
    for(_i, elem) in quotes.iter().enumerate(){
        println!("Quote: {} : {}",elem["symbol"], elem["lastPrice"]);
    }

}
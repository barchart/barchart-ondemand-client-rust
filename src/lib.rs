#[macro_use]
extern crate error_chain;

extern crate reqwest;
extern crate serde_json;

pub mod errors;
pub mod ondemand;

pub use ondemand::Client as OnDemandClient;

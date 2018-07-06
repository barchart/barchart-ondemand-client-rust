use std;
use reqwest;
use serde_json;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors { FooError }

    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }
}

use error_chain::error_chain;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CoingeckoContentError {
    pub msg: String,
}

error_chain! {
    errors {
       CoingeckoError(response: CoingeckoContentError)
    }
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        TimestampError(std::time::SystemTimeError);
    }
}

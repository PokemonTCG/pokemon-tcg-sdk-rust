use crate::errors::{ClientError, ErrorEnvelope};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Client {
    pub(super) base_url: String,
    pub(super) http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: Option<&str>) -> Result<Self, Box<dyn Error>> {
        Ok(Client {
            base_url: String::from(r#"https://api.pokemontcg.io/v2"#),
            http_client: Client::get_http_client(api_key)?,
        })
    }

    pub fn with_base_url(base_url: &str, api_key: Option<&str>) -> Result<Self, Box<dyn Error>> {
        Ok(Client {
            base_url: String::from(base_url),
            http_client: Client::get_http_client(api_key)?,
        })
    }

    pub(super) fn get_http_client(
        api_key: Option<&str>,
    ) -> Result<reqwest::Client, Box<dyn Error>> {
        let http_client = reqwest::Client::builder();
        if let Some(key) = api_key {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-Api-Key", reqwest::header::HeaderValue::from_str(key)?);
        }

        http_client.build().map_err(|e| e.into())
    }

    pub(super) fn parse_response<T>(result: ApiResult<T>) -> Result<T, ClientError> {
        match result {
            ApiResult::Ok(v) => Ok(v.data),
            ApiResult::Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataEnvelope<T> {
    pub data: T,
    #[serde(alias = "totalCount")]
    pub total_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(DataEnvelope<T>),
    Err(ErrorEnvelope),
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None).unwrap()
    }
}

use crate::errors::{ClientError, ErrorEnvelope};
use serde::{Deserialize, Serialize};

pub struct Client {
    pub(super) base_url: String,
    pub(super) http_client: reqwest::Client,
}

impl Client {
    /// Constructs a new client
    /// 
    /// # Errors
    /// This method fails if the API key is invalid ("\n" etc.) 
    /// or if a TLS backend cannot be initialized, or the resolver
    /// cannot load the system configuration.
    pub fn new(api_key: Option<&str>) -> Result<Self, ClientError> {
        Ok(Client {
            base_url: String::from(r#"https://api.pokemontcg.io/v2"#),
            http_client: Client::get_http_client(api_key)?,
        })
    }

    /// Constructs a client with a different base url than the default for the API.
    ///
    /// # Errors
    /// This method fails if the API key is invalid ("\n" etc.) 
    /// or if a TLS backend cannot be initialized, or the resolver
    /// cannot load the system configuration.
    pub fn with_base_url(base_url: &str, api_key: Option<&str>) -> Result<Self, ClientError> {
        Ok(Client {
            base_url: String::from(base_url),
            http_client: Client::get_http_client(api_key)?,
        })
    }

    /// Constructs a client with an API key that will be passed on every request.
    /// 
    /// # Errors
    /// This method fails if the API key is invalid ("\n" etc.) 
    /// or if a TLS backend cannot be initialized, or the resolver
    /// cannot load the system configuration.
    pub fn with_api_key(api_key: &str) -> Result<Self, ClientError> {
        Client::new(Some(api_key))
    }

    /// Builds the reqwest http client that will be used for all API requests
    pub(super) fn get_http_client(api_key: Option<&str>) -> Result<reqwest::Client, ClientError> {
        let mut http_client = reqwest::Client::builder();
        if let Some(key) = api_key {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-Api-Key", reqwest::header::HeaderValue::from_str(key)?);
            http_client = http_client.default_headers(headers);
        }

        http_client.build().map_err(|e| e.into())
    }

    /// Parses the response from the API into either a generic data envelope ({data: T}) or 
    /// an error envelope ({error: ...}) and returns T if successful
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
    /// Constructs a basic client with no API Key using the default URL.
    /// 
    /// # Panics
    /// This method will panic if the construction of the reqwest http client fails,
    /// if a TLS backend cannot be initialized, or the resolver
    /// cannot load the system configuration.
    fn default() -> Self {
        Self::new(None).unwrap()
    }
}

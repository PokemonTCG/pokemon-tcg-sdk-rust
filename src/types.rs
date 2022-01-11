use crate::{
    client::{ApiResult, Client},
    errors::ClientError,
};

impl Client {
    /// Get all possible types
    ///
    /// https://docs.pokemontcg.io/api-reference/types/get-types
    pub async fn get_types(&self) -> Result<Vec<String>, ClientError> {
        let types = self
            .http_client
            .get(format!("{}/types", self.base_url))
            .send()
            .await?
            .json::<ApiResult<Vec<String>>>()
            .await?;

        Client::parse_response(types)
    }

    /// Get all possible subtypes
    ///
    /// https://docs.pokemontcg.io/api-reference/subtypes/get-subtypes
    pub async fn get_subtypes(&self) -> Result<Vec<String>, ClientError> {
        let types = self
            .http_client
            .get(format!("{}/subtypes", self.base_url))
            .send()
            .await?
            .json::<ApiResult<Vec<String>>>()
            .await?;

        Client::parse_response(types)
    }

    /// Get all possible supertypes
    ///
    /// https://docs.pokemontcg.io/api-reference/supertypes/get-supertypes
    pub async fn get_supertypes(&self) -> Result<Vec<String>, ClientError> {
        let types = self
            .http_client
            .get(format!("{}/supertypes", self.base_url))
            .send()
            .await?
            .json::<ApiResult<Vec<String>>>()
            .await?;

        Client::parse_response(types)
    }

    /// Get all possible rarities
    ///
    /// https://docs.pokemontcg.io/api-reference/rarities/get-rarities
    pub async fn get_rarities(&self) -> Result<Vec<String>, ClientError> {
        let types = self
            .http_client
            .get(format!("{}/rarities", self.base_url))
            .send()
            .await?
            .json::<ApiResult<Vec<String>>>()
            .await?;

        Client::parse_response(types)
    }
}

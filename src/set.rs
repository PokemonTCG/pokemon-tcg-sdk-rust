mod images;
mod legality;

use serde::{Deserialize, Serialize};

use crate::{
    client::{ApiResult, Client},
    errors::ClientError,
};

use self::{images::SetImages, legality::Legality};

/// The Set Object
/// https://docs.pokemontcg.io/api-reference/sets/set-object
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
pub struct Set {
    /// Unique identifier for the object.
    pub id: String,
    /// The name of the set.
    pub name: String,
    /// The series the set belongs to, like Sword and Shield or Base.
    pub series: String,
    /// The number printed on the card that represents the total. This total does not include secret rares.
    #[serde(alias = "printedTotal")]
    pub printed_total: usize,
    /// The total number of cards in the set, including secret rares, alternate art, etc.
    pub total: usize,
    /// The legalities of the set. If a given format is not legal, it will not appear in the hash.
    pub legalities: Legality,
    /// The code the Pokémon Trading Card Game Online uses to identify a set.
    #[serde(alias = "ptcgoCode")]
    pub ptcgo_code: Option<String>,
    /// The date the set was released (in the USA). Format is YYYY/MM/DD.
    #[serde(alias = "releaseDate")]
    pub release_date: String,
    /// The date and time the set was updated. Format is YYYY/MM/DD HH:MM:SS.
    #[serde(alias = "updatedAt")]
    pub updated_at: String,
    /// Any images associated with the set, such as symbol and logo.
    pub images: SetImages,
}

pub struct GetSetRequest {
    pub id: String,
}

impl GetSetRequest {
    pub fn new(id: &str) -> Self {
        GetSetRequest { id: id.into() }
    }
}

pub struct SearchSetsRequest {
    ///The search query.
    pub query: Option<String>,
    /// The page of data to access.
    pub page: Option<u16>,
    /// The maximum amount of sets to return. Max of 250.
    pub page_size: Option<u8>,
    /// The field(s) to order the results by.
    pub order_by: Option<String>,
}

impl SearchSetsRequest {
    pub fn new(query: &str) -> Self {
        SearchSetsRequest {
            query: Some(String::from(query)),
            page: None,
            page_size: None,
            order_by: None,
        }
    }
}

impl Client {
    /// Fetch the details of a single set.
    ///
    /// https://docs.pokemontcg.io/api-reference/sets/get-set
    pub async fn get_set(&self, request: GetSetRequest) -> Result<Set, ClientError> {
        let set = self
            .http_client
            .get(format!("{}/sets/{}", self.base_url, request.id))
            .send()
            .await?
            .json::<ApiResult<Set>>()
            .await?;

        Client::parse_response(set)
    }

    /// Search for one or many sets given a search query.
    ///
    /// https://docs.pokemontcg.io/api-reference/sets/search-cards
    pub async fn search_sets(&self, request: SearchSetsRequest) -> Result<Vec<Set>, ClientError> {
        let sets = self
            .http_client
            .get(format!("{}/sets", self.base_url))
            .query(&[
                ("q", request.query),
                ("page", request.page.map(|p| p.to_string())),
                ("pageSize", request.page_size.map(|p| p.to_string())),
                ("orderBy", request.order_by),
            ])
            .send()
            .await?
            .json::<ApiResult<Vec<Set>>>()
            .await?;

        Client::parse_response(sets)
    }

    /// Get all sets (automatically pages through data)
    pub async fn get_all_sets(&self) -> Result<Vec<Set>, ClientError> {
        let mut page = 1;
        let page_size = 250;
        let mut total_pages: usize = 0;
        let mut sets: Vec<Set> = vec![];

        loop {
            let resp = self
                .http_client
                .get(format!("{}/sets", self.base_url))
                .query(&[("page", page)])
                .send()
                .await?
                .json::<ApiResult<Vec<Set>>>()
                .await?;

            let total_count = if let ApiResult::Ok(ev) = &resp {
                ev.total_count
            } else {
                None
            };

            match Client::parse_response(resp) {
                Ok(mut cv) => {
                    sets.append(&mut cv);
                    if sets.len() % page_size != 0 {
                        break;
                    }
                    if let Some(tc) = total_count {
                        total_pages = ((tc / page_size) as f64).ceil() as usize;
                    }

                    if page > total_pages {
                        break;
                    }

                    page += 1;
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }

        Ok(sets)
    }
}

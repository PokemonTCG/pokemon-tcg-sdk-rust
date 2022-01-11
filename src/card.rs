mod ability;
mod ancient_trait;
mod attack;
mod cardmarket;
mod image;
mod legality;
mod resistance;
mod tcgplayer;
mod weakness;

use std::str::FromStr;

use crate::{
    client::{ApiResult, Client},
    errors::ClientError,
    set::Set,
};
use serde::{Deserialize, Serialize};

use self::{
    ability::Ability, ancient_trait::AncientTrait, attack::Attack, cardmarket::CardMarket,
    image::Image, legality::Legality, resistance::Resistance, tcgplayer::TcgPlayer,
    weakness::Weakness,
};

/// The Card Object
/// https://docs.pokemontcg.io/api-reference/cards/card-object
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Card {
    /// Unique identifier for the object.
    pub id: String,
    /// The name of the card.
    pub name: String,
    /// The supertype of the card, such as Pokémon, Energy, or Trainer.
    pub supertype: String,
    /// A list of subtypes, such as Basic, EX, Mega, Rapid Strike, etc.
    pub subtypes: Option<Vec<String>>,
    /// The level of the card. This only pertains to cards from older sets and those of supertype Pokémon.
    pub level: Option<String>,
    /// The hit points of the card.
    pub hp: Option<String>,
    /// The energy types for a card, such as Fire, Water, Grass, etc.
    pub types: Option<Vec<String>>,
    /// Which Pokémon this card evolves from.
    #[serde(alias = "evolvesFrom")]
    pub evolves_from: Option<String>,
    /// Which Pokémon this card evolves to. Can be multiple, for example, Eevee.
    #[serde(alias = "evolvesTo")]
    pub evolves_to: Option<Vec<String>>,
    /// Any rules associated with the card. For example, VMAX rules, Mega rules, or various trainer rules.
    pub rules: Option<Vec<String>>,
    /// The ancient trait for a given card.
    #[serde(alias = "ancientTrait")]
    pub ancient_trait: Option<AncientTrait>,
    /// One or more abilities for a given card.
    pub abilities: Option<Vec<Ability>>,
    /// One or more attacks for a given card
    pub attacks: Option<Vec<Attack>>,
    /// One or more weaknesses for a given card
    pub weaknesses: Option<Vec<Weakness>>,
    /// One or more resistances for a given card
    pub resistances: Option<Vec<Resistance>>,
    /// A list of costs it takes to retreat and return the card to your bench. Each cost is an energy type, such as Water or Fire.
    #[serde(alias = "retreatCost")]
    pub retreat_cost: Option<Vec<String>>,
    /// The converted retreat cost for a card is the count of energy types found within the retreatCost field. For example, ["Fire", "Water"] has a converted retreat cost of 2.
    #[serde(alias = "convertedRetreatCost")]
    pub converted_retreat_cost: Option<usize>,
    /// The set details embedded into the card. See the set object for more details.
    pub set: Set,
    /// The number of the card.
    pub number: Option<String>,
    /// The artist of the card.
    pub artist: Option<String>,
    /// The rarity of the card, such as "Common" or "Rare Rainbow".
    pub rarity: Option<String>,
    /// The flavor text of the card. This is the text that can be found on some Pokémon cards that is usually italicized near the bottom of the card.
    #[serde(alias = "flavorText")]
    pub flavor_text: Option<String>,
    /// The national pokedex numbers associated with any Pokémon featured on a given card.
    #[serde(alias = "nationalPokedexNumbers")]
    pub national_pokedex_numbers: Option<Vec<usize>>,
    /// The legalities for a given card. A legality will not be present in the hash if it is not legal. If it is legal or banned, it will be present.
    pub legalities: Option<Legality>,
    /// A letter symbol found on each card that identifies whether it is legal to use in tournament play. Regulation marks were introduced on cards in the Sword & Shield Series.
    #[serde(alias = "regulationMark")]
    pub regulation_mark: Option<String>,
    /// The images for a card.
    pub images: Option<Image>,
    /// The TCGPlayer information for a given card. ALL PRICES ARE IN US DOLLARS.
    pub tcgplayer: Option<TcgPlayer>,
    /// The cardmarket information for a given card. ALL PRICES ARE IN EUROS.
    pub cardmarket: Option<CardMarket>,
}

pub struct GetCardRequest {
    pub id: String,
}

impl GetCardRequest {
    pub fn new(id: &str) -> Self {
        GetCardRequest { id: id.into() }
    }
}

pub struct SearchCardsRequest {
    ///The search query.
    pub query: Option<String>,
    /// The page of data to access.
    pub page: Option<u16>,
    /// The maximum amount of cards to return. Max of 250.
    pub page_size: Option<u8>,
    /// The field(s) to order the results by.
    pub order_by: Option<String>,
}

impl SearchCardsRequest {
    pub fn new(query: &str) -> Self {
        SearchCardsRequest {
            query: Some(String::from_str(query).unwrap()),
            page: None,
            page_size: None,
            order_by: None,
        }
    }
}

impl Client {
    /// Fetch the details of a single card.
    ///
    /// https://docs.pokemontcg.io/api-reference/cards/get-card
    pub async fn get_card(&self, request: GetCardRequest) -> Result<Card, ClientError> {
        let card = self
            .http_client
            .get(format!("{}/cards/{}", self.base_url, request.id))
            .send()
            .await?
            .json::<ApiResult<Card>>()
            .await?;

        Client::parse_response(card)
    }

    /// Search for one or many cards given a search query.
    ///
    /// https://docs.pokemontcg.io/api-reference/cards/search-cards
    pub async fn search_cards(
        &self,
        request: SearchCardsRequest,
    ) -> Result<Vec<Card>, ClientError> {
        let cards = self
            .http_client
            .get(format!("{}/cards", self.base_url))
            .query(&[
                ("q", request.query),
                ("page", request.page.map(|p| p.to_string())),
                ("pageSize", request.page_size.map(|p| p.to_string())),
                ("orderBy", request.order_by),
            ])
            .send()
            .await?
            .json::<ApiResult<Vec<Card>>>()
            .await?;

        Client::parse_response(cards)
    }

    /// Get all cards (will take awhile, automatically pages through data)
    pub async fn get_all_cards(&self) -> Result<Vec<Card>, ClientError> {
        let mut page = 1;
        let page_size = 250;
        let mut total_pages: usize = 0;
        let mut cards: Vec<Card> = vec![];

        loop {
            let resp = self
                .http_client
                .get(format!("{}/cards", self.base_url))
                .query(&[("page", page)])
                .send()
                .await?
                .json::<ApiResult<Vec<Card>>>()
                .await?;

            let total_count = if let ApiResult::Ok(ev) = &resp {
                ev.total_count
            } else {
                None
            };

            match Client::parse_response(resp) {
                Ok(mut cv) => {
                    cards.append(&mut cv);
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

        Ok(cards)
    }
}

pub mod card;
pub mod client;
pub mod errors;
pub mod set;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::card::{Card, SearchCardsRequest};
    use crate::client::{ApiResult, DataEnvelope};
    use crate::set::{GetSetRequest, SearchSetsRequest};
    use crate::{card::GetCardRequest, client::Client};
    use wiremock::matchers::{header, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn get_card_sends_request_with_proper_id() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        let test_card_id = "base1-5";

        Mock::given(path(format!("/cards/{}", test_card_id)))
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_card(GetCardRequest::new(test_card_id)).await;
    }

    #[tokio::test]
    async fn appends_api_key_to_header() {
        let mock_server = MockServer::start().await;
        let api_key = "abc123";
        let client = Client::with_base_url(mock_server.uri().as_str(), Some(api_key)).unwrap();
        let test_card_id = "base1-5";

        Mock::given(path(format!("/cards/{}", test_card_id)))
            .and(header("X-Api-Key", api_key))
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_card(GetCardRequest::new(test_card_id)).await;
    }

    #[tokio::test]
    async fn search_cards_sends_request_with_params() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();
        let query = "name:celebi";
        let page = 1;
        let page_size = 250;
        let order_by = "name,-number";

        Mock::given(path("/cards"))
            .and(query_param("q", query))
            .and(query_param("page", page.to_string()))
            .and(query_param("pageSize", page_size.to_string()))
            .and(query_param("orderBy", order_by.to_string()))
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client
            .search_cards(SearchCardsRequest {
                query: Some(String::from(query)),
                page: Some(page),
                page_size: Some(page_size),
                order_by: Some(String::from(order_by)),
            })
            .await;
    }

    #[tokio::test]
    async fn get_all_cards_makes_multiple_requests() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();
        let body = ApiResult::Ok(DataEnvelope {
            data: vec![Card::default()],
            // greater than the single page size
            total_count: Some(251),
        });

        let card_response = ResponseTemplate::new(200).set_body_json(body);

        Mock::given(path("/cards"))
            .respond_with(card_response)
            .expect(2)
            .mount(&mock_server)
            .await;

        let _result = client.get_all_cards().await;
    }

    #[tokio::test]
    async fn search_sets_sends_request_with_params() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();
        let query = "name:base1";
        let page = 1;
        let page_size = 250;
        let order_by = "name,-number";

        Mock::given(path("/sets"))
            .and(query_param("q", query))
            .and(query_param("page", page.to_string()))
            .and(query_param("pageSize", page_size.to_string()))
            .and(query_param("orderBy", order_by.to_string()))
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client
            .search_sets(SearchSetsRequest {
                query: Some(String::from(query)),
                page: Some(page),
                page_size: Some(page_size),
                order_by: Some(String::from(order_by)),
            })
            .await;
    }

    #[tokio::test]
    async fn get_set_sends_request_with_proper_id() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();
        let test_set_id = "base1";

        Mock::given(path(format!("/sets/{}", test_set_id)))
            .respond_with(ResponseTemplate::new(400))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_set(GetSetRequest::new(test_set_id)).await;
    }

    #[tokio::test]
    async fn get_all_sets_sends_request_to_correct_url() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        Mock::given(path("/sets"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_all_sets().await;
    }

    #[tokio::test]
    async fn get_all_types_hits_correct_url() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        Mock::given(path("/types"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_types().await;
    }

    #[tokio::test]
    async fn get_all_subtypes_hits_correct_url() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        Mock::given(path("/subtypes"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_subtypes().await;
    }

    #[tokio::test]
    async fn get_all_supertypes_hits_correct_url() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        Mock::given(path("/supertypes"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_supertypes().await;
    }

    #[tokio::test]
    async fn get_all_rarities_hits_correct_url() {
        let mock_server = MockServer::start().await;
        let client = Client::with_base_url(mock_server.uri().as_str(), None).unwrap();

        Mock::given(path("/rarities"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = client.get_rarities().await;
    }
}

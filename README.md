# Pokémon TCG SDK - Rust

This is the Pokémon TCG SDK Rust implementation. It is a wrapper around the Pokémon TCG API of [pokemontcg.io](http://pokemontcg.io/).

## Usage

### Configuration

```toml
[dependencies]
pokemon-tcg-sdk = "0.2.0"
```

### Using an API Key
```rust 
// This method fails for the same reasons a reqwest::ClientBuilder would fail (TLS, system config)
// or if your API key contains invalid characters for a header.
let client = Client::with_api_key("YOUR_API_KEY")?;
```


### Cards

#### Get a single card by ID

```rust
let client = Client::default();
let card = client.get_card(GetCardRequest::new("base1-1")).await;

match card {
    // Card
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

#### Filter cards via the q parameter

```rust
let client = Client::default();
let cards = client.search_cards(SearchCardsRequest::new("name:celebi")).await;

match cards {
    // Vec<Card>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}

// You can also construct a SearchCardsRequest with more parameters
let search_request = SearchCardsRequest {
    query: Some(String::from("name:celebi")),
    page: Some(10),
    page_size: None,
    order_by: None,
}
```

#### Automatically page through card data

```rust
let client = Client::default();
let cards = client.get_all_cards().await;

match cards {
    // Vec<Card>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

### Sets

#### Get a single set by ID

```rust
let client = Client::default();
let set = client.get_set(GetSetRequest::new("base1")).await;

match set {
    // Set
    Ok(s) => println!("{:?}", s),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

#### Filter sets via the q parameter

```rust
let client = Client::default();
let sets = client.search_sets(SearchSetsRequest::new("series:base")).await;

match sets {
    // Vec<Set>
    Ok(s) => println!("{:?}", s),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}

// You can also construct a SearchSetsRequest with more parameters
let search_request = SearchSetsRequest {
    query: Some(String::from("series:base")),
    page: Some(2),
    page_size: None,
    order_by: None,
}
```

#### Automatically page through set data

```rust
let client = Client::default();
let sets = client.get_all_sets().await;

match sets {
    // Vec<Set>
    Ok(s) => println!("{:?}", s),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

### Supertypes

```rust
let client = Client::default();
let types = client.get_supertypes().await;

match types {
    // Vec<String>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

### Subtypes

```rust
let client = Client::default();
let types = client.get_subtypes().await;

match types {
    // Vec<String>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

### Types

```rust
let client = Client::default();
let types = client.get_types().await;

match types {
    // Vec<String>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

### Rarities

```rust
let client = Client::default();
let types = client.get_rarities().await;

match types {
    // Vec<String>
    Ok(c) => println!("{:?}", c),
    // Will be a 'ClientError' enum
    Err(e) => println!("{:?}", e),
}
```

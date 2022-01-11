use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Default))]
pub struct SetImages {
    /// The url to the symbol image.
    pub symbol: String,
    /// The url to the logo image.
    pub logo: String,
}

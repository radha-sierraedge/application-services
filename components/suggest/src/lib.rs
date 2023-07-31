/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use remote_settings::RemoteSettingsConfig;
mod db;
mod error;
mod keyword;
mod rs;
mod schema;
mod store;

pub use error::SuggestApiError;
use rs::SuggestionProvider;
pub use store::{SuggestIngestionConstraints, SuggestStore};

pub(crate) type Result<T> = std::result::Result<T, error::Error>;
pub type SuggestApiResult<T> = std::result::Result<T, error::SuggestApiError>;

/// A suggestion from the database to show in the address bar.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Suggestion {
    pub block_id: i64,
    pub advertiser: String,
    pub iab_category: String,
    pub is_sponsored: bool,
    pub full_keyword: String,
    pub title: String,
    pub url: String,
    pub icon: Option<Vec<u8>>,
    pub impression_url: Option<String>,
    pub click_url: Option<String>,
    pub provider: SuggestionProvider,
}

/// A query for suggestions to show in the address bar.
#[derive(Debug, Default)]
pub struct SuggestionQuery {
    pub keyword: String,
    pub include_sponsored: bool,
    pub include_non_sponsored: bool,
}

uniffi::include_scaffolding!("suggest");

#[cfg(feature = "commands")]
pub mod details;
#[cfg(feature = "commands")]
pub mod images;
#[cfg(feature = "commands")]
pub mod latest;
#[cfg(feature = "commands")]
pub mod popular;
#[cfg(feature = "commands")]
pub mod search;
#[cfg(feature = "commands")]
pub mod similar;
#[cfg(feature = "commands")]
pub mod watch_providers;

pub mod episode;
pub mod season;

use crate::common::country::Country;
use crate::common::language::Language;
use crate::company::CompanyShort;
use crate::genre::Genre;
use crate::people::PersonShort;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TVShowBase {
    pub id: u64,
    pub name: String,
    pub original_name: String,
    pub original_language: String,
    pub origin_country: Vec<String>,
    #[serde(default)]
    pub overview: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub first_air_date: Option<chrono::NaiveDate>,
    #[serde(default)]
    pub poster_path: Option<String>,
    #[serde(default)]
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    #[serde(default)]
    pub adult: bool,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TVShowShort {
    #[serde(flatten)]
    pub inner: TVShowBase,
    pub genre_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeShort {
    pub air_date: chrono::NaiveDate,
    pub episode_number: u64,
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub overview: Option<String>,
    pub production_code: String,
    pub season_number: u64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Episode {
    #[serde(flatten)]
    pub inner: EpisodeShort,
    //
    pub crew: Vec<PersonShort>,
    pub guest_stars: Vec<PersonShort>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SeasonBase {
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub air_date: Option<chrono::NaiveDate>,
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SeasonShort {
    #[serde(flatten)]
    pub inner: SeasonBase,
    //
    pub episode_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Season {
    pub _id: String,
    #[serde(flatten)]
    pub inner: SeasonBase,
    pub episodes: Vec<Episode>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TVShow {
    #[serde(flatten)]
    pub inner: TVShowBase,
    pub created_by: Vec<PersonShort>,
    pub episode_run_time: Vec<u64>,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub in_production: bool,
    pub languages: Vec<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub last_air_date: Option<chrono::NaiveDate>,
    pub last_episode_to_air: Option<EpisodeShort>,
    pub next_episode_to_air: Option<EpisodeShort>,
    pub networks: Vec<CompanyShort>,
    /// Unlikely to be `None` but found with 81040.
    /// In this case, could be computed by summing the `episodes_count` of the `seasons` field.
    pub number_of_episodes: Option<u64>,
    pub number_of_seasons: u64,
    pub production_companies: Vec<CompanyShort>,
    pub production_countries: Vec<Country>,
    pub seasons: Vec<SeasonShort>,
    pub spoken_languages: Vec<Language>,
    pub status: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub tagline: Option<String>,
    #[serde(rename = "type")]
    pub ttype: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeGroupItem {
    pub description: String,
    pub episode_count: u64,
    pub group_count: u64,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EpisodeGroup {
    pub results: Vec<EpisodeGroupItem>,
    pub id: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct EpisodeGroupDetailsItem {
    pub id: String,
    pub name: String,
    pub order: u64,
    pub episodes: Vec<EpisodeShort>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct EpisodeGroupDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub episode_count: u64,
    pub group_count: u64,
    #[serde(rename = "type")]
    pub group_type: i32,
    pub groups: Vec<EpisodeGroupDetailsItem>,
}

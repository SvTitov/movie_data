use anyhow::Result;

use crate::connectors::omdb::dto::OmdbDto;

mod dto {
    use serde::Deserialize;

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OmdbDto {
        #[serde(rename = "Title")]
        pub title: Option<String>,

        #[serde(rename = "Year")]
        pub year: Option<String>,

        #[serde(rename = "Rated")]
        pub rated: Option<String>,

        #[serde(rename = "Released")]
        pub released: Option<String>,

        #[serde(rename = "Runtime")]
        pub runtime: Option<String>,

        #[serde(rename = "Genre")]
        pub genre: Option<String>,

        #[serde(rename = "Director")]
        pub director: Option<String>,

        #[serde(rename = "Writer")]
        pub writer: Option<String>,

        #[serde(rename = "Actors")]
        pub actors: Option<String>,

        #[serde(rename = "Plot")]
        pub plot: Option<String>,

        #[serde(rename = "Language")]
        pub language: Option<String>,

        #[serde(rename = "Country")]
        pub country: Option<String>,

        #[serde(rename = "Awards")]
        pub awards: Option<String>,

        #[serde(rename = "Poster")]
        pub poster: Option<String>,

        #[serde(rename = "Ratings")]
        pub ratings: Option<Vec<Rating>>,

        #[serde(rename = "Metascore")]
        pub metascore: Option<String>,

        pub imdb_rating: Option<String>,

        pub imdb_votes: Option<String>,

        #[serde(rename = "imdbID")]
        pub imdb_id: Option<String>,

        #[serde(rename = "Type")]
        pub type_field: Option<String>,

        #[serde(rename = "DVD")]
        pub dvd: Option<String>,

        #[serde(rename = "BoxOffice")]
        pub box_office: Option<String>,

        #[serde(rename = "Production")]
        pub production: Option<String>,

        #[serde(rename = "Website")]
        pub website: Option<String>,

        #[serde(rename = "Response")]
        pub response: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Rating {
        #[serde(rename = "Source")]
        pub source: Option<String>,

        #[serde(rename = "Value")]
        pub value: Option<String>,
    }
}

const HOST: &str = "http://www.omdbapi.com/";

pub struct OmdbConnector {
    apikey: String,
}

impl OmdbConnector {
    pub fn new(apikey: &str) -> Self {
        Self {
            apikey: apikey.to_string(),
        }
    }

    pub async fn get_info(&self, title: &str) -> Result<OmdbDto> {
        let url = format!("{}?apikey={}&t={}", HOST, self.apikey, title);
        let response = reqwest::get(url).await?;
        let dto = response.json::<OmdbDto>().await?;

        Ok(dto)
    }
}

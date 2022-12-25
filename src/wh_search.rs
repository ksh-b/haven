use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Search {
    #[serde(rename = "data")]
    pub(crate) data: Vec<Datum>,

    #[serde(rename = "meta")]
    pub meta: Meta,
}

impl Search {
    pub fn empty () -> Self{
        Search { data: vec![], meta: Meta {
            current_page: 0,
            last_page: 0,
            per_page: 0,
            total: 0,
            query: None,
            seed: None,
        } }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "short_url")]
    pub short_url: String,

    #[serde(rename = "views")]
    pub views: i64,

    #[serde(rename = "favorites")]
    pub favorites: i64,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "purity")]
    pub purity: Purity,

    #[serde(rename = "category")]
    pub category: Category,

    #[serde(rename = "dimension_x")]
    pub dimension_x: i64,

    #[serde(rename = "dimension_y")]
    pub dimension_y: i64,

    #[serde(rename = "resolution")]
    pub resolution: String,

    #[serde(rename = "ratio")]
    pub ratio: String,

    #[serde(rename = "file_size")]
    pub file_size: i64,

    #[serde(rename = "file_type")]
    pub file_type: FileType,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "colors")]
    pub colors: Vec<String>,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "thumbs")]
    pub thumbs: Thumbs,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbs {
    #[serde(rename = "large")]
    pub large: String,

    #[serde(rename = "original")]
    pub original: String,

    #[serde(rename = "small")]
    pub small: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "current_page")]
    pub current_page: i32,

    #[serde(rename = "last_page")]
    pub last_page: i32,

    #[serde(rename = "per_page")]
    pub per_page: i32,

    #[serde(rename = "total")]
    pub total: i32,

    #[serde(rename = "query")]
    pub query: Option<serde_json::Value>,

    #[serde(rename = "seed")]
    pub seed: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "anime")]
    Anime,

    #[serde(rename = "general")]
    General,
}

#[derive(Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "image/jpeg")]
    ImageJpeg,

    #[serde(rename = "image/png")]
    ImagePng,
}

#[derive(Serialize, Deserialize)]
pub enum Purity {
    #[serde(rename = "sfw")]
    Sfw,
}

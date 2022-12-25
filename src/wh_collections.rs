use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Collections {
    #[serde(rename = "data")]
    pub(crate) data: Vec<CollectionData>
}

impl Collections {
    pub fn empty () -> Self{
        Collections { data: vec![] }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CollectionData {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "views")]
    pub views: i32,

    #[serde(rename = "public")]
    pub public: i32,

    #[serde(rename = "count")]
    pub count: i32,
}

use hdk::prelude::holo_hash::AnyLinkableHashB64;
use hdk::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexHashByKeywordsInput {
    pub index_name: String,
    pub indexed_hash: AnyLinkableHashB64,
    pub indexed_by_keywords: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchByKeywordInput {
    pub index_name: String,
    pub keyword_prefix: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchByKeywordMatch {
    pub matched_hash: AnyLinkableHashB64,
    pub matching_keyword: String,
}

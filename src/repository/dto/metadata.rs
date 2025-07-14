use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListLanguageReply {
    pub data: Vec<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SdtmVersion {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmVersionRequest {
    pub lang_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSdtmVersionReply {
    pub data: Vec<SdtmVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SdtmDomain {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmDomainsRequest {
    pub version_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSdtmDomainsReply {
    pub data: Vec<SdtmDomain>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdtmVariable {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub variable_type: String,
    pub codelist: String,
    pub variable_core: String,
    pub variable_role: String,
    pub variable_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSdtmVariableRequest {
    pub domain_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSdtmVariableReply {
    pub data: Vec<SdtmVariable>,
}

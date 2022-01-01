use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlertResourceCollection {
    pub uri: String,
    pub category: String,
    pub created: String,
    pub modified: String,
    pub start: u64,
    pub count: u64,
    pub total: u64,
    #[serde(rename = "prevPageUri")]
    pub prevpageuri: Option<String>,
    #[serde(rename = "nextPageUri")]
    pub nextpageuri: Option<String>,
    pub members: Vec<AlertSource>,
}

#[derive(Serialize, Deserialize)]
pub struct AlertSource {
    pub uri: String,
    pub category: String,
    pub created: String,
    pub modified: String,
    #[serde(rename = "associatedEventUris")]
    pub associatedeventuris: Vec<String>,
    #[serde(rename = "physicalResourceType")]
    pub physicalresourcetype: String,
    pub severity: String,
    #[serde(rename = "alertState")]
    pub alertstate: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventResource {
    pub uri: String,
    pub created: String,
    pub modified: String,
    #[serde(rename = "healthCategory")]
    pub healthcategory: String,
    pub description: String,
    #[serde(rename = "rxTime")]
    pub rxtime: String,
    pub processed: bool,
    pub severity: String,
    pub urgency: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    #[serde(rename = "sessionID")]
    pub sessionid: Option<String>,
}

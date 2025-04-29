// Больше не нужно: extern crate quick_xml as xml;
// И не нужно: use yaserde::YaDeserialize;

use serde::{Deserialize, Serialize};

// Эти структуры будут использоваться позже
#[derive(Debug, Serialize, Deserialize)]
pub struct SoapEnvelope {
    pub body: SoapBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoapBody {
    pub discover: Option<Discover>,
    // pub execute: Option<Execute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Discover {
    pub request_type: String,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub property_list: PropertyList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyList {
    pub data_source_info: Option<String>,
    pub catalog: Option<String>,
}
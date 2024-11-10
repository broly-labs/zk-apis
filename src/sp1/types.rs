use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub output: Mutex<Option<Vec<u8>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OutputRequest {
    pub output: Vec<u8>,
}

#[derive(Serialize)]
pub struct OutputResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateProofRequest {
    pub elf: Vec<u8>,
    pub input: Vec<u8>
}

#[derive(Serialize)]
pub struct GenerateProofResponse {
    pub proof: Vec<u8>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateElfRequest {
    pub destination_path: String
}

#[derive(Debug, Serialize)]
pub struct GenerateElfResponse {
    pub success: bool,
    pub message: String,
    pub file_path: String,
}

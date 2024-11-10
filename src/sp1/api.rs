use actix_web::{web, Responder};
use crate::sp1::proof::{generate_proof_and_verify, add_inputs};
use crate::sp1::generate_elf::generate_elf;
use crate::sp1::types::{
    AppState,
    OutputRequest,
    OutputResponse,
    GenerateProofRequest,
    GenerateProofResponse,
    GenerateElfRequest, 
    GenerateElfResponse
};

pub async fn set_output(data: web::Json<OutputRequest>, state: web::Data<AppState>) -> impl Responder {
    let mut output = state.output.lock().unwrap();
    *output = Some(data.output.clone());
    web::Json(OutputResponse {
        success: true,
        message: "Output set successfully".to_string(),
    })
}

pub async fn handle_generate_proof_and_verify(
    request: web::Json<GenerateProofRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let output = state.output.lock().unwrap().clone();
    match output {
        None => Ok(web::Json(GenerateProofResponse {
            success: false,
            proof: Vec::new(),
            message: "Output not added".to_string(),
        })),
        Some(output) => {
            let stdin = add_inputs(request.input.clone(), output);
            match generate_proof_and_verify(&request.elf, stdin) {
                Ok(_proof) => Ok(web::Json(GenerateProofResponse { 
                    success: true,
                    proof: _proof,
                    message: "".to_string(),
                })),
                Err(_) => Ok(web::Json(GenerateProofResponse { 
                    success: false,
                    proof: Vec::new(),
                    message: "Incorrect prove".to_string(),
                }))
            }
        }
    }
}

pub async fn handle_generate_elf(
    request: web::Json<GenerateElfRequest>,
) -> Result<impl Responder, actix_web::Error> {
    match generate_elf(&request.destination_path) {
        Ok(file_path) => Ok(web::Json(GenerateElfResponse {
            success: true,
            message: "ELF file generated successfully".to_string(),
            file_path,
        })),
        Err(e) => Ok(web::Json(GenerateElfResponse {
            success: false,
            message: format!("Error: {}", e),
            file_path: String::new(),
        })),
    }
}

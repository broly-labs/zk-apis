mod sp1;

use actix_web::{web, App, HttpServer};
use sp1::types::AppState;
use std::sync::Mutex;
use sp1::api::{
    set_output, 
    handle_generate_proof_and_verify,
    handle_generate_elf
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        output: Mutex::new(None),
    });

    println!("Server starting on http://localhost:3030");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/sp1/set-output", web::post().to(set_output))
            .route("/sp1/generate-elf", web::post().to(handle_generate_elf))
            .route("/sp1/generate-proof-and-verify", web::post().to(handle_generate_proof_and_verify))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}

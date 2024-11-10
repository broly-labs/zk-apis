use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use uuid::Uuid;

pub fn generate_elf(destination: &str) -> Result<String, String> {
    // Create a unique temporary directory within the caller's path
    let caller_dir = Path::new(destination);
    let temp_dir = caller_dir.join(format!("program_{}", Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    // Create Cargo.toml
    let cargo_toml = include_str!("program/cargo.toml");
    fs::write(temp_dir.join("Cargo.toml"), cargo_toml).map_err(|e| e.to_string())?;

    // Create src directory and main.rs
    fs::create_dir_all(temp_dir.join("src")).map_err(|e| e.to_string())?;

    // Write the program
    let program_code = include_str!("program/program.rs");
    fs::write(temp_dir.join("src/main.rs"), program_code).map_err(|e| e.to_string())?;

    // Run `cargo prove build` in the temporary directory
    let status = Command::new("cargo")
        .args(&["prove", "build"])
        .current_dir(&temp_dir)
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        fs::remove_dir_all(&temp_dir).ok();
        return Err("Failed to build program".to_string());
    }

    // Create destination directory if it doesn't exist
    let destination_path = PathBuf::from(destination);
    if let Some(parent) = destination_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // Move the generated ELF file from temp project to destination
    let source_elf = temp_dir.join("elf/riscv32im-succinct-zkvm-elf");
    let destination_file = destination_path.join("riscv32im-succinct-zkvm-elf");
    
    fs::copy(&source_elf, &destination_file).map_err(|e| e.to_string())?;

    // Clean up the temporary directory
    fs::remove_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    Ok(destination_file.to_string_lossy().into_owned())
}

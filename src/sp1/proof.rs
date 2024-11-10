use sp1_sdk::{ProverClient, SP1Stdin};

pub fn add_inputs(input: Vec<u8>, output: Vec<u8>) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);
    stdin.write(&output);
    stdin
}

pub fn generate_proof_and_verify(
    elf: &[u8],
    stdin: SP1Stdin
) -> Result<Vec<u8>, String> {
    let client = ProverClient::new();
    let (pk, vk) = client.setup(elf);

    let Ok(proof) = client.prove(&pk, stdin).run() else {
        println!("Incorrect prove!");
        return Err("Incorrect prove!".to_string());
    };

    client.verify(&proof, &vk).expect("verification failed");
    let proof = bincode::serialize(&proof).expect("Failed to serialize proof");
    Ok(proof)
}


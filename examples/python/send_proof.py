import requests
from aligned_py.sdk import *
from aligned_py.core.types import *
import asyncio
from web3 import Web3

API_URL = "http://localhost:3030"
rpc_api = "https://ethereum-holesky-rpc.publicnode.com"
private_key = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'

def open_file(elf_file_path):
    with open(elf_file_path, 'rb') as f:
        elf_data = f.read()
    return elf_data

def generate_proof(elf_file_path, input_data):
    url = f"{API_URL}/sp1/generate-proof-and-verify"

    elf_data = open_file(elf_file_path)
    input_bytes = input_data.encode() if isinstance(input_data, str) else input_data

    data = { "elf": list(elf_data), "input": list(input_bytes) }

    response = requests.post(url, json=data)
    response_json = response.json()

    if response_json.get("message") == "Incorrect prove":
        raise ValueError("Incorrect prove!")
    elif response_json.get("message") == "Output not added":
        raise ValueError("Output not added!")

    return response_json

async def main():
    web3 = Web3(Web3.HTTPProvider(rpc_api))
    account = web3.eth.account.from_key(private_key)
    nonce = get_next_nonce(rpc_api, account.address, Network.Holesky)
    max_fee = estimate_fee(rpc_api, PriceEstimate.Instant)
    print(f"Nonce: {nonce}")

    proof = generate_proof("examples/elf/riscv32im-succinct-zkvm-elf", "cca").get("proof")
    program_code = open_file("examples/elf/riscv32im-succinct-zkvm-elf")

    verification_data = VerificationData(
        proving_system=ProvingSystemId.SP1,
        proof=proof,
        public_input=None,
        verification_key=None,
        vm_program_code=bytearray(program_code),
        proof_generator_address=account.address
    )

    result = await submit_and_wait_verification(
        "wss://batcher.alignedlayer.com",
        rpc_api,
        Network.Holesky,
        verification_data,
        max_fee,
        account,
        nonce
    )
    print(f"0x{bytes(result.batch_merkle_root).hex()}")

asyncio.run(main())

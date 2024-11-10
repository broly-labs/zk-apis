import path from "path";
import { fileURLToPath } from "url";
import axios from 'axios';
import { ethers } from 'ethers';
import fs from 'fs/promises';
import { 
    Network, VerificationData, ProvingSystemId,
    getNextNonce, estimateFee, PriceEstimate,
    submitAndWaitVerification
} from "aligned-js";

const API_URL = 'http://localhost:3030';
const RPC_API = 'https://ethereum-holesky-rpc.publicnode.com';
const PRIVATE_KEY = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function openFile(elfFilePath) {
    const elfData = await fs.readFile(elfFilePath);
    return Array.from(elfData);
}

async function generateProof(elfFilePath, inputData) {
    const url = `${API_URL}/sp1/generate-proof-and-verify`;

    const elfData = await openFile(elfFilePath);
    const inputBytes = typeof inputData === 'string'
        ? Array.from(Buffer.from(inputData, 'utf-8'))
        : inputData;

    const data = { elf: elfData, input: inputBytes };

    const response = await axios.post(url, data);
    const responseJson = response.data;

    if (responseJson.message === 'Incorrect prove') {
        throw new Error('Incorrect prove!');
    } else if (responseJson.message === 'Output not added') {
        throw new Error('Output not added!');
    }

    return responseJson;
}

async function main() {
    const provider = new ethers.JsonRpcProvider(RPC_API);
    const wallet = new ethers.Wallet(PRIVATE_KEY, provider);

    const nonce = await getNextNonce(RPC_API, wallet.getAddress(), Network.Holesky);
    const maxFee = await estimateFee(RPC_API, PriceEstimate.Instant);
    const chainId = (await wallet.provider.getNetwork()).chainId;
    console.log(`Nonce: ${nonce}`);

    const root = path.join(__dirname, "..", "elf", "riscv32im-succinct-zkvm-elf");

    const proofResponse = await generateProof(root, 'cca');
    const proof = proofResponse.proof;
    const programCode = await openFile(root);

    const verificationData = VerificationData.new({
        provingSystem: ProvingSystemId.SP1,
        proof: proof,
        publicInput: null,
        verificationKey: null,
        vmProgramCode: programCode,
        proofGeneratorAddress: String(await wallet.getAddress())
    });

    const result = await submitAndWaitVerification(
        "wss://batcher.alignedlayer.com",
        RPC_API,
        Network.Holesky,
        verificationData,
        maxFee,
        wallet,
        nonce
    )
    console.log(`0x${Buffer.from(new Uint8Array(result.batchMerkleRoot)).toString('hex')}`);
}

main().catch((error) => {
    console.error('Error:', error);
});

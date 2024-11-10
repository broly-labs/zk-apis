import axios from "axios";
import path from "path";
import { fileURLToPath } from "url";

const API_URL = 'http://localhost:3030';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function generateElf() {
    const url = `${API_URL}/sp1/generate-elf`;

    const destinationPath = path.join(__dirname, "..", "elf");

    const data = { destination_path: destinationPath };

    return axios.post(url, data)
        .then(response => response.data)
        .catch(error => {
            console.error('Error:', error);
            throw error;
        });
}

(async () => {
    try {
        const result = await generateElf();
        console.log(`Result: ${JSON.stringify(result)}`);
    } catch (error) {
        console.error('Failed to generate ELF:', error);
    }
})();

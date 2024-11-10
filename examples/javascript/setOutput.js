import axios from "axios";

const API_URL = 'http://localhost:3030';

function setOutput(outputValue) {
    // Convert the string to a byte array if outputValue is a string
    const outputArray = typeof outputValue === 'string'
        ? Array.from(Buffer.from(outputValue, 'utf-8'))
        : outputValue;

    const url = `${API_URL}/sp1/set-output`;
    const data = { output: outputArray };

    return axios.post(url, data)
        .then(response => response.data)
        .catch(error => {
            console.error('Error:', error);
            throw error;
        });
}

(async () => {
    try {
        const setSecretResult = await setOutput("cca");
        console.log(`Set Secret Result: ${JSON.stringify(setSecretResult)}`);
    } catch (error) {
        console.error('Failed to set output:', error);
    }
})();

import os
import requests

API_URL = "http://localhost:3030"

def generate_elf():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    destination_path = os.path.join(script_dir, "examples", "elf")
    
    url = f"{API_URL}/sp1/generate-elf"
    data = {"destination_path": destination_path}
    
    response = requests.post(url, json=data)
    return response.json()

if __name__ == "__main__":
    result = generate_elf()

    print(f"Result: {result}")

import requests

API_URL = "http://localhost:3030"

def set_output(output_value):
    """Send a request to set the output value on the server."""
    url = f"{API_URL}/sp1/set-output"

    output_value = output_value.encode() if isinstance(output_value, str) else output_value
    
    data = {"output": list(output_value)}
    response = requests.post(url, json=data)
    return response.json()

if __name__ == "__main__":
    set_secret_result = set_output("cca")

    print(f"Set Secret Result: {set_secret_result}")
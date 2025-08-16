import requests
import json

# Test the deploy endpoint
deploy_url = "http://localhost:8081/api/dapp/deploy"
deploy_data = {
    "contract_path": "src/dapp_examples/smart_contract.sol",
    "network": "testnet"
}

try:
    response = requests.post(deploy_url, json=deploy_data)
    print(f"Deploy API Response Status Code: {response.status_code}")
    print(f"Deploy API Response Body: {response.text}")
except Exception as e:
    print(f"Error calling deploy API: {e}")

# Test the call endpoint
call_url = "http://localhost:8081/api/dapp/call"
call_data = {
    "contract_address": "0x1234567890abcdef1234567890abcdef12345678",
    "function_name": "transfer",
    "params": {
        "to": "0xabcdef1234567890abcdef1234567890abcdef12",
        "amount": 100
    },
    "network": "testnet"
}

try:
    response = requests.post(call_url, json=call_data)
    print(f"Call API Response Status Code: {response.status_code}")
    print(f"Call API Response Body: {response.text}")
except Exception as e:
    print(f"Error calling call API: {e}")
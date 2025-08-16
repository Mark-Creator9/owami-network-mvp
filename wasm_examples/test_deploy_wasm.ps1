# PowerShell script to test deploying a WASM contract to Owami Network

# Build the contract first
.\wasm_examples\compile_contract.ps1

# Deploy the compiled WASM contract
$wasmPath = "wasm_examples\simple_contract\target\wasm32-unknown-unknown\debug\simple_contract.wasm"

# Create a multipart form data request
$boundary = [System.Guid]::NewGuid().ToString()
$headers = @{
    "Content-Type" = "multipart/form-data; boundary=`"$boundary`""
}

# Read the WASM file
$wasmBytes = [System.IO.File]::ReadAllBytes($wasmPath)

# Create the multipart body
$body = New-Object System.IO.MemoryStream
$writer = New-Object System.IO.StreamWriter($body, [System.Text.Encoding]::UTF8)

# Write the file part
$writer.WriteLine("--$boundary")
$writer.WriteLine('Content-Disposition: form-data; name="file"; filename="simple_contract.wasm"')
$writer.WriteLine("Content-Type: application/wasm")
$writer.WriteLine()
$writer.Flush()

# Write the file content
$body.Write($wasmBytes, 0, $wasmBytes.Length)
$writer.WriteLine()
$writer.WriteLine("--$boundary--")
$writer.Flush()

# Convert to byte array
$bodyBytes = $body.ToArray()
$writer.Close()
$body.Close()

try {
    $response = Invoke-RestMethod -Uri 'http://localhost:8081/dapp/deploy_wasm' -Method Post -Headers $headers -Body $bodyBytes
    Write-Host "Deployment Result:"
    $response | ConvertTo-Json -Depth 10 | Write-Host
} catch {
    Write-Host "Error during deployment:"
    Write-Host $_.Exception.Message
    Write-Host $_.ErrorDetails.Message
}
# PowerShell script to test deploying a WASM contract to Owami Network

# Ensure uploads directory exists
if (!(Test-Path "uploads")) {
    New-Item -ItemType Directory -Path "uploads" -Force | Out-Null
}

# Build the contract first
Write-Host "Building WASM contract..."
try {
    .\compile_contract.ps1
    Write-Host "Contract built successfully!"
} catch {
    Write-Host "Error building contract: $_"
    exit 1
}

# Check if WASM file exists
$wasmPath = "simple_contract.wasm"
if (!(Test-Path $wasmPath)) {
    Write-Host "Error: WASM file not found at $wasmPath"
    exit 1
}

# Deploy the compiled WASM contract
Write-Host "Deploying WASM contract..."

# Create a multipart form data request
$boundary = [System.Guid]::NewGuid().ToString()
$headers = @{
    "Content-Type" = "multipart/form-data; boundary=`"$boundary`""
}

try {
    # Read the WASM file
    $wasmBytes = [System.IO.File]::ReadAllBytes($wasmPath)
    
    # Create the multipart body
    $body = New-Object System.IO.MemoryStream
    $writer = New-Object System.IO.StreamWriter($body, [System.Text.Encoding]::UTF8)
    
    # Write the file part
    $writer.Write("--$boundary`r`n")
    $writer.Write("Content-Disposition: form-data; name=`"file`"; filename=`"simple_contract.wasm`"`r`n")
    $writer.Write("Content-Type: application/wasm`r`n`r`n")
    $writer.Flush()
    
    # Write the file content
    $body.Write($wasmBytes, 0, $wasmBytes.Length)
    
    # Write the closing boundary
    $writer.Write("`r`n--$boundary--`r`n")
    $writer.Flush()
    
    # Convert to byte array
    $bodyBytes = $body.ToArray()
    $writer.Close()
    $body.Close()
    
    # Make the API call
    $uri = "http://localhost:8081/api/dapp/deploy_wasm"
    Write-Host "Calling endpoint: $uri"
    
    $response = Invoke-RestMethod -Uri $uri -Method Post -Headers $headers -Body $bodyBytes
    
    Write-Host "Deployment Result:"
    $response | ConvertTo-Json -Depth 10 | Write-Host
    
    # Save the response to a file for reference
    $response | ConvertTo-Json -Depth 10 | Out-File -FilePath "deployment_result.json"
    Write-Host "Response saved to deployment_result.json"
    
} catch {
    Write-Host "Error during deployment:"
    Write-Host "Status Code: $($_.Exception.Response.StatusCode.value__)"
    Write-Host "Status Description: $($_.Exception.Response.StatusDescription)"
    
    if ($_.ErrorDetails) {
        Write-Host "Error Details: $($_.ErrorDetails.Message)"
    } else {
        Write-Host "Exception: $($_.Exception.Message)"
    }
    
    # Try to read the response stream for more details
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $reader.BaseStream.Position = 0
        $reader.DiscardBufferedData()
        $responseBody = $reader.ReadToEnd()
        Write-Host "Response Body: $responseBody"
    }
    
    exit 1
}

Write-Host "Deployment script completed successfully!"
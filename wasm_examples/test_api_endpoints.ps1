# PowerShell script to test API endpoints

Write-Host "Testing Owami Network API endpoints..."

# Test basic health check
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8081/api/dapp/deploy" -Method Post -Body '{"contract_path":"test.wasm"}' -ContentType "application/json"
    Write-Host "✓ Deploy endpoint is working"
    Write-Host "Response: $($response | ConvertTo-Json -Depth 10)"
} catch {
    Write-Host "✗ Deploy endpoint failed: $_"
}

# Test call endpoint
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8081/api/dapp/call" -Method Post -Body '{"contract_address":"0x123","function_name":"test"}' -ContentType "application/json"
    Write-Host "✓ Call endpoint is working"
    Write-Host "Response: $response"
} catch {
    Write-Host "✗ Call endpoint failed: $_"
}

# Test WASM deployment endpoint with a simple test
try {
    # Create a simple test file
    $testContent = [System.Text.Encoding]::UTF8.GetBytes("test wasm content")
    [System.IO.File]::WriteAllBytes("test.wasm", $testContent)
    
    $boundary = [System.Guid]::NewGuid().ToString()
    $headers = @{
        "Content-Type" = "multipart/form-data; boundary=`"$boundary`""
    }
    
    $body = "--$boundary`r`n"
    $body += "Content-Disposition: form-data; name=`"file`"; filename=`"test.wasm`"`r`n"
    $body += "Content-Type: application/wasm`r`n`r`n"
    $body += "test wasm content`r`n"
    $body += "--$boundary--`r`n"
    
    $response = Invoke-RestMethod -Uri "http://localhost:8081/api/dapp/deploy_wasm" -Method Post -Headers $headers -Body ([System.Text.Encoding]::UTF8.GetBytes($body))
    Write-Host "✓ WASM deploy endpoint is working"
    Write-Host "Response: $($response | ConvertTo-Json -Depth 10)"
    
    # Clean up
    Remove-Item "test.wasm" -Force
} catch {
    Write-Host "✗ WASM deploy endpoint failed: $_"
}

Write-Host "API endpoint testing completed!"
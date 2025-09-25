<#
Quick run for developers
- Optional: set JWT_SECRET if missing
- Runs cargo in release or debug based on flag
#>
param(
    [switch]$Debug
)

if (-not $env:JWT_SECRET) {
    $env:JWT_SECRET = "dev-secret-change-me"
    Write-Host "[Owami] Set default JWT_SECRET for dev." -ForegroundColor Yellow
}

if ($Debug) {
    Write-Host "[Owami] Running cargo run (debug)..." -ForegroundColor Cyan
    cargo run
} else {
    Write-Host "[Owami] Running cargo run --release..." -ForegroundColor Cyan
    cargo run --release
}
# Builds a release zip: dist/CodeForge-<version>-windows.zip
# Usage: powershell -ExecutionPolicy Bypass -File scripts/package.ps1
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$version = (Select-String -Path Cargo.toml -Pattern '^version\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value
Write-Host "Code Forge $version" -ForegroundColor Yellow

Write-Host "1/3 Running tests..."
cargo test --quiet
if ($LASTEXITCODE -ne 0) { throw "Tests failed - not packaging." }

Write-Host "2/3 Building release..."
cargo build --release --quiet
if ($LASTEXITCODE -ne 0) { throw "Release build failed." }

Write-Host "3/3 Creating zip..."
$stage = Join-Path $root "dist\CodeForge-$version"
if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
New-Item -ItemType Directory -Force $stage | Out-Null
$target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "target" }
Copy-Item (Join-Path $target "release\code_forge.exe")(Join-Path $stage "CodeForge.exe")
Copy-Item README.md, CREDITS.md $stage
$zip = Join-Path $root "dist\CodeForge-$version-windows.zip"
if (Test-Path $zip) { Remove-Item -Force $zip }
Compress-Archive -Path "$stage\*" -DestinationPath $zip
$size = [math]::Round((Get-Item $zip).Length / 1MB, 2)
Write-Host "Done: $zip ($size MB)" -ForegroundColor Green

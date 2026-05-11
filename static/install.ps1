# markdown2pdf installer for Windows.
#
#   powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
#
# Optional environment variables (set before piping to iex):
#   $env:M2P_INSTALL_DIR  Install directory       (default: $env:LOCALAPPDATA\Programs\markdown2pdf)
#   $env:M2P_VERSION      Release tag to install  (default: latest)
#   $env:M2P_NO_PATH      Set to 1 to skip adding the install dir to user PATH
#
# Example with custom dir:
#   $env:M2P_INSTALL_DIR = "$HOME\.local\bin"; irm https://markdown2pdf.eu/install.ps1 | iex
#
# The script:
#   1. detects your OS/arch
#   2. downloads from https://github.com/woodyjon/markdown2pdf/releases/<tag>/download/markdown2pdf-x86_64-pc-windows-msvc.zip
#   3. verifies the SHA256 against the SHA256SUMS asset
#   4. extracts markdown2pdf.exe into the install dir
#   5. adds the install dir to your user PATH (unless $env:M2P_NO_PATH is set)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$Repo       = 'woodyjon/markdown2pdf'
$InstallDir = if ($env:M2P_INSTALL_DIR) { $env:M2P_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'Programs\markdown2pdf' }
$Release    = if ($env:M2P_VERSION)     { $env:M2P_VERSION }     else { 'latest' }
$NoPath     = $env:M2P_NO_PATH -eq '1'

# Detect arch — only x86_64 Windows is published today
$arch = if ([Environment]::Is64BitOperatingSystem) {
    if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'aarch64' } else { 'x86_64' }
} else { 'x86' }

if ($arch -ne 'x86_64') {
    Write-Error @"
No prebuilt binary for Windows-$arch yet.
Options:
  1. Browse releases:   https://github.com/$Repo/releases/latest
  2. Build from source: cargo install --git https://github.com/$Repo markdown2pdf-cli
"@
    return
}

$target  = 'x86_64-pc-windows-msvc'
$asset   = "markdown2pdf-$target.zip"
$baseUrl = if ($Release -eq 'latest') {
    "https://github.com/$Repo/releases/latest/download"
} else {
    "https://github.com/$Repo/releases/download/$Release"
}
$url     = "$baseUrl/$asset"
$sumsUrl = "$baseUrl/SHA256SUMS"

# Stage in a fresh temp dir
$tmp = Join-Path $env:TEMP "m2p-install-$([guid]::NewGuid().ToString('N'))"
New-Item -ItemType Directory -Path $tmp | Out-Null

try {
    $zipPath = Join-Path $tmp $asset
    Write-Host "Downloading $url"
    Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing

    # Verify SHA256 against published SHA256SUMS (fail-closed: any error aborts the install).
    $sumsPath = Join-Path $tmp 'SHA256SUMS'
    Write-Host 'Fetching SHA256SUMS'
    Invoke-WebRequest -Uri $sumsUrl -OutFile $sumsPath -UseBasicParsing -ErrorAction Stop
    $line = Get-Content $sumsPath | Where-Object { $_ -match " $([regex]::Escape($asset))$" } | Select-Object -First 1
    if (-not $line) { throw "Aborting install: no entry for $asset in SHA256SUMS." }
    $expected = ($line -split '\s+')[0].ToLower()
    $actual   = (Get-FileHash $zipPath -Algorithm SHA256).Hash.ToLower()
    if ($expected -ne $actual) {
        throw "Aborting install: SHA256 mismatch for ${asset}: expected $expected, got $actual."
    }
    Write-Host 'SHA256 verified.'

    Write-Host 'Extracting'
    Expand-Archive -Path $zipPath -DestinationPath $tmp -Force

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    $exeSrc = Join-Path $tmp 'markdown2pdf.exe'
    $exeDst = Join-Path $InstallDir 'markdown2pdf.exe'
    Move-Item -Path $exeSrc -Destination $exeDst -Force

    Write-Host ''
    Write-Host "Installed $exeDst"
    & $exeDst --version

    if (-not $NoPath) {
        $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
        $entries = if ($userPath) { $userPath -split ';' } else { @() }
        if ($entries -notcontains $InstallDir) {
            $newPath = if ($userPath) { "$userPath;$InstallDir" } else { $InstallDir }
            [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
            Write-Host ''
            Write-Host "Added $InstallDir to your user PATH."
            Write-Host '(open a new PowerShell window for the change to take effect)'
        }
    }
} finally {
    Remove-Item -Path $tmp -Recurse -Force -ErrorAction SilentlyContinue
}

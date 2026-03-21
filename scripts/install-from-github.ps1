param(
    [string]$Version = "latest",
    [string]$Repo = "thecharge/sndv-scalpel",
    [string]$InstallDir = "$env:USERPROFILE\\bin"
)

$ErrorActionPreference = "Stop"

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
$platform = "windows"

if ($Version -eq "latest") {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $release.tag_name.TrimStart("v")
}

$asset = "scalpel-$Version-$platform-$arch.zip"
$url = "https://github.com/$Repo/releases/download/v$Version/$asset"

New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
$tmp = Join-Path $env:TEMP "scalpel-install"
Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $tmp -Force | Out-Null

$archive = Join-Path $tmp $asset
Invoke-WebRequest -Uri $url -OutFile $archive
Expand-Archive -Path $archive -DestinationPath $tmp -Force

$exe = Get-ChildItem -Path $tmp -Filter "scalpel.exe" -Recurse | Select-Object -First 1
if (-not $exe) {
    throw "scalpel.exe not found in archive"
}

Copy-Item $exe.FullName (Join-Path $InstallDir "scalpel.exe") -Force
Write-Host "Installed scalpel.exe to $InstallDir"
Write-Host "Add $InstallDir to PATH if not already present."

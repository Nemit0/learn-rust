Param(
    [string[]]$Packages
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Discover-PackagesUnderLectures {
    $metadataJson = (& cargo metadata --format-version 1 --no-deps | Out-String)
    $meta = $metadataJson | ConvertFrom-Json
    # Filter packages whose manifest path lives under lectures/
    $meta.packages |
        Where-Object { $_.manifest_path -match "(\\|/)lectures(\\|/)" } |
        ForEach-Object { $_.name }
}

if (-not $Packages -or $Packages.Count -eq 0) {
    Write-Host "Discovering lecture packages..." -ForegroundColor Cyan
    $Packages = @(Discover-PackagesUnderLectures)
}

if (-not $Packages -or $Packages.Count -eq 0) {
    Write-Host "No lecture packages found under 'lectures/'." -ForegroundColor Yellow
    exit 1
}

$results = @()
foreach ($pkg in $Packages) {
    Write-Host "Testing $pkg" -ForegroundColor Yellow
    & cargo test -p $pkg --quiet
    $passed = ($LASTEXITCODE -eq 0)
    $results += [PSCustomObject]@{ package = $pkg; passed = $passed }
}

$total = $results.Count
$ok = ($results | Where-Object { $_.passed }).Count
Write-Host ""; Write-Host "Summary: $ok/$total sections passed" -ForegroundColor Cyan

if ($ok -ne $total) {
    Write-Host "Failed sections:" -ForegroundColor Red
    $results | Where-Object { -not $_.passed } | ForEach-Object { Write-Host " - $($_.package)" -ForegroundColor Red }
    exit 1
}

exit 0


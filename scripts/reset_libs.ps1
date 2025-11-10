Param(
    [switch]$Init,
    [string]$GitRef,
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Resolve important paths relative to this script
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptRoot '..')
$lecturesRoot = Join-Path $repoRoot 'lectures'
$baselineRoot = Join-Path $scriptRoot 'baseline'

function Write-Info($msg)  { Write-Host $msg -ForegroundColor Cyan }
function Write-Act($msg)   { Write-Host $msg -ForegroundColor Yellow }
function Write-Warn($msg)  { Write-Host $msg -ForegroundColor DarkYellow }
function Write-ErrMsg($msg){ Write-Host $msg -ForegroundColor Red }

function Ensure-LecturesExists {
    if (-not (Test-Path $lecturesRoot)) {
        Write-ErrMsg "Lectures folder not found at '$lecturesRoot'"
        exit 1
    }
}

function Get-RelativePath([string]$basePath, [string]$targetPath) {
    # PowerShell 5 / .NET Framework lacks [System.IO.Path]::GetRelativePath
    try {
        $baseFull = [System.IO.Path]::GetFullPath($basePath)
        $targetFull = [System.IO.Path]::GetFullPath($targetPath)
        $sep = [System.IO.Path]::DirectorySeparatorChar
        if (-not ($baseFull.EndsWith($sep))) { $baseFull = $baseFull + $sep }
        $baseUri = [System.Uri]::new($baseFull)
        $targetUri = [System.Uri]::new($targetFull)
        $relUri = $baseUri.MakeRelativeUri($targetUri)
        $rel = [System.Uri]::UnescapeDataString($relUri.ToString())
        return ($rel -replace '/', [System.IO.Path]::DirectorySeparatorChar)
    } catch {
        # Fallback: if anything goes wrong, return the input path
        return $targetPath
    }
}

function Ensure-Directory($path) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path | Out-Null
    }
}

function In-GitRepo {
    $git = (Get-Command git -ErrorAction SilentlyContinue)
    if (-not $git) { return $false }
    $pinfo = New-Object System.Diagnostics.ProcessStartInfo
    $pinfo.FileName = $git.Source
    $pinfo.ArgumentList = @('-C', $repoRoot, 'rev-parse', '--is-inside-work-tree')
    $pinfo.RedirectStandardOutput = $true
    $pinfo.RedirectStandardError = $true
    $pinfo.UseShellExecute = $false
    $p = [System.Diagnostics.Process]::Start($pinfo)
    $null = $p.WaitForExit()
    return ($p.ExitCode -eq 0)
}

function Git-RestoreFile($relPath, $ref) {
    $git = (Get-Command git -ErrorAction Stop).Source
    $argsRestore = @('-C', $repoRoot, 'restore', '--worktree', '--source', $ref, '--', $relPath)
    $pinfo = New-Object System.Diagnostics.ProcessStartInfo
    $pinfo.FileName = $git
    $pinfo.ArgumentList = $argsRestore
    $pinfo.RedirectStandardOutput = $true
    $pinfo.RedirectStandardError = $true
    $pinfo.UseShellExecute = $false
    $p = [System.Diagnostics.Process]::Start($pinfo)
    $out = $p.StandardOutput.ReadToEnd()
    $err = $p.StandardError.ReadToEnd()
    $null = $p.WaitForExit()
    if ($p.ExitCode -ne 0) {
        # Fallback to older checkout syntax
        $argsCheckout = @('-C', $repoRoot, 'checkout', $ref, '--', $relPath)
        $pinfo2 = New-Object System.Diagnostics.ProcessStartInfo
        $pinfo2.FileName = $git
        $pinfo2.ArgumentList = $argsCheckout
        $pinfo2.RedirectStandardOutput = $true
        $pinfo2.RedirectStandardError = $true
        $pinfo2.UseShellExecute = $false
        $p2 = [System.Diagnostics.Process]::Start($pinfo2)
        $null = $p2.WaitForExit()
        return ($p2.ExitCode -eq 0)
    }
    return $true
}

Write-Info "Reset lectures utility"
Write-Info ("Repo root: {0}" -f $repoRoot)

Ensure-LecturesExists

if ($Init) {
    Write-Info "Initializing baseline snapshot of entire 'lectures/' tree..."
    $destLectures = Join-Path $baselineRoot 'lectures'
    Ensure-Directory $destLectures
    if ($DryRun) {
        Write-Act "Would snapshot: lectures/ -> scripts/baseline/lectures/"
    } else {
        if (Test-Path $destLectures) { $null = Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $destLectures }
        Ensure-Directory $destLectures
        Copy-Item -Recurse -Force -Path (Join-Path $lecturesRoot '*') -Destination $destLectures
        Write-Act "Snapshotted: lectures/"
    }
    Write-Info "Baseline initialized in scripts/baseline/lectures."
    exit 0
}

$usedGit = $false
if ($GitRef) {
    if (In-GitRepo) {
        Write-Info ("Restoring entire 'lectures/' from git ref: {0}" -f $GitRef)
        if ($DryRun) {
            Write-Act ("Would git-restore lectures/ from {0}" -f $GitRef)
        } else {
            $git = (Get-Command git -ErrorAction Stop).Source
            $p = Start-Process -FilePath $git -ArgumentList @('-C', $repoRoot, 'restore', '--worktree', '--source', $GitRef, '--', 'lectures') -PassThru -NoNewWindow -Wait
            if ($p.ExitCode -ne 0) {
                & $git -C $repoRoot checkout $GitRef -- lectures | Out-Null
            }
        }
        $usedGit = $true
    } else {
        Write-Warn "Not in a git repository; cannot use -GitRef. Falling back to baseline if available."
    }
}

if (-not $usedGit) {
    $baselineLectures = Join-Path $baselineRoot 'lectures'
    if (-not (Test-Path $baselineLectures)) {
        Write-ErrMsg "No baseline found at scripts/baseline/lectures and no -GitRef provided."
        Write-ErrMsg "Run:  ./scripts/reset_libs.ps1 -Init   to snapshot the current starter state."
        exit 1
    }
    Write-Info "Restoring entire 'lectures/' from local baseline snapshot..."
    if ($DryRun) {
        Write-Act "Would restore: lectures/ <- baseline"
    } else {
        Copy-Item -Recurse -Force -Path (Join-Path $baselineLectures '*') -Destination $lecturesRoot
        Write-Act "Restored: lectures/"
    }
}

Write-Info "Done."

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

function Get-LibFiles {
    if (-not (Test-Path $lecturesRoot)) {
        Write-ErrMsg "Lectures folder not found at '$lecturesRoot'"
        exit 1
    }
    Get-ChildItem -Path $lecturesRoot -Recurse -File -Filter 'lib.rs' |
        Where-Object { $_.FullName -match "(\\|/)src(\\|/)lib\.rs$" }
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

Write-Info "Reset libs utility"
Write-Info ("Repo root: {0}" -f $repoRoot)

$files = @(Get-LibFiles)
if ($files.Count -eq 0) {
    Write-Warn "No lib.rs files found under lectures/"
    exit 0
}

if ($Init) {
    Write-Info "Initializing baseline snapshot from current files..."
    foreach ($f in $files) {
        $rel = Get-RelativePath -basePath $repoRoot -targetPath $f.FullName
        $dest = Join-Path $baselineRoot $rel
        Ensure-Directory (Split-Path -Parent $dest)
        if ($DryRun) {
            Write-Act ("Would snapshot: {0} -> {1}" -f $rel, ([System.IO.Path]::GetRelativePath($repoRoot, $dest)))
        } else {
            Copy-Item -LiteralPath $f.FullName -Destination $dest -Force
            Write-Act ("Snapshotted: {0}" -f $rel)
        }
    }
    Write-Info "Baseline initialized in scripts/baseline."
    exit 0
}

$usedGit = $false
if ($GitRef) {
    if (In-GitRepo) {
        Write-Info ("Restoring from git ref: {0}" -f $GitRef)
        foreach ($f in $files) {
            $rel = Get-RelativePath -basePath $repoRoot -targetPath $f.FullName
            if ($DryRun) {
                Write-Act ("Would git-restore {0} from {1}" -f $rel, $GitRef)
            } else {
                $ok = Git-RestoreFile -relPath $rel -ref $GitRef
                if (-not $ok) { Write-ErrMsg ("Failed to restore {0} from {1}" -f $rel, $GitRef) }
            }
        }
        $usedGit = $true
    } else {
        Write-Warn "Not in a git repository; cannot use -GitRef. Falling back to baseline if available."
    }
}

if (-not $usedGit) {
    if (-not (Test-Path $baselineRoot)) {
        Write-ErrMsg "No baseline found at scripts/baseline and no -GitRef provided."
        Write-ErrMsg "Run:  ./scripts/reset_libs.ps1 -Init   to snapshot the current starter state."
        exit 1
    }
    Write-Info "Restoring from local baseline snapshot..."
    foreach ($f in $files) {
        $rel = Get-RelativePath -basePath $repoRoot -targetPath $f.FullName
        $src = Join-Path $baselineRoot $rel
        if (-not (Test-Path $src)) {
            Write-Warn ("Missing baseline for {0}; skipping" -f $rel)
            continue
        }
        if ($DryRun) {
            Write-Act ("Would restore: {0} <- baseline" -f $rel)
        } else {
            Copy-Item -LiteralPath $src -Destination $f.FullName -Force
            Write-Act ("Restored: {0}" -f $rel)
        }
    }
}

Write-Info "Done."

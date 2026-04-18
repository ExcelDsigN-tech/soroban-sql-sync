$ErrorActionPreference = "Stop"
$repo = "ExcelDsigN-tech/soroban-sql-sync"

function Invoke-GhRetry([string[]]$Args) {
  for ($i = 1; $i -le 4; $i++) {
    & gh @Args *> $null
    if ($LASTEXITCODE -eq 0) { return $true }
    Start-Sleep -Seconds (2 * $i)
  }
  return $false
}

$labels = @(
  @{n="stellar-wave"; c="7B2FBE"; d="Stellar Wave project"},
  @{n="type:backend"; c="0075ca"; d="Backend / Rust service"},
  @{n="type:frontend"; c="e4e669"; d="Frontend / React UI"},
  @{n="type:database"; c="c2e0c6"; d="Database schema or queries"},
  @{n="type:migration"; c="bfd4f2"; d="SQL migration scripts"},
  @{n="type:api"; c="d93f0b"; d="REST API endpoints"},
  @{n="type:blockchain-integration"; c="f9d0c4"; d="Soroban / Stellar RPC integration"},
  @{n="type:data-processing"; c="fef2c0"; d="XDR decode / transform logic"},
  @{n="type:ingestion"; c="0e8a16"; d="Event ingestion pipeline"},
  @{n="type:ui"; c="e99695"; d="UI components and layout"},
  @{n="type:dashboard"; c="fbca04"; d="Dashboard views"},
  @{n="type:smart-contract"; c="5319e7"; d="Soroban smart contract code"},
  @{n="type:blockchain"; c="1d76db"; d="On-chain logic"},
  @{n="type:spec"; c="cccccc"; d="Specification / documentation"},
  @{n="type:platform"; c="006b75"; d="Dev tooling / project setup"},
  @{n="type:devex"; c="c5def5"; d="Developer experience"},
  @{n="phase:foundation"; c="0052cc"; d="Phase 1 - Foundation"},
  @{n="phase:ingestion"; c="e11d48"; d="Phase 2 - Ingestion"},
  @{n="phase:visibility"; c="16a34a"; d="Phase 3 - Visibility"},
  @{n="priority:p0"; c="b60205"; d="Critical / blocking"},
  @{n="priority:p1"; c="e4e669"; d="High priority"},
  @{n="priority:p2"; c="0e8a16"; d="Normal priority"}
)

foreach ($l in $labels) {
  [void](Invoke-GhRetry @("label","create",$l.n,"--repo",$repo,"--color",$l.c,"--description",$l.d,"--force"))
}

$templates = Get-ChildItem ".github/ISSUE_TEMPLATE/*.md" | Sort-Object Name
$created = 0
$skipped = 0

foreach ($t in $templates) {
  $raw = Get-Content -Raw $t.FullName
  if ($raw -notmatch '(?s)^---\r?\n(.*?)\r?\n---\r?\n(.*)$') { continue }
  $fm = $matches[1]
  $body = $matches[2]

  $title = ""
  $labelsLine = ""
  if ($fm -match '(?m)^title:\s*"(.*)"\s*$') { $title = $matches[1] }
  elseif ($fm -match '(?m)^title:\s*(.*)\s*$') { $title = $matches[1].Trim('"') }
  if ($fm -match '(?m)^labels:\s*(.*)\s*$') { $labelsLine = $matches[1].Trim() }
  if ([string]::IsNullOrWhiteSpace($title)) { continue }

  $existingJson = gh issue list --repo $repo --search ('"' + $title + '" in:title') --limit 20 --json title,number 2>$null
  $exists = $false
  if ($LASTEXITCODE -eq 0 -and $existingJson) {
    $arr = $existingJson | ConvertFrom-Json
    foreach ($it in $arr) {
      if ($it.title -eq $title) { $exists = $true; break }
    }
  }

  if ($exists) {
    $skipped++
    continue
  }

  if ([string]::IsNullOrWhiteSpace($labelsLine)) {
    gh issue create --repo $repo --title $title --body $body *> $null
  } else {
    gh issue create --repo $repo --title $title --label $labelsLine --body $body *> $null
  }

  if ($LASTEXITCODE -eq 0) { $created++ }
}

Write-Output ("CREATED=" + $created + " SKIPPED=" + $skipped)
gh issue list --repo $repo --limit 30
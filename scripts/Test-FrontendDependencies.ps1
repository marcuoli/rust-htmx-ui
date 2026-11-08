param([string]$ProjectStorage='V:\Development\rust-htmx-ui')
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$scratch=Join-Path $ProjectStorage 'tmp\frontend-version-tests'
$run=Join-Path $ProjectStorage 'artifacts\runs\frontend-version-tests'
New-Item -ItemType Directory -Force -Path $scratch,$run|Out-Null
$checker=Join-Path $PSScriptRoot 'Check-FrontendDependencies.ps1'
& $checker -Manifest (Join-Path $root 'package.json') -Output (Join-Path $run 'current.json')
$result=Get-Content (Join-Path $run 'current.json') -Raw|ConvertFrom-Json
if(@($result.Packages).Count -lt 6){throw 'Current-version check reported an empty/incomplete graph'}
$package=Get-Content (Join-Path $root 'package.json') -Raw|ConvertFrom-Json
$package.dependencies.'htmx.org'='0.0.0'
$stale=Join-Path $scratch 'stale.json'
$package|ConvertTo-Json -Depth 6|Set-Content $stale
try{
 & $checker -Manifest $stale -Output (Join-Path $run 'stale.json')
 throw 'Stale frontend dependency was accepted'
}catch{
 if($_.Exception.Message -notlike 'Frontend pins differ*'){throw}
}
$incomplete=Join-Path $scratch 'incomplete.json'
'{"dependencies":{},"devDependencies":{}}'|Set-Content $incomplete
try{
 & $checker -Manifest $incomplete -Output (Join-Path $run 'incomplete.json')
 throw 'Empty frontend dependency graph was accepted'
}catch{
 if($_.Exception.Message -ne 'Required shared frontend dependency is missing'){throw}
}
'Frontend gate regression tests passed: current graph covered; stale and empty graphs rejected.'

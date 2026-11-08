param([string]$ProjectStorage='V:\Development\rust-htmx-ui')
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$scratch=Join-Path $ProjectStorage 'tmp\issue-1'
$stage=Join-Path $scratch ('styles-'+[DateTime]::UtcNow.ToString('yyyyMMddTHHmmssfffZ'))
$logs=Join-Path $ProjectStorage 'artifacts\runs\issue-1\styles'
$changes=@{TEMP=$scratch;TMP=$scratch;TMPDIR=$scratch;npm_config_cache=(Join-Path $ProjectStorage 'caches\npm');PATH=('V:\Development\tools\node\24.22.1;'+$env:PATH)}
$saved=@{}
New-Item -ItemType Directory -Force -Path $stage,$logs|Out-Null
try{
 foreach($key in $changes.Keys){$saved[$key]=[Environment]::GetEnvironmentVariable($key,'Process');[Environment]::SetEnvironmentVariable($key,$changes[$key],'Process')}
 $manifest=Get-Content (Join-Path $root 'package.json') -Raw|ConvertFrom-Json
 foreach($name in @('tailwindcss','@tailwindcss/cli')){
  $info=Invoke-RestMethod ('https://registry.npmjs.org/'+[Uri]::EscapeDataString($name)+'/latest')
  if($manifest.devDependencies.$name -ne $info.version){throw 'Update and qualify latest stable Tailwind pins before building shared CSS'}
 }
 Copy-Item -LiteralPath (Join-Path $root 'package.json'),(Join-Path $root 'package-lock.json') -Destination $stage
 Copy-Item -LiteralPath (Join-Path $root 'styles') -Destination (Join-Path $stage 'styles') -Recurse
 Copy-Item -LiteralPath (Join-Path $root 'scripts') -Destination (Join-Path $stage 'scripts') -Recurse
 Push-Location $stage
 try{
  & npm.cmd ci --ignore-scripts --no-audit --no-fund *> (Join-Path $logs 'npm-ci.log')
  if($LASTEXITCODE){throw 'Style dependencies failed'}
  & npm.cmd audit --json *> (Join-Path $logs 'npm-audit.json')
  if($LASTEXITCODE){throw 'Style dependency advisories require review'}
  $output=Join-Path $logs 'ui.css'
  & node.exe scripts/build-styles.mjs (Join-Path $stage 'node_modules') $output *> (Join-Path $logs 'build.log')
  if($LASTEXITCODE){throw 'Shared style build failed'}
  Copy-Item -LiteralPath $output -Destination (Join-Path $root 'assets\ui.css') -Force
  @{checked_utc=[DateTime]::UtcNow.ToString('o');tailwind=$manifest.devDependencies.tailwindcss;sha256=(Get-FileHash -LiteralPath $output -Algorithm SHA256).Hash.ToLowerInvariant()}|ConvertTo-Json|Set-Content (Join-Path $logs 'verification.json')
 }finally{Pop-Location}
}finally{foreach($key in $saved.Keys){[Environment]::SetEnvironmentVariable($key,$saved[$key],'Process')}}

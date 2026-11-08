param(
 [string]$ProjectStorage='V:\Development\rust-htmx-ui',
 [ValidatePattern('^[a-zA-Z0-9_-]+$')][string]$TaskName='frontend'
)
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$scratch=Join-Path $ProjectStorage ('tmp\'+$TaskName)
$stage=Join-Path $scratch ('assets-'+[DateTime]::UtcNow.ToString('yyyyMMddTHHmmssfffZ'))
$logs=Join-Path $ProjectStorage ('artifacts\runs\'+$TaskName+'\frontend')
$changes=@{TEMP=$scratch;TMP=$scratch;TMPDIR=$scratch;npm_config_cache=(Join-Path $ProjectStorage 'caches\npm');npm_config_update_notifier='false';PATH=('V:\Development\tools\node\24.22.1;'+$env:PATH)}
$saved=@{}
New-Item -ItemType Directory -Force -Path $stage,$logs,(Join-Path $logs 'assets')|Out-Null
try{
 foreach($key in $changes.Keys){$saved[$key]=[Environment]::GetEnvironmentVariable($key,'Process');[Environment]::SetEnvironmentVariable($key,$changes[$key],'Process')}
 & (Join-Path $PSScriptRoot 'Check-FrontendDependencies.ps1') -Manifest (Join-Path $root 'package.json') -Output (Join-Path $logs 'versions.json')
 foreach($file in @('package.json','package-lock.json')){Copy-Item -LiteralPath (Join-Path $root $file) -Destination $stage}
 foreach($dir in @('styles','scripts','runtime','src','examples','licenses')){Copy-Item -LiteralPath (Join-Path $root $dir) -Destination (Join-Path $stage $dir) -Recurse}
 Push-Location $stage
 try{
  & npm.cmd ci --ignore-scripts --no-audit --no-fund *> (Join-Path $logs 'npm-ci.log')
  if($LASTEXITCODE){throw 'Locked frontend dependency installation failed'}
  & npm.cmd audit --json *> (Join-Path $logs 'npm-audit.json')
  if($LASTEXITCODE){throw 'Frontend advisories require review'}
  $output=Join-Path $logs 'assets'
  & node.exe scripts/build-styles.mjs (Join-Path $stage 'node_modules') (Join-Path $output 'ui.css') *> (Join-Path $logs 'styles.log')
  if($LASTEXITCODE){throw 'Component stylesheet build failed'}
  & node.exe node_modules/@tailwindcss/cli/dist/index.mjs -i styles/frontend.css -o (Join-Path $output 'frontend.css') --minify *> (Join-Path $logs 'tailwind.log')
  if($LASTEXITCODE){throw 'Shared frontend stylesheet build failed'}
  & node.exe scripts/build-runtime.mjs $output *> (Join-Path $logs 'runtime.log')
  if($LASTEXITCODE){Get-Content (Join-Path $logs 'runtime.log') -Tail 15;throw 'Shared runtime build failed'}
  $hashes=@{}
  foreach($file in Get-ChildItem -LiteralPath $output -File){
   Copy-Item -LiteralPath $file.FullName -Destination (Join-Path $root ('assets\'+$file.Name)) -Force
   $hashes[$file.Name]=(Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
  }
  @{CheckedUtc=[DateTime]::UtcNow.ToString('o');Files=$hashes}|ConvertTo-Json -Depth 4|Set-Content (Join-Path $logs 'asset-hashes.json')
 }finally{Pop-Location}
}finally{foreach($key in $saved.Keys){[Environment]::SetEnvironmentVariable($key,$saved[$key],'Process')}}
'Shared frontend assets built and audited.'

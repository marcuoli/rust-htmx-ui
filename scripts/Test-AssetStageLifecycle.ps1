param(
 [string]$ProjectStorage='V:\Development\rust-htmx-ui',
 [ValidatePattern('^[a-zA-Z0-9_-]+$')][string]$TaskName='asset-stage-lifecycle-tests',
 [string]$Case='*'
)
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$fixture=Join-Path $ProjectStorage ('tmp\'+$TaskName+'\ps'+$PSVersionTable.PSVersion.Major+'-'+[Guid]::NewGuid().ToString('N'))
$run=Join-Path $ProjectStorage ('artifacts\runs\'+$TaskName)
New-Item -ItemType Directory -Path $fixture | Out-Null
New-Item -ItemType Directory -Force -Path $run | Out-Null
foreach($directory in @('scripts','styles','runtime','src','examples','licenses','assets')){
 New-Item -ItemType Directory -Path (Join-Path $fixture $directory) | Out-Null
 'fixture input' | Set-Content -LiteralPath (Join-Path $fixture ($directory+'\input.txt'))
}
foreach($file in @('Build-Assets.ps1','Check-FrontendDependencies.ps1')){
 Copy-Item -LiteralPath (Join-Path $PSScriptRoot $file) -Destination (Join-Path $fixture 'scripts')
}
'{"dependencies":{"htmx.org":"1.0.0","@alpinejs/csp":"1.0.0","echarts":"1.0.0"},"devDependencies":{"tailwindcss":"1.0.0","@tailwindcss/cli":"1.0.0","esbuild":"1.0.0"}}' | Set-Content -LiteralPath (Join-Path $fixture 'package.json')
'{"lockfileVersion":3,"packages":{}}' | Set-Content -LiteralPath (Join-Path $fixture 'package-lock.json')
$runId=[Guid]::NewGuid().ToString('N').Substring(0,8)
$mockState=@{Mode='';LockedFile=$null;Commands=@()}
function Invoke-RestMethod {
 if($mockState.Mode -eq 'version-failure'){return [pscustomobject]@{version='0.0.0'}}
 [pscustomobject]@{version='1.0.0'}
}
function npm.cmd {
 $mockState.Commands+=('npm '+($args -join ' '))
 $global:LASTEXITCODE=0
 if($args[0] -eq 'ci'){
  New-Item -ItemType Directory -Path 'node_modules' | Out-Null
  'installed fixture dependency' | Set-Content -LiteralPath 'node_modules\input.txt'
  if($mockState.Mode -eq 'install-failure'){$global:LASTEXITCODE=17}
  if($mockState.Mode -eq 'redirect'){
   New-Item -ItemType Junction -Path 'node_modules\outside' -Target (Join-Path $fixture 'protected') | Out-Null
  }
 }
 'fixture npm output'
}
function node.exe {
 $mockState.Commands+=('node '+($args -join ' '))
 $global:LASTEXITCODE=0
 if($mockState.Mode -eq 'build-failure'){$global:LASTEXITCODE=23;return}
 switch($args[0]){
  'scripts/build-styles.mjs' {'component css' | Set-Content -LiteralPath $args[2]}
  'node_modules/@tailwindcss/cli/dist/index.mjs' {'frontend css' | Set-Content -LiteralPath $args[4]}
  'scripts/build-runtime.mjs' {
   'runtime js' | Set-Content -LiteralPath (Join-Path $args[1] 'ui-core.js')
   if($mockState.Mode -eq 'cleanup-failure'){
    $mockState.LockedFile=[IO.File]::Open((Join-Path (Get-Location).Path 'locked.bin'),[IO.FileMode]::Create,[IO.FileAccess]::ReadWrite,[IO.FileShare]::None)
   }
  }
  default {throw 'Unexpected node invocation'}
 }
}
# Bind mocks to this harness state even when another .ps1 owns the script scope.
foreach($command in @('Invoke-RestMethod','npm.cmd','node.exe')){
 $mock=(Get-Item -LiteralPath ('Function:\'+$command)).ScriptBlock.GetNewClosure()
 Set-Item -LiteralPath ('Function:\'+$command) -Value $mock
}
New-Item -ItemType Directory -Path (Join-Path $fixture 'protected') | Out-Null
'protected external input' | Set-Content -LiteralPath (Join-Path $fixture 'protected\input.txt')
$protectedHash=(Get-FileHash -LiteralPath (Join-Path $fixture 'protected\input.txt')).Hash
$cases=@(
 @{Name='success';Error='';Commands=5},
 @{Name='version-failure';Error='Frontend pins differ*';Commands=0},
 @{Name='install-failure';Error='Locked frontend dependency installation failed';Commands=1},
 @{Name='build-failure';Error='Component stylesheet build failed';Commands=3},
 @{Name='cleanup-failure';Error='*';Commands=5},
 @{Name='redirect';Error='*redirect*';Commands=5}
)
$results=@()
foreach($test in @($cases | Where-Object {$_.Name -like $Case})){
 $mockState.Mode=$test.Name
 $mockState.Commands=@()
 $caseTask=$TaskName+'-ps'+$PSVersionTable.PSVersion.Major+'-'+$runId+'-'+$test.Name
 $scratch=Join-Path $ProjectStorage ('tmp\'+$caseTask)
 $logs=Join-Path $ProjectStorage ('artifacts\runs\'+$caseTask+'\frontend')
 $unrelated=Join-Path $scratch ('assets-unrelated-'+[Guid]::NewGuid().ToString('N'))
 New-Item -ItemType Directory -Path $unrelated | Out-Null
 'unrelated stage' | Set-Content -LiteralPath (Join-Path $unrelated 'keep.txt')
 $unrelatedHash=(Get-FileHash -LiteralPath (Join-Path $unrelated 'keep.txt')).Hash
 $locationBefore=(Get-Location).Path
 $environmentBefore=@{}
 foreach($name in @('TEMP','TMP','TMPDIR','npm_config_cache','npm_config_update_notifier','PATH')){
  $environmentBefore[$name]=[Environment]::GetEnvironmentVariable($name,'Process')
 }
 $errorText=$null
 $output=[Collections.Generic.List[object]]::new()
 try{
  try{
   & (Join-Path $fixture 'scripts\Build-Assets.ps1') -ProjectStorage $ProjectStorage -TaskName $caseTask 3>&1 | ForEach-Object {$output.Add($_)}
  }catch{$errorText=$_.Exception.Message}
  if($mockState.LockedFile){$mockState.LockedFile.Dispose();$mockState.LockedFile=$null}
  $stages=@(Get-ChildItem -LiteralPath $scratch -Directory | Where-Object {$_.FullName -ne $unrelated})
  if($mockState.Commands.Count -ne $test.Commands){throw 'Expected tool steps were not exercised'}
  if($test.Name -eq 'success'){
   if($errorText){throw ('Successful build failed: '+$errorText)}
   if($stages.Count){throw 'Successful build left its copied-source stage'}
  }else{
   if(-not $errorText -or $errorText -notlike $test.Error){throw ('Wrong failure: '+$errorText)}
   if($stages.Count -ne 1){throw 'Failed build did not retain its exact stage'}
   if(-not ($output | Where-Object {"$_" -like ('*'+$stages[0].FullName+'*')})){throw 'Failure did not report the retained stage path'}
  }
  if((Get-Location).Path -ne $locationBefore){throw 'Location was not restored'}
  foreach($name in $environmentBefore.Keys){
   if([Environment]::GetEnvironmentVariable($name,'Process') -cne $environmentBefore[$name]){throw ('Environment was not restored: '+$name)}
  }
  if((Get-FileHash -LiteralPath (Join-Path $unrelated 'keep.txt')).Hash -ne $unrelatedHash){throw 'Unrelated stage changed'}
  if((Get-FileHash -LiteralPath (Join-Path $fixture 'protected\input.txt')).Hash -ne $protectedHash){throw 'Redirect target changed'}
  if($test.Name -notin @('version-failure','install-failure','build-failure')){
   $hashes=Get-Content -LiteralPath (Join-Path $logs 'asset-hashes.json') -Raw | ConvertFrom-Json
   foreach($name in @('ui.css','frontend.css','ui-core.js')){
    $expected=(Get-FileHash -LiteralPath (Join-Path $logs ('assets\'+$name))).Hash.ToLowerInvariant()
    if($hashes.Files.$name -ne $expected -or (Get-FileHash -LiteralPath (Join-Path $fixture ('assets\'+$name))).Hash.ToLowerInvariant() -ne $expected){throw 'Retained/copied asset hash mismatch'}
   }
  }
  if($test.Name -ne 'version-failure' -and -not (Test-Path -LiteralPath (Join-Path $logs 'npm-ci.log'))){throw 'Build log was removed'}
  $results+=[pscustomobject]@{Case=$test.Name;Passed=$true;Stage=@($stages.FullName);Scratch=$scratch;Logs=$logs;Commands=$mockState.Commands}
 }catch{
  $results+=[pscustomobject]@{Case=$test.Name;Passed=$false;Error=$_.Exception.Message;Scratch=$scratch;Logs=$logs}
 }finally{
  if($mockState.LockedFile){$mockState.LockedFile.Dispose();$mockState.LockedFile=$null}
 }
}
$receipt=[ordered]@{CheckedUTC=[DateTime]::UtcNow.ToString('o');PowerShell=$PSVersionTable.PSVersion.ToString();Fixture=$fixture;BuilderSHA256=(Get-FileHash -LiteralPath (Join-Path $fixture 'scripts\Build-Assets.ps1')).Hash;Results=$results;Scope='Deterministic lifecycle checks with simulated registry/npm/node; no packages installed or frontend code executed. Fixtures retained for reviewed task closeout.'}
$receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $run ('lifecycle-ps'+$PSVersionTable.PSVersion.Major+'-'+[DateTime]::UtcNow.ToString('HHmmssfff')+'.json'))
$results | Select-Object Case,Passed,Error
if(-not $results.Count -or @($results | Where-Object {-not $_.Passed}).Count){throw 'Asset stage lifecycle regression failed'}

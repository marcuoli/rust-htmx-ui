param(
 [string]$Manifest=(Join-Path (Split-Path $PSScriptRoot -Parent) 'package.json'),
 [string]$Output='V:\Development\rust-htmx-ui\artifacts\runs\validation\frontend-versions.json'
)
$ErrorActionPreference='Stop'
# PowerShell variable names are case-insensitive; do not reuse the typed path parameter.
$packageManifest=Get-Content -LiteralPath $Manifest -Raw|ConvertFrom-Json
$declared=@{}
foreach($group in @($packageManifest.dependencies,$packageManifest.devDependencies)){
 if($null -eq $group){continue}
 foreach($property in $group.PSObject.Properties){
  if($declared.ContainsKey($property.Name)){throw 'Duplicate frontend dependency declaration'}
  $declared[$property.Name]=$property.Value
 }
}
foreach($required in @('tailwindcss','@tailwindcss/cli','htmx.org','@alpinejs/csp','echarts','esbuild')){
 if(-not $declared.ContainsKey($required)){throw 'Required shared frontend dependency is missing'}
}
$checks=@()
foreach($name in @($declared.Keys|Sort-Object)){
 $info=Invoke-RestMethod ('https://registry.npmjs.org/'+[Uri]::EscapeDataString($name)+'/latest')
 $checks+=[pscustomobject]@{Name=$name;Pinned=$declared[$name];Latest=$info.version;Current=($declared[$name] -eq $info.version)}
}
if($checks.Count -eq 0 -or $checks.Count -ne $declared.Count){throw 'Frontend version check did not cover the declared dependencies'}
New-Item -ItemType Directory -Force -Path (Split-Path $Output -Parent)|Out-Null
@{CheckedUtc=[DateTime]::UtcNow.ToString('o');Packages=$checks}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath $Output
if(@($checks|Where-Object {-not $_.Current}).Count){throw 'Frontend pins differ from current official stable releases; update and qualify before building'}
'Shared frontend pins match current stable releases.'

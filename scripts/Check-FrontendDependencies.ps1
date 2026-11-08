param(
 [string]$Manifest=(Join-Path (Split-Path $PSScriptRoot -Parent) 'package.json'),
 [string]$Output='V:\Development\rust-htmx-ui\artifacts\runs\validation\frontend-versions.json'
)
$ErrorActionPreference='Stop'
$manifest=Get-Content -LiteralPath $Manifest -Raw|ConvertFrom-Json
$checks=@()
foreach($group in @($manifest.dependencies,$manifest.devDependencies)){
 foreach($property in $group.PSObject.Properties){
  $info=Invoke-RestMethod ('https://registry.npmjs.org/'+[Uri]::EscapeDataString($property.Name)+'/latest')
  $checks+=[pscustomobject]@{Name=$property.Name;Pinned=$property.Value;Latest=$info.version;Current=($property.Value -eq $info.version)}
 }
}
New-Item -ItemType Directory -Force -Path (Split-Path $Output -Parent)|Out-Null
@{CheckedUtc=[DateTime]::UtcNow.ToString('o');Packages=$checks}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath $Output
if(@($checks|Where-Object {-not $_.Current}).Count){throw 'Frontend pins differ from current official stable releases; update and qualify before building'}
'Shared frontend pins match current stable releases.'

import {build} from 'esbuild';
import {readFileSync,writeFileSync,existsSync,copyFileSync} from 'node:fs';
import {join,resolve} from 'node:path';
import {createHash} from 'node:crypto';
const output=resolve(process.argv[2]);
const packageInfo=name=>JSON.parse(readFileSync(join('node_modules',name,'package.json'),'utf8'));
const manifest=JSON.parse(readFileSync('package.json','utf8'));
const versions={tailwind:packageInfo('tailwindcss').version,htmx:packageInfo('htmx.org').version,alpine:packageInfo('@alpinejs/csp').version,echarts:packageInfo('echarts').version};
for(const [name,version] of Object.entries({...manifest.dependencies,...manifest.devDependencies})){
 if(packageInfo(name).version!==version) throw new Error('Installed frontend dependency differs from its exact pin');
}
writeFileSync('runtime/versions.json',JSON.stringify(versions));
await build({entryPoints:['runtime/frameworks.js'],bundle:true,minify:true,format:'iife',target:'es2022',legalComments:'inline',outfile:join(output,'ui-core.js')});
writeFileSync(join(output,'frontend-versions.json'),JSON.stringify(versions,null,2)+'\n');
let notices='';
for(const name of ['tailwindcss','htmx.org','@alpinejs/csp','@vue/reactivity','@vue/shared']){
 const root=join('node_modules',name);
 let license=['LICENSE','LICENSE.txt','LICENSE.md'].map(file=>join(root,file)).find(existsSync);
 if(!license && name==='@alpinejs/csp'){
  const provenance=JSON.parse(readFileSync('licenses/alpinejs.provenance.json','utf8'));
  license='licenses/alpinejs.txt';
  if(provenance.version!==versions.alpine || createHash('sha256').update(readFileSync(license)).digest('hex')!==provenance.sha256)throw new Error('Alpine license provenance does not match the selected runtime');
 }
 if(!license)throw new Error('Missing official runtime license for '+name);
 notices+='\n'+name+' '+packageInfo(name).version+'\n'+readFileSync(license,'utf8')+'\n';
}
writeFileSync(join(output,'frontend-LICENSES.txt'),notices.trim()+'\n');
const chartFiles={
 'echarts.min.js':'echarts/dist/echarts.min.js','echarts.LICENSE.txt':'echarts/LICENSE','echarts.NOTICE.txt':'echarts/NOTICE',
 'echarts-d3.LICENSE.txt':'echarts/licenses/LICENSE-d3','zrender.LICENSE.txt':'zrender/LICENSE',
 'tslib.LICENSE.txt':'tslib/LICENSE.txt','tslib.CopyrightNotice.txt':'tslib/CopyrightNotice.txt',
};
const hashes={};
for(const [name,source] of Object.entries(chartFiles)){
 const original=join('node_modules',source);
 copyFileSync(original,join(output,name));
 hashes[name]=createHash('sha256').update(readFileSync(join(output,name))).digest('hex');
 if(hashes[name]!==createHash('sha256').update(readFileSync(original)).digest('hex'))throw new Error('Upstream asset copy changed bytes');
}
const locked=JSON.parse(readFileSync('package-lock.json','utf8')).packages['node_modules/echarts'];
writeFileSync(join(output,'echarts.provenance.json'),JSON.stringify({package:'echarts',version:versions.echarts,source:locked.resolved,integrity:locked.integrity,distribution:'dist/echarts.min.js',sha256:hashes},null,2)+'\n');

const lock=JSON.parse(readFileSync('package-lock.json','utf8'));
const dependencies={};
for(const name of Object.keys({...manifest.dependencies,...manifest.devDependencies})){
 const entry=lock.packages['node_modules/'+name];
 dependencies[name]={version:entry.version,source:entry.resolved,integrity:entry.integrity};
}
const built={};
for(const name of ['ui.css','frontend.css','ui-core.js','frontend-versions.json','frontend-LICENSES.txt',...Object.keys(chartFiles)]){
 built[name]=createHash('sha256').update(readFileSync(join(output,name))).digest('hex');
}
writeFileSync(join(output,'frontend.provenance.json'),JSON.stringify({dependencies,assets:built},null,2)+'\n');
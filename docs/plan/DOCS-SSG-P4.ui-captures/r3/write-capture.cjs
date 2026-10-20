const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root=process.cwd(),out=__dirname;
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const b=JSON.parse(fs.readFileSync(out+'/baseline.json'));
const sources=['AGENTS.md','README.md','PRODUCT.md','DESIGN.md','DESIGN.docs-ssg.md','.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md','docs/PRD-DOCS-SSG.md','docs/plan/DOCS-SSG-P4.md','docs/plan/DOCS-SSG-P4.ui-target.md'];
fs.writeFileSync(out+'/source-hashes.json',JSON.stringify(Object.fromEntries(sources.map(p=>[p,hash(fs.readFileSync(p))])),null,2));
const supplements=fs.readdirSync(out).filter(f=>/^U4-.*\.(png|jpg)$|^M.*\.png$/.test(f)).map(f=>({file:f,savedAtUTC:fs.statSync(out+'/'+f).mtime.toISOString(),sha256:hash(fs.readFileSync(out+'/'+f)),source:f.endsWith('.jpg')?'CUA native app':f.startsWith('U4')?'Kimi native Chrome':'post-judge Playwright runtime check'}));
fs.writeFileSync(out+'/supplement-captures.json',JSON.stringify(supplements,null,2));
const shared=`人工历史续接R3，全量复审；仅DOM静态阅读与搜索。

- HEAD ${b.head}，${b.branch}；初始tracked diff SHA256 ${b.dirtyDiffSha256}，不含未跟踪文件；完整状态见baseline.json。
- 固定实装CLI0.1.27，C ${b.candidate} / R ${b.framework}；相关产品相对C无diff。
- 目标沿用R2：${b.targetFingerprintAlgorithm}:${b.targetFingerprint}。目标全文hash也相同，未重建基线；延续人工历史算法，不冒称Phoenix协议指纹。本循环没有managed索引。
- 按serve.sh重新构建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath及meta 176bea4ed7e4d96f；启动日志保留原文。
- Playwright1.63.0 / Chromium${b.browser}，DPR1，1280×900/375×812/320×812；站内按钮设置light/dark，reduce motion，无CPU/网络节流（逐图故障干预除外），等待字体和两次rAF；禁脚本新建禁用JS上下文。
- 已有固定种子100页/语种语料和官方站，所有91张均本轮重拍，不是评审自制界面，未复制旧图；仅延迟/失败真实module或索引请求，逐图注明。没有目标图附件，仅规范模式。
`;
let all=[];
const head='|单元|状态|原图|URL|视口/主题|UTC|干预|\n|---|---|---|---|---|---|---|\n';
for(const u of ['U1','U2','U3']){
 const records=JSON.parse(fs.readFileSync(out+'/'+u+'-capture.json'));
 const rows=records.map(x=>{
  const buf=fs.readFileSync(out+'/'+x.file);
  if(hash(buf)!==x.sha256||buf.readUInt32BE(16)!==x.viewport.width||buf.readUInt32BE(20)!==x.viewport.height)throw new Error(x.file);
  return `|${u}|${x.state}|[${x.file}](${x.file})|${x.url}|${x.viewport.width}×${x.viewport.height} DPR1 ${x.theme}|${x.at}|${x.interception}|`;
 });
 all.push(...rows);
 fs.writeFileSync(out+'/'+u+'-capture.md',`# R3 ${u} 实拍清单\n\n${shared}\n实际操作与页面观测见[${u}-operations.md](${u}-operations.md)；逐图hash见${u}-capture.json。\n\n${head}${rows.join('\n')}\n`);
}
fs.writeFileSync(out+'/capture.md',`# DOCS-SSG-P4 R3 · 采集清单\n\n${shared}\n${head}${all.join('\n')}\n\n实际操作与页面观测见各U*-operations.md；逐图hash见各U*-capture.json。原生和事后检查补充见[capture-notes.md](capture-notes.md)与supplement-captures.json，不混入独立评分材料。\n`);
console.log(JSON.stringify({matrix:all.length,supplements:supplements.length,target:b.targetFingerprint,planHash:hash(fs.readFileSync('docs/plan/DOCS-SSG-P4.md'))}));

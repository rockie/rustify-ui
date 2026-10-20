const fs = require('node:fs');
const crypto = require('node:crypto');
const path = require('node:path');
const root = process.cwd();
const out = path.join(root, 'docs/plan/DOCS-SSG-P4.ui-captures/r2');
const hash = x => crypto.createHash('sha256').update(x).digest('hex');
const b = JSON.parse(fs.readFileSync(path.join(out, 'baseline.json')));
b.targetFileSha256 = hash(fs.readFileSync('docs/plan/DOCS-SSG-P4.ui-target.md'));
fs.writeFileSync(path.join(out, 'baseline.json'), JSON.stringify(b, null, 2));
for (const name of fs.readdirSync(out).filter(x => x.startsWith('U4-native-chrome') && x.endsWith('.png'))) fs.renameSync(path.join(out, name), path.join(out, name.replace('.png', '.jpg')));
const shared = `- 人工R2，全量复审，仅DOM静态阅读与搜索。HEAD ${b.head}，${b.branch}，dirty tracked diff SHA256 ${b.dirtyDiffSha256}；完整status与指纹定义见baseline.json。tracked diff不含未跟踪文件。
- 固定实装CLI0.1.27，C ${b.candidate} / R ${b.framework}；被测相关产品路径相对C无diff。
- 目标指纹 ${b.targetFingerprintAlgorithm}:${b.targetFingerprint}，与R1相同，无目标附件，仅规范模式。
- 本轮按serve.sh重建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath与meta 176bea4ed7e4d96f。真实构建身份日志在本目录。
- Playwright1.63.0 / Chromium${b.browser}，DPR1；1280×900/375×812/320×812；站内按钮设定light/dark，reduce motion；无CPU/网络节流（逐图故障干预除外）。等待字体、网络和两次rAF，禁脚本不等待rAF。窄屏仅CSS视口。
- 数据：项目已有固定种子100页/语种语料和官方站。只干预真实请求，未替换DOM、CSS或页面脚本，不是临时模拟界面。
`;
const head = '|单元|状态|截图|实际URL|视口/DPR|主题|时间UTC|干预|\n|---|---|---|---|---|---|---|---|---|\n';
const rows = [];
for (const u of ['U1', 'U2', 'U3']) {
  const captures = JSON.parse(fs.readFileSync(path.join(out, `${u}-capture.json`)));
  const sub = captures.map(x => {
    const data = fs.readFileSync(path.join(out, x.file));
    const w = data.readUInt32BE(16), h = data.readUInt32BE(20);
    if (w !== x.viewport.width || h !== x.viewport.height || hash(data) !== x.sha256) throw new Error(x.file);
    return `|${u}|${x.state}|[${x.file}](${x.file})|${x.url}|${w}×${h}/1|${x.theme}|${x.at}|${x.interception}|`;
  });
  rows.push(...sub);
  fs.writeFileSync(path.join(out, `${u}-capture.md`), `# R2 ${u} 采集清单\n\n${shared}\n实际操作与观察见[${u}-operations.md](${u}-operations.md)，图像hash见本单元capture.json。\n\n${head}${sub.join('\n')}\n`);
}
fs.writeFileSync(path.join(out, 'capture.md'), `# DOCS-SSG-P4 UI验收R2 · 采集清单\n\n${shared}\n${head}${rows.join('\n')}\n\n实际操作见各U*-operations.md。browser-raw.jsonl只在独立评审后分析。缺证与原生补充见[capture-notes.md](capture-notes.md)。\n`);
const prev = fs.readFileSync('docs/plan/DOCS-SSG-P4.ui-captures/r1/judge.md', 'utf8');
for (const [u, ids] of [['U1', ['UI-01','UI-02']], ['U2', ['UI-02','UI-04']], ['U3', ['UI-03']]]) {
  const score = prev.split('\n').filter(l => l.startsWith(`| ${u} `) || l.startsWith(`- ${u}：`)).join('\n');
  const gaps = prev.split('### ').slice(1).filter(p => ids.some(id => p.startsWith(id+' '))).map(p => '### '+p.split('\n## 建议')[0]).join('\n\n');
  fs.writeFileSync(path.join(out, `previous-${u}.md`), `# R1 ${u} 原结论逐字摘录\n\n${score}\n\n${gaps}\n`);
}
const sources = ['AGENTS.md','README.md','PRODUCT.md','DESIGN.md','DESIGN.docs-ssg.md','.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md','docs/PRD-DOCS-SSG.md','docs/plan/DOCS-SSG-P4.md','docs/plan/DOCS-SSG-P4.ui-target.md'];
fs.writeFileSync(path.join(out,'source-hashes.json'), JSON.stringify(Object.fromEntries(sources.map(p => [p, hash(fs.readFileSync(p))])), null, 2));
console.log(JSON.stringify({captures:rows.length, target:b.targetFingerprint}));

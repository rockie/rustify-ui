const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root=process.cwd(),out=__dirname;
const artifact='docs/plan/DOCS-SSG-P4.ui-review.r3.md';
const reportHash=crypto.createHash('sha256').update(fs.readFileSync(artifact)).digest('hex');
const id=crypto.randomUUID(),event=crypto.randomUUID(),plan='f36dda5e-ce97-49af-926c-0146968daa01',previous='8b66add8-79df-4310-9cea-02a428928dc7';
const observed=new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00');
const baseline={head:'93c4d101055f16c4493c15abde7b43141f1f8a47',branch:'main',worktree:'dirty'};
const relations=[{type:'reviews',target:plan,scope:'P4 DOM UI；U1-U4全量，U4缺证'},{type:'rechecks',target:previous,scope:'沿用目标；已关闭UI-01–UI-04回归核对及U4补证'}];
const front=o=>'---\n'+Object.entries(o).map(([k,v])=>k+': '+JSON.stringify(v)).join('\n')+'\n---\n\n';
const eventDir='.xgent-ai/sdlc/events/review/'+id;
const status='.xgent-ai/sdlc/status/review/'+id+'.md';
const eventText=front({schema_version:3,event_id:event,object_id:id,kind:'review',title:'DOCS-SSG-P4 · UI 验收 R3',artifact,facet:'review',scope:'UI-R3',state:'review-recorded',source_state:'需补证',evidence_status:'confirmed',review_type:'other',observed_at:observed,predecessors:[],baseline,sources:[{path:artifact,revision:'sha256:'+reportHash,anchor:'验收判定、评分、轮次轨迹、机器检查、采集与覆盖'},{path:'docs/plan/DOCS-SSG-P4.ui-captures/r3/capture.md',revision:'sha256:'+crypto.createHash('sha256').update(fs.readFileSync(out+'/capture.md')).digest('hex'),anchor:'被测版本、目标指纹、逐图清单'}],relations})+`# UI 验收评审 R3 · 需补证

- 本轮真实重拍91张，3位全新上下文评审，U1/U2/U3在已观察范围均10.0，U4未评分，无全量分；未见UI-01–04回归，未关闭P0/P1为0。
- 被测HEAD93c4d101，dirty指纹5b56a9c61406；固定C1bbc54b0/R740610db，CLI0.1.27。目标沿用人工规范指纹604274d613d8，未重建基线。精确定义与完整版本见原报告和baseline.json。
- 新增Safari阅读/搜索/缺译锚点/主题/语言菜单、官方中文quickstart的Chrome200%部分原生实拍，以及语言Escape、查询保留、改词重试、链接和内部滚动实测。
- 57页面/状态样本无横溢出，30次axe无violations，仍37个incomplete规则实例；不代填完整WCAG或辅助技术验收。
- 缺当前稳定桌面完整任务、两实体手机与软键盘、VoiceOver朗读及完整200%矩阵；补齐后全量review-ui。无需产品apply；不改变6/7、M7、PRD终验或发布状态。
- 报告、截图和登记仅当前工作树，未提交/推送。原文与证据：[完整报告](../../../../../${artifact})。
`;
const stateText=front({object_id:id,kind:'review',review_type:'other',artifact,archived:false,observed_at:observed,baseline,included_events:[event],relations})+`# DOCS-SSG-P4 · UI 验收 R3

来源：[完整报告](../../../../${artifact})，sha256:${reportHash}；关联[计划](../plan/${plan}.md)，复审[UI R2](${previous}.md)。

| 维度/范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review/UI-R3 | review-recorded | 需补证；已评U1/U2/U3均10.0，U4未评分，无全量分 | confirmed | 补齐U4真实环境后全量review-ui |

UI验收评审R3：旧UI-01–04未见回归，新增0；91张评分原图、3位独立评审。原生Safari部分任务、官方页Chrome200%与运行期键盘/链接/内部滚动补充见报告。被测C1bbc54b0/R740610db（0.1.27），HEAD93c4d101/dirty5b56a9c61406；目标指纹604274d613d8。

完整稳定浏览器、实体手机/软键盘、VoiceOver及200%全矩阵仍未完成。原计划6/7、M7与PRD终验不变；本轮无产品差距需apply，不自行启动代码复审或发布。资料未提交/推送。

## 已纳入事件

- [${event}](../../events/review/${id}/${event}.md) · review/UI-R3
`;
const indexes=['.xgent-ai/sdlc/status/review/index.md','.xgent-ai/sdlc/events/review/index.md'];
const before=indexes.map(p=>fs.readFileSync(p,'utf8'));
const rows=[`| ${id} | [DOCS-SSG-P4 UI 验收 R3](../../../../${artifact}) | [对象](${id}.md) | 活动 |\n\n`,`| ${id} | 子目录入口 | [对象事件](${id}/index.md) |\n\n`];
if(before.some(x=>!x.includes('维护约定：')))throw Error('Index anchor missing');
const planPath='.xgent-ai/sdlc/status/plan/'+plan+'.md';
let planText=fs.readFileSync(planPath,'utf8');
if(!planText.includes('## 本轮 UI 验收'))throw Error('Plan view anchor missing');
planText=planText.replace(/^observed_at: .*$/m,'observed_at: '+JSON.stringify(observed)).replace('## 本轮 UI 验收','## UI 验收历史与当前').replace('本轮增加原生Chrome200%定制页','R2增加原生Chrome200%定制页');
planText+=`\n[UI复审R3](../review/${id}.md)继续判需补证；沿用C1bbc54b0/R740610db和原目标指纹，U1/U2/U3已观察范围均10.0，UI-01–04未见回归，U4未评分，无全量分。本轮已补原生Safari部分任务、官方中文quickstart的Chrome200%和键盘/链接/内部滚动实测；Safari通道本轮可操作，先前cgWindowNotFound只属历史调查。完整稳定矩阵、手机/软键盘及VoiceOver仍未齐；本轮无需apply，计划状态与实施进度不变。[报告](../../../../${artifact})。\n`;
const topPath='.xgent-ai/sdlc/index.md';let top=fs.readFileSync(topPath,'utf8');
const lines=top.split('\n'),line=lines.findIndex(x=>x.startsWith('- [DOCS-SSG-P4]'));
if(line<0)throw Error('Top index plan entry missing');
lines[line]+=` [UI复审R3](status/review/${id}.md)续判需补证；U1–U3均10.0、旧差距无回归；新增Safari及官方页Chrome200%部分实拍，U4完整矩阵/手机/VoiceOver仍缺，计划6/7不变。`;
fs.mkdirSync(eventDir,{recursive:false});fs.writeFileSync(eventDir+'/'+event+'.md',eventText,{flag:'wx'});
fs.writeFileSync(eventDir+'/index.md',`# DOCS-SSG-P4 UI 验收 R3 · 事件索引\n\n本目录保存本轮独立UI评审事件，遵循[目录索引维护](../../../protocol.md#目录索引维护)，原报告和证据保留原位。\n\n| 事件ID | 维度/范围 | 文件 |\n| --- | --- | --- |\n| ${event} | review/UI-R3 | [事件](${event}.md) |\n`,{flag:'wx'});
fs.writeFileSync(status,stateText,{flag:'wx'});
for(let i=0;i<indexes.length;i++)fs.writeFileSync(indexes[i],before[i].replace('维护约定：',rows[i]+'维护约定：'));
fs.writeFileSync(planPath,planText);fs.writeFileSync(topPath,lines.join('\n'));
const result={reviewId:id,eventId:event,observedAt:observed,report:artifact,reportSha256:reportHash,files:[eventDir+'/'+event+'.md',eventDir+'/index.md',status,...indexes,planPath,topPath]};
fs.writeFileSync(out+'/sdlc-registration.json',JSON.stringify(result,null,2));console.log(JSON.stringify(result));

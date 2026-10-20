const fs=require('fs'),crypto=require('crypto');
const out=__dirname,artifact='docs/plan/DOCS-SSG-P4.ui-review.r4.md';
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const id=crypto.randomUUID(),event=crypto.randomUUID(),plan='f36dda5e-ce97-49af-926c-0146968daa01',previous='00ba6df5-d8f1-416a-9425-a14177f88a69';
const observed=new Date(Date.now()+8*3600000).toISOString().replace('Z','+08:00');
const baseline={head:'93c4d101055f16c4493c15abde7b43141f1f8a47',branch:'main',worktree:'dirty'};
const relations=[{type:'reviews',target:plan,scope:'P4 DOM UI；仅U4原生补证，U1-U3本轮未复核'},{type:'rechecks',target:previous,scope:'沿用目标；续补U4原生资源'}];
const front=o=>'---\n'+Object.entries(o).map(([k,v])=>k+': '+JSON.stringify(v)).join('\n')+'\n---\n\n';
const summary='UI验收评审R4仅U4原生补证，需补证；Safari36张原图由新上下文独立评审核查，无已确认新增产品差距。U4未评分，无全量分，U1–U3未复核。用户已提供iPhone镜像；手机使用/麦克风占用后重连超时，尚无产品图。';
const eDir='.xgent-ai/sdlc/events/review/'+id,status='.xgent-ai/sdlc/status/review/'+id+'.md';
const reportHash=hash(artifact);
const eventText=front({schema_version:3,event_id:event,object_id:id,kind:'review',title:'DOCS-SSG-P4 · UI原生补证R4',artifact,facet:'review',scope:'UI-R4',state:'review-recorded',source_state:'需补证',evidence_status:'confirmed',review_type:'other',observed_at:observed,predecessors:[],baseline,sources:[{path:artifact,revision:'sha256:'+reportHash,anchor:'验收判定、评分、采集与覆盖、下一步'},{path:'docs/plan/DOCS-SSG-P4.ui-captures/r4/capture.md',revision:'sha256:'+hash(out+'/capture.md'),anchor:'逐图出处与未采格子'}],relations})+`# UI验收评审R4 · 需补证

${summary}

固定CLI0.1.27，C1bbc54b0/R740610db，HEAD93c4d101/dirty3d2f056d84b6；目标指纹604274d613d8未变。补了原生Safari版本27.0.1、语言/搜索/章节、Copied、明确200%、module404恢复、演示和中文404；完整键盘、手机及其余原生矩阵仍缺。素材与未完成attempt、元数据修正均保留。

这是已执行评审的登记，不代表U4、M7或PRD通过。原计划6/7不变；无产品修复、提交/发布；临时preview已停止。镜像锁屏恢复可操作后，重启固定产物并续采手机。

[完整报告](../../../../../${artifact})。原文与截图仅当前工作树，未提交/推送。
`;
const stateText=front({object_id:id,kind:'review',review_type:'other',artifact,archived:false,observed_at:observed,baseline,included_events:[event],relations})+`# DOCS-SSG-P4 · UI原生补证R4

来源：[完整报告](../../../../${artifact})，sha256:${reportHash}；续接[UI R3](${previous}.md)，关联[计划](../plan/${plan}.md)。

| 维度/范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review/UI-R4 | review-recorded | 需补证；仅U4、未评分，无全量分 | confirmed | 恢复可操作iPhone镜像，重启preview并续补原生矩阵 |

${summary}

被测C1bbc54b0/R740610db，目标指纹604274d613d8；Safari27.0.1原生局部任务有证据，不能填满Chrome/Firefox/Android/VoiceOver。未完成键盘attempt不判产品不可达。原计划6/7、M7与PRD终验保持；本轮无需产品apply，未提交/发布。

## 已纳入事件

- [${event}](../../events/review/${id}/${event}.md) · review/UI-R4
`;
const indexes=['.xgent-ai/sdlc/status/review/index.md','.xgent-ai/sdlc/events/review/index.md'];
const originalIndexes=indexes.map(p=>fs.readFileSync(p,'utf8'));
if(originalIndexes.some(s=>!s.includes('维护约定：')))throw Error('Missing index anchor');
const planPath='.xgent-ai/sdlc/status/plan/'+plan+'.md';let planText=fs.readFileSync(planPath,'utf8');
planText=planText.replace(/^observed_at: .*$/m,'observed_at: '+JSON.stringify(observed));
planText+=`\n[UI原生补证R4](../review/${id}.md)：用户已提供iPhone镜像与Safari；Safari27.0.1的36张原图完成独立核查，无确认新增产品差距。手机因使用/麦克风占用后重连超时，未进入本轮产物，不能仍简单写作“未提供设备”，也不能记手机已验。仅U4补证、U4未评分，U1–U3本轮未复核；整体仍需补证，计划6/7和M7状态不变。镜像锁屏恢复可操作后重启固定产物续采；[报告](../../../../${artifact})。\n`;
const topPath='.xgent-ai/sdlc/index.md',topLines=fs.readFileSync(topPath,'utf8').split('\n');const at=topLines.findIndex(s=>s.startsWith('- [DOCS-SSG-P4]'));
if(at<0)throw Error('Missing P4 index entry');
topLines[at]+=` [U4原生补证R4](status/review/${id}.md)仍需补证；Safari36图已独立核查，手机镜像接入后超时、无产品图；U4未评分，6/7保持。`;
fs.mkdirSync(eDir);fs.writeFileSync(eDir+'/'+event+'.md',eventText,{flag:'wx'});fs.writeFileSync(eDir+'/index.md',`# DOCS-SSG-P4 UI原生补证R4 · 事件索引\n\n本目录仅保存本轮评审事件，遵循[目录索引维护](../../../protocol.md#目录索引维护)，原报告与证据保留原位。\n\n| 事件ID | 维度/范围 | 文件 |\n| --- | --- | --- |\n| ${event} | review/UI-R4 | [事件](${event}.md) |\n`,{flag:'wx'});fs.writeFileSync(status,stateText,{flag:'wx'});
const rows=[`| ${id} | [DOCS-SSG-P4 UI原生补证R4](../../../../${artifact}) | [对象](${id}.md) | 活动 |\n\n`,`| ${id} | 子目录入口 | [对象事件](${id}/index.md) |\n\n`];
indexes.forEach((p,i)=>fs.writeFileSync(p,originalIndexes[i].replace('维护约定：',rows[i]+'维护约定：')));fs.writeFileSync(planPath,planText);fs.writeFileSync(topPath,topLines.join('\n'));
const result={reviewId:id,eventId:event,observedAt:observed,report:artifact,reportSha256:reportHash,files:[eDir+'/'+event+'.md',eDir+'/index.md',status,...indexes,planPath,topPath]};fs.writeFileSync(out+'/sdlc-registration.json',JSON.stringify(result,null,2));console.log(JSON.stringify(result));

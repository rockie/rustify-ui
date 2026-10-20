const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const base = '.xgent-ai/sdlc';
const out = 'docs/plan/DOCS-SSG-P4.ui-captures/r2';
const hash = p => 'sha256:' + crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const reviewId = crypto.randomUUID(), eventId = crypto.randomUUID(), followupId = crypto.randomUUID();
const previousReview = '14f8fbe9-fef1-483e-86eb-47a0a8dec8df';
const planId = 'f36dda5e-ce97-49af-926c-0146968daa01';
const previousEvent = 'a4e3def5-99a9-4edf-86a4-ab872e5d8c4a';
const report = 'docs/plan/DOCS-SSG-P4.ui-review.r2.md';
const now = new Date().toLocaleString('sv-SE', {timeZone:'Asia/Shanghai'}).replace(' ', 'T') + '+08:00';
const baseline = {head:cp.execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(), branch:'main', worktree:'dirty'};
const reportHash = hash(report);
function writeNew(file, text) { fs.mkdirSync(path.dirname(file), {recursive:true}); fs.writeFileSync(file,text,{flag:'wx'}); }
function update(file, transform) { const before = fs.readFileSync(file,'utf8'); const after = transform(before); if(before===after) throw Error('no update '+file); fs.writeFileSync(file, after); }
function appendRow(file, row) { update(file, s => s.trimEnd()+'\n'+row+'\n'); }
const relations = [{type:'reviews', target:planId, scope:'P4 DOM UI；U1-U4全量，U4缺证'}, {type:'rechecks', target:previousReview, scope:'UI-01–UI-04；原规范基线'}];
const summary = 'UI验收评审R2需补证；U1/U2/U3在已观察范围均10.0，UI-01–04已解决，新增0；U4未评分，无全量分。';
const yaml = o => Object.entries(o).map(([k,v])=>`${k}: ${JSON.stringify(v)}`).join('\n');
const common = {schema_version:3,event_id:eventId,object_id:reviewId,kind:'review',review_type:'other',title:'DOCS-SSG-P4 UI 验收 R2',artifact:report,facet:'review',scope:'UI-R2',state:'review-recorded',source_state:'需补证',evidence_status:'confirmed',observed_at:now,predecessors:[],baseline,sources:[{path:report,revision:reportHash,anchor:'验收判定、评分、上轮差距复核、机器检查、采集与覆盖'},{path:out+'/baseline.json',revision:hash(out+'/baseline.json'),anchor:'head、dirtyDiffSha256、candidate、framework、targetFingerprint'}],relations};
const epath=`${base}/events/review/${reviewId}`;
writeNew(`${epath}/${eventId}.md`, `---\n${yaml(common)}\n---\n\n# UI 验收评审 R2\n\n${summary}\n\n- 被测HEAD93c4d101/dirty tracked diff4a112900b88a，固定CLI0.1.27 C1bbc54b0/R740610db；相关产品与C无diff。目标manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4，与R1相同。\n- 91张评分图片、3位全新上下文评审；另2张原生Chrome200%与1张事后机器图，均本轮实拍。57个运行期样本无页面横向溢出，30次axe无violations但37个incomplete未人工全验。\n- U4缺完整原生稳定浏览器任务、两手机/软键盘与VoiceOver；Chrome200%仅定制页阅读/搜索冒烟。不记全量通过、已合并、已发布或PRD终验通过。\n- 原产物与当前台账未提交/推送；未修改计划/产品/历史报告。四临时服务已停。补齐环境后按原计划全量review-ui，本轮无产品差距需apply。\n`);
writeNew(`${epath}/index.md`, `# DOCS-SSG-P4 UI 验收 R2 · 事件索引\n\n本目录保存本轮独立UI评审事件，遵循[目录索引维护](../../../protocol.md#目录索引维护)。原报告和证据保留原位。\n\n| 事件ID | 维度/范围 | 文件 |\n| --- | --- | --- |\n| ${eventId} | review/UI-R2 | [事件](${eventId}.md) |\n`);
const state={object_id:reviewId,kind:'review',review_type:'other',artifact:report,archived:false,observed_at:now,baseline,included_events:[eventId],relations};
writeNew(`${base}/status/review/${reviewId}.md`, `---\n${yaml(state)}\n---\n\n# DOCS-SSG-P4 · UI 验收 R2\n\n来源：[完整报告](../../../../${report})，${reportHash}；关联[计划](../plan/${planId}.md)，复审[UI R1](${previousReview}.md)。\n\n| 维度/范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |\n| --- | --- | --- | --- | --- |\n| review/UI-R2 | review-recorded | 需补证；已评U1/U2/U3均10.0、U4未评分，无全量分 | confirmed | 补U4完整真实环境，再全量review-ui |\n\n${summary} 目标指纹604274d613d8沿用，固定C1bbc54b0/R740610db（0.1.27），HEAD93c4d101/dirty4a112900b88a；91张评分图、3独立评审，机器与原生补充另存。\n\n缺完整原生稳定矩阵、iOS/Android实体设备与软键盘、VoiceOver朗读，Chrome200%只补部分任务。原计划6/7、M7及PRD终验状态保持；本轮不需产品修复，不自行启动代码评审或发布。报告、截图与登记仅当前工作树，未提交/推送。\n\n## 已纳入事件\n\n- [${eventId}](../../events/review/${reviewId}/${eventId}.md) · review/UI-R2\n`);
appendRow(`${base}/status/review/index.md`, `| ${reviewId} | [DOCS-SSG-P4 UI 验收 R2](../../../../${report}) | [对象](${reviewId}.md) | 活动 |`);
appendRow(`${base}/events/review/index.md`, `| ${reviewId} | 子目录入口 | [对象事件](${reviewId}/index.md) |`);
const f={...common,event_id:followupId,object_id:previousReview,title:'DOCS-SSG-P4 UI R1四项修复独立复核',artifact:'docs/plan/DOCS-SSG-P4.ui-review.md',facet:'disposition',scope:'UI-R1',state:'review-partially-applied',source_state:'UI-01–UI-04已独立复核已解决；U4仍缺证',predecessors:[previousEvent],relations:[{type:'rechecks',target:reviewId,scope:'UI-01–UI-04独立复核依据'}]};
writeNew(`${base}/events/review/${previousReview}/${followupId}.md`, `---\n${yaml(f)}\n---\n\n# R1处置更新：四项产品差距已独立复核\n\n新[UI R2报告](../../../../../${report})在固定C1bbc54b0下确认UI-01–UI-04全部已解决、新增0；R1原结论与原报告不改写。U4环境缺证仍在，因此整轮处置保留review-partially-applied，区分四项产品差距已关闭与全量UI尚未通过。\n\n无需继续修这四项；提供完整原生/实体设备/VoiceOver后全量复审。没有修改代码、计划或修复记录，没有提交/发布。\n`);
appendRow(`${base}/events/review/${previousReview}/index.md`, `| ${followupId} | disposition/UI-R1 · R2独立复核四项已解决，U4缺证 | [事件](${followupId}.md) |`);
update(`${base}/status/review/${previousReview}.md`, s => {
  s=s.replace(/^observed_at:.*$/m,`observed_at: ${JSON.stringify(now)}`).replace(/^baseline:.*$/m,`baseline: ${JSON.stringify(baseline)}`);
  s=s.replace(/^(included_events: \[.*)\]$/m,`$1, "${followupId}"]`);
  s=s.replace('| disposition/UI-R1 | review-partially-applied | UI-01–UI-04已修改待评审，未关闭差距；U4缺证 | confirmed | 新C实装下补证并独立review-ui |',`| disposition/UI-R1 | review-partially-applied | R2独立复核UI-01–04已解决；U4仍缺证 | confirmed | 补齐U4后全量复审，本轮无需产品修复 |`);
  return s+`\n## R2独立复核\n\n[UI R2](${reviewId}.md)在新C1bbc54b0下确认四项差距均已解决，U1/U2/U3已观察范围均10.0。U4仍缺证，无全量分；原R1结论保留。\n\n- [${followupId}](../../events/review/${previousReview}/${followupId}.md) · disposition/UI-R1，接续${previousEvent}。\n`;
});
update(`${base}/status/plan/${planId}.md`, s => {
  const marker='## 本轮 UI 验收';const at=s.indexOf(marker);if(at<0)throw Error('missing UI section');
  s=s.slice(0,at)+`${marker}\n\n[UI验收R1](../review/${previousReview}.md)的原结论需补证保留。[UI复审R2](../review/${reviewId}.md)已在R740610db/C1bbc54b0（CLI0.1.27）独立确认UI-01–UI-04全部已解决，新增0，已观察U1/U2/U3均10.0；U4未评分，无全量分，判定仍为需补证。91张评分图、三独立评审及运行期复核见[完整报告](../../../../${report})。\n\n本轮增加原生Chrome200%定制页阅读/搜索冒烟，但完整原生稳定矩阵、两实体手机/软键盘与VoiceOver仍缺。下一步补真实环境后全量review-ui；本轮无需产品修复。原计划6/7与M7风险门不变，UI修复代码尚需后续代码复审；未改原计划/代码，未提交/发布。\n`;
  return s.replace(/^observed_at:.*$/m,`observed_at: ${JSON.stringify(now)}`).replace(/^baseline:.*$/m,`baseline: ${JSON.stringify(baseline)}`);
});
update(`${base}/index.md`, s=>s.replace(' [UI验收R1](status/review/14f8fbe9-fef1-483e-86eb-47a0a8dec8df.md)需补证；已评9.0/9.4/9.7、U4未评分，1项P1与其余3项差距待修；Safari现可原生截图冒烟，完整环境门保留。',` [UI复审R2](status/review/${reviewId}.md)仍需补证；UI-01–04已解决，已观察U1/U2/U3均10.0，U4未评分，无全量分；新增原生Chrome200%部分冒烟，完整原生/手机/VoiceOver门保留。`));
fs.writeFileSync(out+'/sdlc-registration.json',JSON.stringify({reviewId,eventId,previousReview,followupId,now,reportHash,baseline},null,2),{flag:'wx'});
console.log(JSON.stringify({reviewId,eventId,followupId,now,reportHash}));

# DOM-COMPONENTS-P2 · UI/UX 验收 R1

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R1 · 首轮，全量范围、矩阵未全 | [DOM-COMPONENTS-P2](DOM-COMPONENTS-P2.md) | [R1冻结包](DOM-COMPONENTS-P2.ui-target.r1.md) · 43a74974da55 | main / 64bb1728，dirty 19479199a276；Studio :4181、raw/consumer :4190、catalog :4176、workbench :4174 | [r1采集](DOM-COMPONENTS-P2.ui-captures/r1/capture.md) | 无 |

## 验收判定

**需补证。**

本轮六单元已获得有效独立评分，最低 **7.5/10（U5 Select）**，默认门槛 **8/10**。完整键盘主路径、保存中、长错误/长结果窄屏、不同候选取消及兼容入口最终成功等必需证据尚缺，先按缺证裁定。即使补齐证据，已确认的浮层叠字、状态难辨及默认对比度/控件名称问题仍需处理，不能据当前材料判通过。

本轮采用DOM入口；GPU只看既有compare兼容边界。未修改产品代码、原计划及历史报告，未提交或发布。首轮无停滞预警；未进行修复，未以补采冒充修复后复审。

你已确认“**只要求已有说明正确关联（推荐）**”。R1最初将卡片条款抽取过严，冻结目标和独立原文保留；[当前目标包](DOM-COMPONENTS-P2.ui-target.md)已按确认修正。**UI-02不要求修复方给状态样本新增说明**，下一轮用新指纹重建基线，再由独立评审核对适用性。此决定不是对整体界面的豁免，不回改本轮评分。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互状态 /3 | 细节 /1 | 合并总分 /10 | 初采 / 补采 |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| U1 完整本地表单 | 3 | 3 | 2 | 1 | **9.0** | 9.5 / 9.0，取补采整组 |
| U2 选择/卡片工具 | 3 | 2.5 | 1.5 | 1 | **8.0** | 8.0 / 8.5，取初采整组 |
| U3 字段/插槽 | 3 | 2.5 | 2.5 | 1 | **9.0** | 9.0 / 9.0，沿用初采整组 |
| U4 滑块 | 2 | 3 | 2 | 1 | **8.0** | 8.0 / — |
| U5 Select | 3 | 2 | 2 | 0.5 | **7.5** | 7.5 / — |
| U6 raw/兼容入口 | 3 | 3 | 1.5 | 0.5 | **8.0** | 8.0 / — |

四维理由及全部评审差距见[合并 judge.md](DOM-COMPONENTS-P2.ui-captures/r1/judge.md)，原文保留见文末。每个单元取较低总分那位评审的完整四维组，不逐维拼接；差距取全部评审并集。分数只覆盖实际展示范围，缺证未计通过或失败。

- 本轮分数/本基线历史最佳：7.5 / 7.5；完整矩阵仍缺证。
- 冻结R1未关闭P0：0；P1：5（UI-02、09、14、18、20）。其中UI-02已有用户范围澄清，保留历史结论，下一轮重建基线；另4条仍有独立实拍/页面实测支持。
- 机器检查不改独立分数；其P1另外计入门槛。因本轮不拟通过，不触发临界通过确认评审。
- 新目标指纹尚无评分，不能以本轮7.5替代其新基线分。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 7.5（U5） | 7.5 | 0 / 5，含1条用户已澄清适用范围 | — | 需补证；首轮冻结指纹43a74974da55；下一轮因卡片口径修正重建基线 |

## 差距清单

共20条：17条独立评审来源、3条额外机器检查来源。正式编号映射保留在judge.md。下列按严重度、可挽回分排序；机器项不另造可挽回分。UI-02保留冻结R1结论及用户决定，UI-05保留两评审分歧，修复方须先核实，不盲目增补内容。

### UI-09 · [P1] 无皮肤Select浮层与底层叠字（U5 · 可挽回1）

- 位置：[U5-raw-long-end-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U5-raw-long-end-1440-light.png)；约x8–211,y229–686列表及空列表。
- 现状 → 期望：长列表与底层标题、标签、按钮叠字，空列表也压在底层内容上。 → G-2/U5-2：基本可读性及活动项在自身viewport呈现。
- 修法：由raw消费者提供最小隔离底面与边界；不引入品牌皮肤。复拍空列表与长列表End。
- 来源：[judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md)。

### UI-14 · [P1] 无皮肤表单保存中/完成状态不明确（U6 · 可挽回0.75）

- 位置：[U6-raw-form-busy-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U6-raw-form-busy-1440-light.png)；Native fields状态区与Save。
- 现状 → 期望：历史字符串与120/230不能直接区分正在保存或已保存；失败有Try again但序列不清。 → G-2/U6-2：状态可读、消费者状态一致。
- 修法：历史之外输出独立具名Saving/Saved/Save failed状态，不声称发生重复提交。
- 来源：[judge-u6.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u6.raw.md)。

### UI-02 · [P1] 状态工具卡片缺少说明（U2 · 可挽回0.5）

- 位置：[U2-RadioCards-s2-surface-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U2-RadioCards-s2-surface-1440-light.png)；Email / Activity feed卡片。
- 现状 → 期望：卡片只有名称，没有可见说明。 → 冻结U2-1：卡片说明可见且关联。
- 修法：补充次级说明并关联选项，尺寸变化与窄屏保留。
- 来源：[judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md)。
- **用户决定/核对说明：**原PRD要求已有可见说明不丢关联；用户确认无需给每张状态样本新增说明。完整表单已有说明，状态工具只有名称，不能据本条强制添写内容。冻结R1初采P1/0.5与补采P2/0.4均保留；下轮重建基线，不执行上述旧修法。

### UI-18 · [P1] 默认浅色按钮普通文字对比度不足（U1/U2 · 不调整分数）

- 位置：[M-default-contrast-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/M-default-contrast-1440-light.png)；完整表单 Fail next local save；状态工具 Set indeterminate。
- 现状 → 期望：axe及页面色值实测为4.372367774254193:1（报告显示4.37），前景#1570ef、底面#f9fafb、普通15px文字。 → G-2：普通文本4.5:1。
- 修法：调整默认浅色相应按钮的文字/底面配对，复测正常、聚焦和交互态，保留作者局部配色边界。
- 来源：[页面实测](DOM-COMPONENTS-P2.ui-captures/r1/machine-checks.md)，主 agent 页面实测。

### UI-20 · [P1] 独立消费者两个Select有值但没有可访问名称（U5/U6 · 不调整分数）

- 位置：[M-select-names-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/M-select-names-1440-light.png)；select-responsive、select-typed两枚Trigger。
- 现状 → 期望：可见值Apricot；axe两次button-name失败，Chrome AX Tree实测role=combobox、name为空、value为Apricot。 → G-2/U6-1：控件标签与名字有效，值不能替代名称。
- 修法：为这两个消费样例添加可见标签并关联实际Trigger，再读取AX name和键盘走查；不把结论泛化到全部Select。
- 来源：[页面实测](DOM-COMPONENTS-P2.ui-captures/r1/machine-checks.md)，主 agent 页面实测及Chrome无障碍树。

### UI-10 · [P2] 多滑块端点thumb裁切（U4 · 可挽回1）

- 位置：[U4-range-crossed-focus-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U4-range-crossed-focus-1440-light.png)；LTR最右端及dark/RTL最左端。
- 现状 → 期望：接受值100时thumb成为窄形/半圆，其他thumb完整。 → U4-1/G-4：稳定身份与必要圆形。
- 修法：调整轨道端点与thumb半径或局部裁剪，保留完整thumb与焦点轮廓；复拍LTR/RTL max。
- 来源：[judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md)。

### UI-11 · [P2] 无皮肤滑块反馈连写（U4 · 可挽回1）

- 位置：[U4-raw-vertical-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U4-raw-vertical-1440-light.png)；约x8,y759反馈行。
- 现状 → 期望：vertical=36left=20,right=80000等连写，值和计数不可辨。 → G-3/U4-3：区分请求、应用值和完成/取消。
- 修法：用具名分行反馈显示接受值、最后请求、请求数、commit/cancel计数。
- 来源：[judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md)。

### UI-12 · [P2] 无皮肤Select反馈无标签（U5 · 可挽回1）

- 位置：[U5-raw-empty-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U5-raw-empty-1440-light.png)；Clear value之后状态串。
- 现状 → 期望：布尔值、焦点事件、业务值、计数直接连在按钮后。 → G-3：请求数、最后请求、应用值清楚区分。
- 修法：反馈与按钮分离，具名显示接受值、请求、开关及取消结果。
- 来源：[judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md)。

### UI-15 · [P2] 无皮肤选择反馈连写（U6 · 可挽回0.75）

- 位置：[U6-raw-refused-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U6-raw-refused-1440-light.png)；Controlled choices状态。
- 现状 → 期望：Some(a)1Some(b)、Indeterminate00等连续字符串需猜含义；A接受值保留已有证据。 → G-3：策略工具区分请求数、最后请求、应用值。
- 修法：用无主题标签与分行，标清对应控件。
- 来源：[judge-u6.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u6.raw.md)。

### UI-01 · [P2] 错误来源未直接区分（U1 · 可挽回0.5）

- 位置：[U1-constraint-focus-a3-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U1-constraint-focus-a3-1440-light.png)；Display name错误行。
- 现状 → 期望：三类错误都是红框加无来源说明的文本，读者需推断来自约束、规则还是保存。 → U1-2：错误来源可区分。
- 修法：错误行加入面向使用者的简短来源说明，保留修正指引；补采Check错误。
- 来源：[judge-u1.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u1.raw.md)；[judge-supplement-u1.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-supplement-u1.raw.md)。

### UI-03 · [P2] soft Switch未选中轮廓弱（U2 · 可挽回0.5）

- 位置：[U2-Switch-s3-soft-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U2-Switch-s3-soft-1440-light.png)；示例左侧关闭状态Switch。
- 现状 → 期望：轨道、thumb与底面接近，控件近似消失；未断言精确对比度。 → G-2/G-4：必要边界可辨、胶囊形状保持。
- 修法：提高正常未选中轨道/轮廓辨识，保留soft且区别禁用。
- 来源：[judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md)。
- **页面补充实测：**稳定后轨道#f2f4f7对底面#f9fafb仅1.0544:1，透明边框；支持辨识弱的观察。原独立严重度与分数保留，修复时按G-2必要边界3:1核对，不把外层示例框算作控件边界。

### UI-07 · [P2] 字段invalid缺少文字解释（U3 · 可挽回0.5）

- 位置：[U3-ColorField-invalid-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U3-ColorField-invalid-1440-light.png)；颜色输入行。
- 现状 → 期望：有效外观的#336699仅用红框表示invalid，无原因/修正提示。 → G-2/U3-1：帮助与状态，不只用颜色。
- 修法：增加就近状态文字并关联；若是强制 invalid 演示，明确说明演示性质，不编造颜色无效原因，也不把未完成 Draft 都解释成最终错误。
- 来源：[judge-u3.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u3.raw.md)。

### UI-08 · [P2] 试验字段缺少可见标签/格式帮助（U3 · 可挽回0.5）

- 位置：[U3-TextField-s2-surface-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U3-TextField-s2-surface-1440-light.png)；Example输入及同组字段。
- 现状 → 期望：只有通用说明，没有字段可见标签/格式帮助；不推断accessible name缺失。 → U3-1/G-2：标签与帮助指向编辑元素。
- 修法：补简短标签，数字/颜色给格式或操作帮助并关联。
- 来源：[judge-u3.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u3.raw.md)。

### UI-17 · [P2] 未完成数量草稿仅用红框表达（U1 · 可挽回0.5）

- 位置：[U1-draft-native-before-a4-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U1-draft-native-before-a4-1440-light.png)；Quantity 输入框及帮助行，约x369–868、y484–516。
- 现状 → 期望：聚焦草稿为 - 时红框可见，帮助仍是通用提交说明，没有文字说明数字未完成。 → G-2：状态不只用颜色反馈，允许保留未完成 Draft。
- 修法：在现有帮助旁显示数字未完成及继续输入/取消提示，保持应用接受值，不提前提交 Draft。
- 来源：[judge-supplement-u1.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-supplement-u1.raw.md)。

### UI-04 · [P2] Checkbox/Switch缺少可见用途标签（U2 · 可挽回0.4）

- 位置：[U2-Checkbox-s2-surface-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U2-Checkbox-s2-surface-1440-light.png)；实际试验控件。
- 现状 → 期望：上方Component只说明类型，试验控件没有说明选择/开启什么。 → G-2：控件有标签、帮助、状态。
- 修法：补可见用途标签及必要帮助并关联；不推断隐藏名称缺失。
- 来源：[judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md)。

### UI-05 · [P2] 卡片invalid仅靠红边表达（U2 · 可挽回0.3）

- 位置：[U2-CheckboxCards-invalid-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U2-CheckboxCards-invalid-1440-light.png)；卡片组。
- 现状 → 期望：小控件红边、选中外框仍蓝；没有局部错误文字。 → G-2：状态反馈不只靠颜色。
- 修法：增加组旁错误文本并关联，保持选中与错误可辨。
- 来源：[judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md)。
- **核对说明：**补采评审认为State选择器的Invalid已提供非颜色状态，判不再适用；初采仍判存在。按合并规则保留未关闭条目。修复方先核实演示状态说明与关联是否已足够，不编造业务错误；下一轮独立评审复核。

### UI-06 · [P2] ToggleGroup选择规则不明确（U2 · 可挽回0.3）

- 位置：[U2-ToggleGroup-default-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U2-ToggleGroup-default-1440-light.png)；Email/Activity feed组。
- 现状 → 期望：未说明单/多选及是否可清空，需要从数组结果推断。 → U2-1：ToggleGroup规则明确。
- 修法：增加简短选择数量和清空规则说明。
- 来源：[judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md)。

### UI-19 · [P2] CheckboxGroup使用角色不支持的必填属性（U1 · 不调整分数）

- 位置：[M-default-contrast-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/M-default-contrast-1440-light.png)；完整表单 CheckboxGroup；实测#rui-choice-group-214。
- 现状 → 期望：实际DOM为role=group并带aria-required=true；浅/深/390深色均触发axe aria-allowed-attr。 → U1-2/G-2：字段语义与关联有效；必填信息应可被正确解释。
- 修法：采用符合多选组语义的必填说明与有效关联，移除不受支持的属性；保留组键盘和提交行为，不改成错误角色以消除扫描。
- 来源：[页面实测](DOM-COMPONENTS-P2.ui-captures/r1/machine-checks.md)，主 agent 页面实测。

### UI-13 · [P3] 无皮肤Select标签/Trigger连写（U5 · 可挽回0.5）

- 位置：[U5-raw-empty-1440-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U5-raw-empty-1440-light.png)；Trigger及Policy/Entries。
- 现状 → 期望：Choose fruitOpen及相邻标签控件连写。 → G-2：标签、帮助和状态可读。
- 修法：增加最小空白或布局间距，不增加品牌样式。
- 来源：[judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md)。

### UI-16 · [P3] 无皮肤相邻文案缺分隔（U6 · 可挽回0.5）

- 位置：[U6-raw-mobile-390-light.png](DOM-COMPONENTS-P2.ui-captures/r1/U6-raw-mobile-390-light.png)；Mixed checkbox/Notifications；Select Trigger。
- 现状 → 期望：mixed markerNotifications、BananaOpen等连写。 → G-2/U6-2：无皮肤标签仍可直接阅读。
- 修法：增加语义片段空白、换行或独立文本块，不引入皮肤。
- 来源：[judge-u6.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u6.raw.md)。

## 待裁定事项

无新增待用户决定事项。卡片说明口径已由用户明确，记录在本报告、当前目标包与其变更记录；下轮重建基线。UI-05是评审证据解释分歧，保留供修复方核实和下轮复核，不因此再次要求用户拍板。

## 建议（不扣分）

- raw消费者补最小布局、隔离底面和具名状态即可，无需增加品牌皮肤或新信息架构。
- 原生校验语言由系统提供；本机EN页面出现中文validationMessage，先明确是否要求统一，不凭截图判翻译缺陷。
- FormData可选紧凑视图；Number/Color取消后可解释Commit/cancel读数统计范围。
- 独立消费者Submit fruit高21px，axe提示WCAG2.2 target-size。目标包未单列该版本门槛，本轮作可用性建议；可扩大命中区域/间距。
- compare旧Profile卡片容纳、catalog悬停与当前项区分另行观察，本轮不重评完整既有应用。

## 机器检查

| 项目 | 工具与结果 | 对应差距/限制 |
| --- | --- | --- |
| 当前产物 | 本轮mbx重建Studio、catalog、workbench release及两个独立DOM消费者，均成功；build manifest见下文 | 浏览器加载当前产物；不复用旧构建冒充本轮 |
| 默认文字对比 | axe4.13.0及页面实测色值4.372367774254193:1 | UI-18/P1 |
| 必填组ARIA | 实际role=group与aria-required组合在浅/深/手机深色触发aria-allowed-attr | UI-19/P2；不把axe impact直接当严重度 |
| Select名字 | axe两次失败；Chrome AX Tree稳定后name空、value Apricot | UI-20/P1；只覆盖两枚独立consumer样例 |
| soft Switch边界 | 稳定后轨道/底面1.0544:1 | UI-03补充测量，不改独立评分 |
| 横向溢出 | Studio1440与390文档及表单scrollWidth未超容器 | 仅限所查范围，内部隐藏thumb文本不算页面溢出 |
| 原生按键 | 名称输入Tab/Shift+Tab有focus-visible及3px轮廓；单次Meta+Z；补采Meta+ArrowLeft选区0–2 | 不代替完整键盘、连续更新、TextArea或剪贴板验证 |
| Slot读数 | 1440及390自定义utility场景gap/padding/颜色已记录 | 不用自定义值推断默认值或完整优先级 |
| console/失败请求 | 162条采集日志：10条document_title警告、5条URL未记录的资源404；无pageerror/requestfailed/页面response≥400记录 | 404具体资源仍待定位，未声称零错误；不猜是favicon |

原始结果、实拍、文件hash、测量限制见[machine-checks.md](DOM-COMPONENTS-P2.ui-captures/r1/machine-checks.md)。局部扫描零violation不代表整体AA或辅助技术通过。本轮未重跑全部Rust测试、全量Playwright、性能与发行门；既有工程记录不作本轮UI通过依据。

## 采集与覆盖

初采131张原始图中112张主证据，补采11张；独立评审输入共123张不同实拍，机器检查另3张。总145张PNG保留，含19张早期取景不足尝试，不删除不覆盖。主 agent已实际查看112张主证据、11张补采及3张机器图；每位独立评审逐张开图结果见原文。

[完整采集清单](DOM-COMPONENTS-P2.ui-captures/r1/capture.md)、[补证矩阵与方法](DOM-COMPONENTS-P2.ui-captures/r1/coverage.md)、[全量PNG哈希](DOM-COMPONENTS-P2.ui-captures/r1/capture-hashes.md)。必需缺证摘要：

| 单元 | 尚缺的关键证据 | 获取方式 |
| --- | --- | --- |
| U1 | busy/重复保存计数、check错误、完整键盘主路径、长错误/长FormData窄屏、reset/Draft完整边界 | 现有完整表单逐步实际操作并在持续状态拍帧；确无入口的check/拒绝reset需提供真实可达fixture |
| U2 | 组完整键盘/表单一致性、延迟接受后焦点、局部能力单变量对照 | 同批数据比较普通组/卡片的请求与接受值/FormData；已有说明关联按用户新口径 |
| U3 | Enter/blur一次提交、连续更新光标、TextArea选区/撤销/resize、长错误、Slot优先级完整对照 | 现有consumer与工具执行成对动作，记录次数、关联与焦点 |
| U4 | 完成最后候选payload、完整键盘/间隔限制、thumb语义 | 实际输出候选/接受值及计数，读取无障碍树，不从commit标记推断payload |
| U5 | A→取消B→仍A、Escape/外点、禁用项/已选移除、局部滚动读数、中文浮层 | 先固定不同原值/候选；修复叠字后记录列表及页面scrollTop和焦点 |
| U6 | raw手机下半段及键盘、Portal方向/清理、catalog编辑结果、Workbench最终保存 | 从正式入口走通最终操作并拍接受结果，不把分类切换/错误清除当任务成功 |

共性Edit/Preview往返、两区独立滚动、宽度/模式对主题文档/历史无副作用、完整标签/帮助/错误程序关联仍需补证。截图只见中文上半段，不等于整页翻译通过。现有环境已可运行，大部分不缺账号或外部服务；本轮受限于未到达/未固定状态和操作覆盖，不把这些写成产品缺陷。

OS IME、VoiceOver、真机触控、浏览器UI原生200%缩放未执行，按计划留P5 missing，不新增为P2 UI门槛。

### 被测环境及指纹

- HEAD：`64bb17281567a646b312a4b7050219fe5a7f0c55`，main；原采集记录的dirty摘要 `19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c`。初始集合为AGENTS.md、历史报告及SDLC的tracked/untracked内容，完整路径和逐文件SHA-256见[version.json](DOM-COMPONENTS-P2.ui-captures/r1/logs/version.json)；该集合摘要的编码未保留，不用于跨轮算法比较，以逐文件SHA-256核对范围。产品源码无未提交差异。本轮仅新增UI评审产物、最小日志忽略和台账；未修改产品实现。
- Playwright1.63.0 + 本机Google Chrome156.0.8078.74，headless；1440×1000、768×1000、390×844，DPR1；locale en-US、Asia/Shanghai。真实字体完成、截图禁用动画；无网络拦截、无模拟页面、无截图编辑；本地演示数据，无外部业务写入。
- GPU兼容页存在document_title警告；5条404无法据现有日志确定资源URL，列环境记录。OS原生validationMessage为中文；Mac选区已纠正Home为Meta+ArrowLeft，原始失败尝试保留。
- 本机缺固定Phoenix review engine（runtime0.1.0探测失败），沿人工Markdown流程；无managed索引，未安装CLI，未假称managed登记。人工指纹算法 `phoenix-ui-target-v1/manual-u64be`：ASCII字符串 `phoenix-ui-target-v1` 后接NUL字节作为域，依次取“单元”“规范要点”h2节（含标题，LF，末尾一个LF），各段UTF-8字节长u64BE前缀；无直接本地附件。未与engine算法互认，未知算法不能跨轮比较。

| 来源/目标 | SHA-256或目标指纹 |
| --- | --- |
| R1冻结目标全文 | 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89 |
| R1冻结期望指纹 | 43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29 |
| 当前目标全文（用户澄清后） | 3badcee12fcd6363901e6fe0be0f87541e7300562e928caa93365b8da859002d |
| 下轮新期望指纹 | c83c6339be779c05baf3a1c91dc24d47ba7bd122b975b35759649fb4342525af |
| 开发计划 | f3a015abb4d8483abd0ffc080f93d9bc9c459b37dbb96880b806ecb7610b65d5 |
| PRD | b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841 |
| DESIGN.md | 9bfb378638e9113fda66d7931275959c75078d3e00e46bf28b4791dc82adeec5 |
| PRODUCT.md | ca23ffd6b5f64fd4d77204d04c2c057f39b9947a4a8ec08fec6b71963040383b |

| 本轮构建 | build_id | build-manifest.json SHA-256 |
| --- | --- | --- |
| theme-studio | dc7fb113f1267f33 | 91291ff32dc6f2b28065dde8f563005b4bdb674046c61b797e32891396056b4b |
| component-catalog | 0197aee98a42ae90 | b224a657ecf592edbb4a9a9db929f11976108b3eb9c311ee05c4ec1d452ef193 |
| property-workbench | b2026f416f646082 | 103dfe22c05bf1b26bb347df362bb00437216344f0c9eee2f3996e7766a7d907 |

DOM消费者分别通过 `mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml` 和consumer/Cargo.toml重建；原始构建、服务、浏览器日志集中本轮logs/。PNG/logs按仓库规则忽略，仅原工作树可直接复现，另一机器需重建并重拍；目标包、报告、采集与评审Markdown不忽略。

本轮临时服务已停止：Studio4181、catalog4176、workbench4174、consumer4190。停止前按PID及监听端口核对均为本轮xtask，停止后四端口无监听；未关闭用户浏览器。

### 独立评审方式与核对

共8位全新上下文、只读、继承主模型的subagent，无模型/effort覆盖：初采5位（U1、U2、U3、U4+U5、U6），补采3位（U1、U2、U3）。每位只读指定目标、自己单元的capture/supplement及其直接PNG，补采另给相应初采评审原文；不给源码、diff、完整报告、修复记录、门槛和机器检查，不沿目标来源链接扩读。

宿主未暴露各调用的具体模型及effort标识，记录为继承，未猜写型号。工具轨迹由各评审原文记录实际开图和白名单边界，不声称操作系统文件权限隔离。

| 阶段/单元 | 原文 | 实际开图 |
| --- | --- | --- |
| 初采U1 | [judge-u1.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u1.raw.md) | 22/22 |
| 初采U2 | [judge-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u2.raw.md) | 28/28 |
| 初采U3 | [judge-u3.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u3.raw.md) | 25/25 |
| 初采U4/U5 | [judge-u4-u5.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u4-u5.raw.md) | 24/24 |
| 初采U6 | [judge-u6.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-u6.raw.md) | 13/13 |
| 补采U1 | [judge-supplement-u1.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-supplement-u1.raw.md) | 27/27 |
| 补采U2 | [judge-supplement-u2.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-supplement-u2.raw.md) | 31/31 |
| 补采U3 | [judge-supplement-u3.raw.md](DOM-COMPONENTS-P2.ui-captures/r1/judge-supplement-u3.raw.md) | 28/28 |

开图失败0。补采因初采看不出的格子发起，仍按较低整组分合并，不因争议换评审。UI-02用户说明、UI-05证据分歧的核对说明只在本报告，不回写评分理由；未以主 agent意见删差距或提分。

## 下一步

先按本报告处理已确认差距，UI-02遵循用户新口径，UI-05先核实，不执行自动添写：

```text
$apply-ui-review docs/plan/DOM-COMPONENTS-P2.ui-review.md
```

补齐[覆盖矩阵](DOM-COMPONENTS-P2.ui-captures/r1/coverage.md)的真实状态/操作证据后重新发起；因目标指纹已变化，下轮为重建基线，沿用仍适用问题编号：

```text
$review-ui docs/plan/DOM-COMPONENTS-P2.md
```

本轮不自行启动修复、代码复审或PRD差异核查。首轮来源为计划、PRD、DESIGN.md及PRODUCT.md，规范模式、三个视口、浅深主题、英文主采中文补充；没有新增目标图。用户已确认唯一提出的卡片说明歧义，其余来源与假设在当前目标包中可查。

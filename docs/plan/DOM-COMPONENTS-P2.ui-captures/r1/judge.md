# R1 独立评审合并

## 评分与理由

| 单元 | 布局 /3 | 视觉 /3 | 交互状态 /3 | 细节 /1 | 总分 /10 | 采用的整组评审 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| U1 | 3 | 3 | 2 | 1 | 9.0 | [judge-supplement-u1.raw.md](judge-supplement-u1.raw.md) |
| U2 | 3 | 2.5 | 1.5 | 1 | 8.0 | [judge-u2.raw.md](judge-u2.raw.md) |
| U3 | 3 | 2.5 | 2.5 | 1 | 9.0 | [judge-u3.raw.md](judge-u3.raw.md) |
| U4 | 2 | 3 | 2 | 1 | 8.0 | [judge-u4-u5.raw.md](judge-u4-u5.raw.md) |
| U5 | 3 | 2 | 2 | 0.5 | 7.5 | [judge-u4-u5.raw.md](judge-u4-u5.raw.md) |
| U6 | 3 | 3 | 1.5 | 0.5 | 8.0 | [judge-u6.raw.md](judge-u6.raw.md) |

初采与补采的同单元整组总分：U1 9.5 / 9.0，取9.0；U2 8.0 / 8.5，取8.0；U3 9.0 / 9.0，沿用初采整组。其余只有初采。补采未进行产品修复。

### U1

- 布局 3：桌面双列表单、平板与手机单列成立；卡片、组、帮助、操作区按可用宽度排布。已展示区域未见横向溢出、文字遮挡或不可达操作。
- 视觉 3：中性编辑壳与深浅预览边界清楚；选中和键盘焦点可见，必要的方框、圆形及卡片形状保持。
- 交互状态 2：错误就近、首错聚焦、本地成功、重试及数量接受值有证据；补采确认组箭头操作和 Select 选择回焦。错误来源仍不够直接可读，未完成数量草稿仅用红框表达。
- 细节 1：已展示标签、帮助、数值和操作文案可读，手机按钮能够换行；未发现另一个有明确规范依据的细节扣分项。

### U2

- 布局 3：桌面分栏与390px单列清楚，已拍范围未见溢出、遮挡或结构崩坏。
- 视觉 2.5：尺寸、变体、形状和局部颜色有变化，但 soft Switch 未选中边界隐约。
- 交互状态 1.5：拒绝、延迟与接受值分离，但卡片说明、用途标签、组规则及局部错误反馈有缺口。
- 细节 1：已拍文字和结果未见明显截断、错位或语言混杂。

### U3

- 布局 3：尺寸递进、RTL插槽交换及390px单列成立，未见重叠或越界。
- 视觉 2.5：变体和主题有区别，invalid主要依靠红边表达。
- 交互状态 2.5：Tab焦点可见，Draft Escape恢复；试验字段缺少邻近可见标签和专属帮助。
- 细节 1：未见额外截断、语言混杂或细部对齐缺陷；有意utility未扣分。

### U4

- 布局 2：排列稳定，但跨越到端点后LTR右端和RTL左端thumb被裁切。
- 视觉 3：尺寸、变体、局部红色与dark/RTL可辨。
- 交互状态 2：拒绝、跨越焦点、方向和取消有记录；raw反馈连写，状态难以辨认。
- 细节 1：Studio文字、数值格式与对齐没有额外可确认差异。

### U5

- 布局 3：桌面与390px dark/RTL浮层在视口内，未见横向溢出。
- 视觉 2：Themes可读，raw浮层底层文字穿透并叠字。
- 交互状态 2：键盘打开、定位、选择与返回焦点有记录；raw反馈没有可辨标签和边界。
- 细节 0.5：raw Trigger和控制标签直接连写。

### U6

- 布局 3：Raw桌面/390px可换行，本期表单字段仍可读。
- 视觉 3：Raw原生呈现；Workbench错误和焦点、compare边界可辨。
- 交互状态 1.5：接受值和原生阻塞有证据，但请求连写，保存中/完成状态不明确。
- 细节 0.5：相邻标签或按钮文字缺分隔。

## 评审来源的全部未关闭差距

差距取初采与补采评审的并集；同一差距复核冲突时保留较靠前的未关闭状态。临时编号映射写在每项来源中。

### UI-01 · [P2] 错误来源未直接区分（U1 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U1-constraint-focus-a3-1440-light.png](U1-constraint-focus-a3-1440-light.png)；Display name错误行。
- 现状 → 目标：三类错误都是红框加无来源说明的文本，读者需推断来自约束、规则还是保存。 → U1-2：错误来源可区分。
- 修法：错误行加入面向使用者的简短来源说明，保留修正指引；补采Check错误。
- 评审来源：judge-u1.raw.md；judge-supplement-u1.raw.md，临时编号「新-1」。

### UI-02 · [P1] 状态工具卡片缺少说明（U2 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U2-RadioCards-s2-surface-1440-light.png](U2-RadioCards-s2-surface-1440-light.png)；Email / Activity feed卡片。
- 现状 → 目标：卡片只有名称，没有可见说明。 → 冻结U2-1：卡片说明可见且关联。
- 修法：补充次级说明并关联选项，尺寸变化与窄屏保留。
- 评审来源：judge-u2.raw.md，临时编号「新-1」。
- 补采评审仍判存在，但建议严重度 P2、可挽回0.4；合并保留初采 P1 和0.5。两位均依据冻结的 R1 目标。

### UI-03 · [P2] soft Switch未选中轮廓弱（U2 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U2-Switch-s3-soft-1440-light.png](U2-Switch-s3-soft-1440-light.png)；示例左侧关闭状态Switch。
- 现状 → 目标：轨道、thumb与底面接近，控件近似消失；未断言精确对比度。 → G-2/G-4：必要边界可辨、胶囊形状保持。
- 修法：提高正常未选中轨道/轮廓辨识，保留soft且区别禁用。
- 评审来源：judge-u2.raw.md，临时编号「新-2」。

### UI-04 · [P2] Checkbox/Switch缺少可见用途标签（U2 · 可挽回 0.4）

- 状态：仍存在。
- 位置：[U2-Checkbox-s2-surface-1440-light.png](U2-Checkbox-s2-surface-1440-light.png)；实际试验控件。
- 现状 → 目标：上方Component只说明类型，试验控件没有说明选择/开启什么。 → G-2：控件有标签、帮助、状态。
- 修法：补可见用途标签及必要帮助并关联；不推断隐藏名称缺失。
- 评审来源：judge-u2.raw.md，临时编号「新-3」。

### UI-05 · [P2] 卡片invalid仅靠红边表达（U2 · 可挽回 0.3）

- 状态：仍存在。
- 位置：[U2-CheckboxCards-invalid-1440-light.png](U2-CheckboxCards-invalid-1440-light.png)；卡片组。
- 现状 → 目标：小控件红边、选中外框仍蓝；没有局部错误文字。 → G-2：状态反馈不只靠颜色。
- 修法：增加组旁错误文本并关联，保持选中与错误可辨。
- 评审来源：judge-u2.raw.md，临时编号「新-4」。
- 补采评审判「不再适用」：State 选择器已有 Invalid，冻结规范未明确要求业务原因；初采判仍存在，合并保留未关闭结论。原文见 [补采 U2](judge-supplement-u2.raw.md)。

### UI-06 · [P2] ToggleGroup选择规则不明确（U2 · 可挽回 0.3）

- 状态：仍存在。
- 位置：[U2-ToggleGroup-default-1440-light.png](U2-ToggleGroup-default-1440-light.png)；Email/Activity feed组。
- 现状 → 目标：未说明单/多选及是否可清空，需要从数组结果推断。 → U2-1：ToggleGroup规则明确。
- 修法：增加简短选择数量和清空规则说明。
- 评审来源：judge-u2.raw.md，临时编号「新-5」。

### UI-07 · [P2] 字段invalid缺少文字解释（U3 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U3-ColorField-invalid-1440-light.png](U3-ColorField-invalid-1440-light.png)；颜色输入行。
- 现状 → 目标：有效外观的#336699仅用红框表示invalid，无原因/修正提示。 → G-2/U3-1：帮助与状态，不只用颜色。
- 修法：增加就近状态文字并关联；若是强制 invalid 演示，明确说明演示性质，不编造颜色无效原因，也不把未完成 Draft 都解释成最终错误。
- 评审来源：judge-u3.raw.md，临时编号「新-1」。

### UI-08 · [P2] 试验字段缺少可见标签/格式帮助（U3 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U3-TextField-s2-surface-1440-light.png](U3-TextField-s2-surface-1440-light.png)；Example输入及同组字段。
- 现状 → 目标：只有通用说明，没有字段可见标签/格式帮助；不推断accessible name缺失。 → U3-1/G-2：标签与帮助指向编辑元素。
- 修法：补简短标签，数字/颜色给格式或操作帮助并关联。
- 评审来源：judge-u3.raw.md，临时编号「新-2」。

### UI-09 · [P1] 无皮肤Select浮层与底层叠字（U5 · 可挽回 1）

- 状态：仍存在。
- 位置：[U5-raw-long-end-1440-light.png](U5-raw-long-end-1440-light.png)；约x8–211,y229–686列表及空列表。
- 现状 → 目标：长列表与底层标题、标签、按钮叠字，空列表也压在底层内容上。 → G-2/U5-2：基本可读性及活动项在自身viewport呈现。
- 修法：由raw消费者提供最小隔离底面与边界；不引入品牌皮肤。复拍空列表与长列表End。
- 评审来源：judge-u4-u5.raw.md，临时编号「新-1」。

### UI-10 · [P2] 多滑块端点thumb裁切（U4 · 可挽回 1）

- 状态：仍存在。
- 位置：[U4-range-crossed-focus-1440-light.png](U4-range-crossed-focus-1440-light.png)；LTR最右端及dark/RTL最左端。
- 现状 → 目标：接受值100时thumb成为窄形/半圆，其他thumb完整。 → U4-1/G-4：稳定身份与必要圆形。
- 修法：调整轨道端点与thumb半径或局部裁剪，保留完整thumb与焦点轮廓；复拍LTR/RTL max。
- 评审来源：judge-u4-u5.raw.md，临时编号「新-2」。

### UI-11 · [P2] 无皮肤滑块反馈连写（U4 · 可挽回 1）

- 状态：仍存在。
- 位置：[U4-raw-vertical-1440-light.png](U4-raw-vertical-1440-light.png)；约x8,y759反馈行。
- 现状 → 目标：vertical=36left=20,right=80000等连写，值和计数不可辨。 → G-3/U4-3：区分请求、应用值和完成/取消。
- 修法：用具名分行反馈显示接受值、最后请求、请求数、commit/cancel计数。
- 评审来源：judge-u4-u5.raw.md，临时编号「新-3」。

### UI-12 · [P2] 无皮肤Select反馈无标签（U5 · 可挽回 1）

- 状态：仍存在。
- 位置：[U5-raw-empty-1440-light.png](U5-raw-empty-1440-light.png)；Clear value之后状态串。
- 现状 → 目标：布尔值、焦点事件、业务值、计数直接连在按钮后。 → G-3：请求数、最后请求、应用值清楚区分。
- 修法：反馈与按钮分离，具名显示接受值、请求、开关及取消结果。
- 评审来源：judge-u4-u5.raw.md，临时编号「新-4」。

### UI-13 · [P3] 无皮肤Select标签/Trigger连写（U5 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U5-raw-empty-1440-light.png](U5-raw-empty-1440-light.png)；Trigger及Policy/Entries。
- 现状 → 目标：Choose fruitOpen及相邻标签控件连写。 → G-2：标签、帮助和状态可读。
- 修法：增加最小空白或布局间距，不增加品牌样式。
- 评审来源：judge-u4-u5.raw.md，临时编号「新-5」。

### UI-14 · [P1] 无皮肤表单保存中/完成状态不明确（U6 · 可挽回 0.75）

- 状态：仍存在。
- 位置：[U6-raw-form-busy-1440-light.png](U6-raw-form-busy-1440-light.png)；Native fields状态区与Save。
- 现状 → 目标：历史字符串与120/230不能直接区分正在保存或已保存；失败有Try again但序列不清。 → G-2/U6-2：状态可读、消费者状态一致。
- 修法：历史之外输出独立具名Saving/Saved/Save failed状态，不声称发生重复提交。
- 评审来源：judge-u6.raw.md，临时编号「新-1」。

### UI-15 · [P2] 无皮肤选择反馈连写（U6 · 可挽回 0.75）

- 状态：仍存在。
- 位置：[U6-raw-refused-1440-light.png](U6-raw-refused-1440-light.png)；Controlled choices状态。
- 现状 → 目标：Some(a)1Some(b)、Indeterminate00等连续字符串需猜含义；A接受值保留已有证据。 → G-3：策略工具区分请求数、最后请求、应用值。
- 修法：用无主题标签与分行，标清对应控件。
- 评审来源：judge-u6.raw.md，临时编号「新-2」。

### UI-16 · [P3] 无皮肤相邻文案缺分隔（U6 · 可挽回 0.5）

- 状态：仍存在。
- 位置：[U6-raw-mobile-390-light.png](U6-raw-mobile-390-light.png)；Mixed checkbox/Notifications；Select Trigger。
- 现状 → 目标：mixed markerNotifications、BananaOpen等连写。 → G-2/U6-2：无皮肤标签仍可直接阅读。
- 修法：增加语义片段空白、换行或独立文本块，不引入皮肤。
- 评审来源：judge-u6.raw.md，临时编号「新-3」。

### UI-17 · [P2] 未完成数量草稿仅用红框表达（U1 · 可挽回 0.5）

- 状态：新发现。
- 位置：[U1-draft-native-before-a4-1440-light.png](U1-draft-native-before-a4-1440-light.png)；Quantity 输入框及帮助行，约x369–868、y484–516。
- 现状 → 目标：聚焦草稿为 - 时红框可见，帮助仍是通用提交说明，没有文字说明数字未完成。 → G-2：状态不只用颜色反馈，允许保留未完成 Draft。
- 修法：在现有帮助旁显示数字未完成及继续输入/取消提示，保持应用接受值，不提前提交 Draft。
- 评审来源：judge-supplement-u1.raw.md，临时编号「临时补-U1-1」。

## 建议（不扣分）

- 固定并分别采集 busy 与成功帧，补齐各单元评审原文所列的缺证；不把未拍状态计为通过或失败。
- 系统原生 validationMessage 的语言口径先确认；多行 FormData 可提供紧凑视图，但规范未要求具体密度。
- Number/Color Escape 后可回显取消结果或说明 Commit / cancel event 的统计范围。
- 接受延迟值后补走键盘焦点过程；仅凭初采 BODY 焦点记录不能认定键盘缺陷。
- raw 只需基本隔离底面、间距与具名反馈，无须增加品牌皮肤。


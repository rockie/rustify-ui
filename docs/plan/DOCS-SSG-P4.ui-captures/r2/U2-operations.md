# U2 本轮实际操作

- U2-loading-1280-dark.png：延迟首次索引响应，输入 CustomOnlyToken；保留输入并显示 loading。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"loading","input":"CustomOnlyToken","status":"Preparing search… You can keep typing.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-closed-01-1280-dark.png：索引仍延迟时真实按 Escape，焦点回到 Search。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":false,"dialogState":"closed","input":"CustomOnlyToken","status":"Preparing search… You can keep typing.","total":"","options":0,"focused":{"tag":"BUTTON","id":"","text":"SearchCtrl / Cmd K","searchInput":false}}

- U2-reopen-02-1280-dark.png：重新点击 Search，重取索引后显示保留查询结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-late-response-03-1280-dark.png：释放旧请求，再输入 ReachableCustomToken；当前结果仍为新查询。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"ReachableCustomToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-module-error-1280-dark.png：首次请求故障后输入 CustomOnlyToken，等候 error；查看 Retry。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"error","input":"CustomOnlyToken","status":"Search is unavailable. Please try again.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-module-retry-1280-dark.png：点击 Retry，恢复真实同源资源；保留 CustomOnlyToken 并显示结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-index-error-1280-dark.png：首次请求故障后输入 CustomOnlyToken，等候 error；查看 Retry。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"error","input":"CustomOnlyToken","status":"Search is unavailable. Please try again.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-index-retry-1280-dark.png：点击 Retry，恢复真实同源资源；保留 CustomOnlyToken 并显示结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-loading-375-light.png：延迟首次索引响应，输入 CustomOnlyToken；保留输入并显示 loading。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"loading","input":"CustomOnlyToken","status":"Preparing search… You can keep typing.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-module-error-375-light.png：首次请求故障后输入 CustomOnlyToken，等候 error；查看 Retry。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"error","input":"CustomOnlyToken","status":"Search is unavailable. Please try again.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-module-retry-375-light.png：点击 Retry，恢复真实同源资源；保留 CustomOnlyToken 并显示结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-index-error-375-light.png：首次请求故障后输入 CustomOnlyToken，等候 error；查看 Retry。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"error","input":"CustomOnlyToken","status":"Search is unavailable. Please try again.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-index-retry-375-light.png：点击 Retry，恢复真实同源资源；保留 CustomOnlyToken 并显示结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-open-before-module-01-375-light.png：点击 Search；首次 module HTTP 响应仍被扣留、尚未执行，输入已获焦。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"loading","input":"","status":"Preparing search… You can keep typing.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-loading-edit-02-375-light.png：module仍未返回，在loading输入CustomOnlyToken，再真实全选并键入ReachableCustomToken。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"loading","input":"ReachableCustomToken","status":"Preparing search… You can keep typing.","total":"","options":0,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-loading-result-03-375-light.png：释放module响应后，显示loading期间最终输入ReachableCustomToken的结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"ReachableCustomToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-longlist-04-375-light.png：改搜SearchCorpusToken，连续真实按ArrowDown19次至首批末项；列表滚动，输入、关闭与Show more仍在视口内。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"SearchCorpusToken","status":"","total":"98 results","options":20,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-retry-focus-05-375-light.png：module503后，输入获焦；真实按Tab到Retry，拍焦点帧。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"error","input":"CustomOnlyToken","status":"Search is unavailable. Please try again.","total":"","options":0,"focused":{"tag":"BUTTON","id":"","text":"Retry","searchInput":false}}

- U2-supp-keyboard-retry-06-375-light.png：真实按Enter激活Retry；输入恢复焦点且保留查询，结果显示。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-close-focus-07-375-light.png：从输入真实按Shift+Tab，焦点到Close。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":true,"dialogState":"results","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"BUTTON","id":"","text":"Close","searchInput":false}}

- U2-supp-close-return-08-375-light.png：真实按Enter激活Close，对话框关闭，焦点返回页头Search。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"light","dialogOpen":false,"dialogState":"closed","input":"CustomOnlyToken","status":"","total":"1 result","options":1,"focused":{"tag":"BUTTON","id":"","text":"SearchCtrl / Cmd K","searchInput":false}}

- U2-supp-new-result-before-old-09-1280-dark.png：旧首个JSON仍被扣留；关闭并重开后，新查询ReachableCustomToken结果已显示。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"ReachableCustomToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}

- U2-supp-old-released-after-new-10-1280-dark.png：新查询结果已显示之后才释放旧JSON，等250ms；仍显示ReachableCustomToken当前结果。
  - 实测记录：{"title":"Theme switching | Search corpus","htmlLang":"en","theme":"dark","dialogOpen":true,"dialogState":"results","input":"ReachableCustomToken","status":"","total":"1 result","options":1,"focused":{"tag":"INPUT","id":"rd-search-input","text":"","searchInput":true}}
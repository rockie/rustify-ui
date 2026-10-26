---
title: |-
  用 Rust，
  构建复杂界面。
description: 面向浏览器的 Rust UI SDK。组合 Leptos DOM 与可选的 Makepad GPU 区域，通过共享状态与类型化动作构建复杂界面。
layout: landing
index: true
tagline: DOM 处理界面与状态，GPU 按需绘制。让 Leptos 与 Makepad，在同一个 Rust 应用中协作。
actions:
  - text: 阅读中文文档
    link: https://rockie.github.io/rustify-ui/zh/
  - text: GitHub 源码
    link: https://github.com/rockie/rustify-ui
---

<div class="[body:has(&)_.rd-header-bar]:max-w-[84rem] [body:has(&)_.rd-header-bar]:min-h-20 [body:has(&)_.rd-header-bar]:px-6 lg:[body:has(&)_.rd-header-bar]:px-12 [body:has(&)_.rd-brand]:text-lg [body:has(&)_.rd-home]:max-w-[84rem] [body:has(&)_.rd-home]:px-6 lg:[body:has(&)_.rd-home]:px-12 [body:has(&)_.rd-home]:pt-12 md:[body:has(&)_.rd-home]:pt-16 [body:has(&)_.rd-home]:pb-0 [body:has(&)_.rd-hero]:max-w-[68rem] [body:has(&)_.rd-home-title]:max-w-[14ch] [body:has(&)_.rd-home-title]:text-[clamp(2.4rem,7.1vw,6rem)] [body:has(&)_.rd-home-title]:font-semibold [body:has(&)_.rd-home-title]:whitespace-pre-line [body:has(&)_.rd-home-title]:leading-[1.08] [body:has(&)_.rd-home-title]:tracking-[-0.04em] [body:has(&)_.rd-tagline]:max-w-[39rem] [body:has(&)_.rd-tagline]:mt-6 [body:has(&)_.rd-tagline]:text-lg md:[body:has(&)_.rd-tagline]:text-xl [body:has(&)_.rd-actions]:mt-8 [body:has(&)_.rd-actions]:grid [body:has(&)_.rd-actions]:grid-cols-2 [body:has(&)_.rd-actions]:max-w-[26rem] [body:has(&)_.rd-action]:min-h-14 [body:has(&)_.rd-action]:rounded-xl [body:has(&)_.rd-action]:justify-center [body:has(&)_.rd-action]:px-3 [body:has(&)_.rd-action]:text-sm sm:[body:has(&)_.rd-action]:px-6 [body:has(&)_.rd-action]:transition-colors motion-reduce:[body:has(&)_.rd-action]:transition-none [body:has(&)_.rd-home-prose]:max-w-none [body:has(&)_.rd-home-prose]:mt-8 [&_h2]:mt-0 [&_h2]:mb-0 [&_h2]:border-0 [&_h2]:pb-0 [&_h3]:mt-0 [&_h3]:mb-0 [&_p]:my-0 [&_a]:no-underline [&_a:hover]:underline [&_figure]:m-0 [&_img]:m-0 [&_ul]:m-0 [&_ul]:list-none [&_ul]:p-0">
  <p class="flex flex-wrap gap-x-5 gap-y-2 text-xs font-medium text-[#596780] sm:text-sm"><span>实验性 Rust SDK</span><span>默认 DOM</span><span>GPU 可选</span></p>

  <section aria-label="DOM 与 GPU 如何协作" class="group/collaboration relative mt-12 overflow-hidden rounded-2xl bg-[#2348e5] text-white md:mt-14">
    <div class="grid grid-cols-2 gap-5 px-6 pt-8 pb-8 md:grid-cols-[1fr_auto_1fr] md:gap-8 md:items-start md:px-12 md:pt-10 md:pb-12">
      <div>
        <div class="flex flex-col gap-2 md:flex-row md:items-baseline md:gap-3"><span class="whitespace-nowrap text-[clamp(2.25rem,8vw,6rem)] font-semibold leading-none tracking-[-0.04em]">DOM</span><span class="text-sm text-[#d8e0ff]">Leptos</span></div>
        <p class="mt-4! max-w-[29ch] text-xs leading-relaxed sm:text-sm text-[#e3e9ff]">处理布局、表单与响应式应用状态。</p>
      </div>
      <div class="hidden items-center gap-4 self-center md:flex md:flex-col md:gap-3" aria-hidden="true"><span class="h-px w-12 bg-[#8ca2ff] transition-colors duration-200 group-hover/collaboration:bg-[#c9f3dd] group-focus-within/collaboration:bg-[#c9f3dd] motion-reduce:transition-none md:h-10 md:w-px"></span><span class="text-3xl font-light text-[#d8e0ff]">+</span><span class="h-px w-12 bg-[#8ca2ff] transition-colors duration-200 group-hover/collaboration:bg-[#c9f3dd] group-focus-within/collaboration:bg-[#c9f3dd] motion-reduce:transition-none md:h-10 md:w-px"></span></div>
      <div class="md:pl-8">
        <div class="flex flex-col gap-2 md:flex-row md:items-baseline md:gap-3"><span class="whitespace-nowrap text-[clamp(2.25rem,8vw,6rem)] font-semibold leading-none tracking-[-0.04em] text-[#c9f3dd]">GPU</span><span class="text-sm text-[#d8e0ff]">Makepad</span></div>
        <p class="mt-4! max-w-[29ch] text-xs leading-relaxed sm:text-sm text-[#e3e9ff]">为需要 GPU 绘制的内容，按需启用 WebGL2 区域。</p>
      </div>
    </div>
    <div class="flex flex-wrap items-center justify-between gap-3 border-t border-[#5674ed] px-6 py-5 text-sm md:px-12"><span>共享状态 · 类型化动作 · 同一个应用</span><a class="inline-flex min-h-8 items-center gap-2 font-medium text-white! underline-offset-4 focus-visible:outline-[#c9f3dd] focus-visible:outline-offset-4" href="https://rockie.github.io/rustify-ui/zh/architecture/">了解协作机制<svg aria-hidden="true" class="h-4 w-4 transition-transform duration-200 ease-out motion-safe:group-hover/collaboration:translate-x-1 motion-safe:group-focus-within/collaboration:translate-x-1 motion-reduce:transition-none" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M5 12h14m-5-5 5 5-5 5"/></svg></a></div>
  </section>

  <section aria-labelledby="built-for-the-real-work" class="py-20 md:py-28">
    <div class="grid gap-6 md:grid-cols-[1fr_1fr] md:gap-16">
      <h2 id="built-for-the-real-work" class="max-w-[15ch] text-3xl! font-semibold! leading-tight! tracking-[-0.03em] md:text-4xl!">原生浏览器体验，<br>按你的方式组合。</h2>
      <p class="max-w-[49ch] text-base leading-relaxed text-[#4a5870]">从 DOM 应用开始，在需要时加入 GPU 区域。Rustify 负责两者之间的状态、生命周期、输入与浮层协作。</p>
    </div>
    <div class="mt-12 divide-y divide-[#ccd5e5] border-y border-[#ccd5e5]">
      <div class="grid gap-4 py-7 md:grid-cols-[0.7fr_1.3fr] md:gap-16"><h3 class="text-lg! font-semibold!">DOM，负责界面与状态。</h3><p class="max-w-[59ch] text-sm leading-relaxed text-[#4a5870]">用 DOM 组件构建表单、导航、文本编辑与大型表格，由 Leptos 管理布局和响应式应用状态。</p></div>
      <div class="grid gap-4 py-7 md:grid-cols-[0.7fr_1.3fr] md:gap-16"><h3 class="text-lg! font-semibold!">GPU，专注需要绘制的区域。</h3><p class="max-w-[59ch] text-sm leading-relaxed text-[#4a5870]">将 Makepad 挂载到 DOM 定位的区域，传入 props，接收类型化动作；应用状态由一处统一管理。</p></div>
      <div class="grid gap-4 py-7 md:grid-cols-[0.7fr_1.3fr] md:gap-16"><h3 class="text-lg! font-semibold!">一套主题，贯穿两端。</h3><p class="max-w-[59ch] text-sm leading-relaxed text-[#4a5870]">跨渲染器共享语义主题值，为应用的不同区域隔离主题，同时协调控件、焦点与浮层。</p></div>
    </div>
  </section>

  <section aria-labelledby="from-components-to-workspaces" class="rounded-2xl bg-[#e7edf8] px-6 py-10 md:px-12 md:py-14">
    <div class="flex flex-wrap items-end justify-between gap-6">
      <h2 id="from-components-to-workspaces" class="max-w-[18ch] text-3xl! font-semibold! leading-tight! tracking-[-0.03em] md:text-4xl!">看组件，<br>也看它们如何协作。</h2>
      <a class="inline-flex min-h-11 items-center gap-3 text-sm font-semibold text-[#173dc8]! underline-offset-4" href="https://rockie.github.io/rustify-ui/zh/themes/">了解 Theme Studio<svg aria-hidden="true" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M5 12h14m-5-5 5 5-5 5"/></svg></a>
    </div>
    <figure class="mt-8!">
      <div class="overflow-hidden rounded-xl bg-white"><img class="block aspect-[16/10] w-full object-cover object-top" src="theme-studio.png" alt="Rustify Theme Studio 真实截图：DOM 主题编辑器、原生表单与 Makepad GPU 预览并列展示。" loading="lazy" width="1440" height="1000"></div>
      <figcaption class="mt-4 text-xs leading-relaxed text-[#4a5870]">Theme Studio 真实示例：基于 Rustify UI 构建，将 DOM 编辑与 GPU 预览放在同一界面中。</figcaption>
    </figure>
    <div class="mt-8 flex flex-wrap gap-x-8 gap-y-4 border-t border-[#c7d2e6] pt-6 text-sm"><a class="min-h-8 content-center font-medium text-[#173dc8]!" href="https://rockie.github.io/rustify-ui/zh/components/button/">浏览组件</a><a class="min-h-8 content-center font-medium text-[#173dc8]!" href="https://rockie.github.io/rustify-ui/zh/vellum/">了解 Vellum 编辑器</a><a class="min-h-8 content-center font-medium text-[#173dc8]!" href="https://github.com/rockie/rustify-ui/tree/main/examples">在 GitHub 查看全部示例</a></div>
  </section>

  <section aria-labelledby="start-with-a-region" class="grid gap-10 py-20 md:grid-cols-2 md:gap-16 md:py-28">
    <div><h2 id="start-with-a-region" class="max-w-[15ch] text-3xl! font-semibold! leading-tight! tracking-[-0.03em] md:text-4xl!">从一个小示例，<br>开始你的应用。</h2><p class="mt-6! max-w-[43ch] text-sm leading-relaxed text-[#4a5870]">从共享计数器到设计编辑器，仓库提供可运行的示例。先运行最小示例，再沿着指南接入自己的应用。</p><a class="mt-6 inline-flex min-h-11 items-center gap-3 text-sm font-semibold text-[#173dc8]! underline-offset-4" href="https://rockie.github.io/rustify-ui/zh/quickstart/">阅读快速上手<svg aria-hidden="true" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M5 12h14m-5-5 5 5-5 5"/></svg></a></div>
    <div class="min-w-0 overflow-hidden rounded-xl bg-[#16243b] text-[#eef3ff]"><div class="flex items-center justify-between border-b border-[#3c4a60] px-6 py-4 text-xs"><span>运行最小示例</span><span class="text-[#b4c2d9]">shell</span></div><pre tabindex="0" role="region" aria-label="克隆并运行最小示例的命令" class="m-0! overflow-x-auto border-0! bg-transparent! p-6! text-[13px]! leading-7! text-[#eef3ff]!"><code class="bg-transparent! text-inherit!">git clone https://github.com/rockie/rustify-ui.git
cd rustify-ui
mise install
mbx xtask build-web --example fusion-basic --release
mbx xtask serve --example fusion-basic --release</code></pre><p class="border-t border-[#3c4a60] px-6 py-4 text-xs leading-relaxed text-[#b4c2d9]">已在 macOS Chrome 验证。环境配置与浏览器支持范围见文档。</p></div>
  </section>

  <section aria-labelledby="your-next-interface" class="flex flex-wrap items-center justify-between gap-8 rounded-2xl bg-[#c9f3dd] px-6 py-10 md:px-12 md:py-14"><div><h2 id="your-next-interface" class="max-w-[18ch] text-3xl! font-semibold! leading-tight! tracking-[-0.03em] md:text-4xl!">下一个界面，<br>用 Rust 开始。</h2><p class="mt-4! text-sm leading-relaxed text-[#294f41]">也可以用测试信息，体验一次 CTA 表单提交。</p></div><a class="inline-flex min-h-14 items-center gap-4 rounded-xl bg-[#16243b] px-6 text-sm font-semibold text-white! transition-colors motion-reduce:transition-none hover:bg-[#30425e]" href="cta.md">测试 CTA 表单<svg aria-hidden="true" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M5 12h14m-5-5 5 5-5 5"/></svg></a></section>

  <footer class="mt-16 flex flex-wrap items-center justify-between gap-6 border-t border-[#ccd5e5] py-8 text-xs leading-relaxed text-[#596780]"><p>Rustify UI · 面向 Web 的实验性 Rust SDK。</p><nav class="flex flex-wrap gap-6" aria-label="项目链接"><a class="inline-flex min-h-8 items-center text-[#34435b]!" href="https://rockie.github.io/rustify-ui/zh/">GitHub Pages 文档站</a><a class="inline-flex min-h-8 items-center text-[#34435b]!" href="https://github.com/rockie/rustify-ui">GitHub</a></nav></footer>
</div>

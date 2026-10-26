---
title: 体验 CTA 表单
description: 使用测试信息，体验校验、提交与返回完成页的完整流程。
layout: landing
landing: cta-test
tagline: 填写示例项目和测试邮箱，走一遍真实的表单提交流程。
---

<div class="[body:has(&)_.rd-header-bar]:max-w-[58rem] [body:has(&)_.rd-header-bar]:min-h-20 [body:has(&)_.rd-header-bar]:px-6 [body:has(&)_.rd-home]:max-w-[58rem] [body:has(&)_.rd-home]:px-6 [body:has(&)_.rd-home]:py-10 [body:has(&)_.rd-hero]:max-w-none [body:has(&)_.rd-home-title]:text-4xl [body:has(&)_.rd-home-title]:font-semibold [body:has(&)_.rd-home-prose]:max-w-none [body:has(&)_.rd-home-prose]:mt-8 [&_p]:my-0 [&_a]:no-underline [&_a:hover]:underline">
  <div class="rounded-2xl bg-[#e7edf8] p-5 text-sm leading-relaxed text-[#34435b]"><p>这是本地演示，请使用示例信息，如 <strong>demo@example.test</strong>。提交内容只保留在本机测试接收器的内存中，不会发送邮件。</p></div>
  <form id="cta-test-form" aria-label="CTA 测试表单" method="post" action="http://127.0.0.1:4211/lead" class="mt-6 rounded-2xl border border-[#ccd5e5] bg-white p-6 sm:p-8">
    <div class="grid gap-5 sm:grid-cols-2">
      <div class="min-w-0">
        <label for="cta-project" class="mb-2 block text-sm font-semibold">测试项目名称</label>
        <input id="cta-project" name="project" type="text" required maxlength="80" autocomplete="off" placeholder="例如：Rustify 示例" aria-describedby="cta-project-error" class="h-14 w-full rounded-xl border border-[#b7c2d7] bg-white px-4 text-base text-[#16243b] outline-offset-2 placeholder:text-[#596780] focus-visible:outline-3 focus-visible:outline-[#2348e5] aria-invalid:border-[#b42318]">
        <p id="cta-project-error" class="mt-2! min-h-5 text-xs leading-5 text-[#b42318]"></p>
      </div>
      <div class="min-w-0">
        <label for="cta-email" class="mb-2 block text-sm font-semibold">测试邮箱</label>
        <input id="cta-email" name="email" type="email" required maxlength="254" autocomplete="off" placeholder="demo@example.test" aria-describedby="cta-email-error" class="h-14 w-full rounded-xl border border-[#b7c2d7] bg-white px-4 text-base text-[#16243b] outline-offset-2 placeholder:text-[#596780] focus-visible:outline-3 focus-visible:outline-[#2348e5] aria-invalid:border-[#b42318]">
        <p id="cta-email-error" class="mt-2! min-h-5 text-xs leading-5 text-[#b42318]"></p>
      </div>
    </div>
    <div class="mt-6 flex flex-wrap items-center gap-5"><button type="submit" class="inline-flex min-h-14 items-center justify-center rounded-xl bg-[#2348e5] px-6 text-sm font-semibold text-white transition-colors hover:bg-[#173dc8] disabled:cursor-wait disabled:opacity-60 motion-reduce:transition-none">提交测试信息</button><a class="inline-flex min-h-11 items-center text-sm font-medium text-[#173dc8]!" href="index.md">返回项目介绍</a></div>
    <p id="cta-status" role="status" aria-live="polite" class="mt-4! min-h-6 text-sm leading-6 text-[#4a5870]"></p>
  </form>
  <p class="mt-6! text-xs leading-relaxed text-[#596780]">此示例需要先启动本地测试接收器。停止接收器后，内存中的测试记录会被清空。</p>
</div>

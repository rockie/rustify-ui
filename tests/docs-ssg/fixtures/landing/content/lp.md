---
title: Ship docs in an afternoon
description: A landing page for paid traffic.
layout: landing
landing: ads
tagline: Markdown in, a fast static site out.
actions:
  - text: Get started
    link: "#lead"
  - text: Read the guide
    link: guide.md
features:
  - title: Strict by default
    details: Every other page keeps the strict policy.
  - title: Measured
    details: The profile's script reports each visit.
  - title: Carried
    details: Advertising parameters reach the form.
---

## Details

Everything below the hero is ordinary Markdown. See the
<a data-rd-carry href="https://example.org/signup">partner sign-up</a> too.

## Request a demo

<form id="lead" class="grid max-w-md gap-3" method="post" action="http://127.0.0.1:4211/lead">
<label for="lead-email">Work email</label>
<input id="lead-email" name="email" type="email" autocomplete="email" required class="rounded-md border border-secondary px-3 py-2">
<input type="hidden" name="gclid" data-rd-carry>
<input type="hidden" name="utm_source" data-rd-carry>
<input type="hidden" name="utm_campaign" data-rd-carry>
<button type="submit" class="rd-action rd-action-primary">Request a demo</button>
</form>

## Or use the embedded form

<iframe data-rd-carry title="Embedded sign-up form" src="http://127.0.0.1:4211/embed" loading="lazy" class="h-64 w-full max-w-md rounded-md border border-secondary"></iframe>

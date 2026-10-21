---
title: Where a form submits
description: The forms on this page, and where each of them submits.
layout: landing
landing: ads
---

## A form that submits to this page

<form id="query" method="get" action="?campaign=paid">
<label for="query-q">Search</label>
<input id="query-q" name="q" value="boots">
<button type="submit">Search this page</button>
</form>

## A form that submits to the thanks page

<form id="thanks" method="post" action="/thanks/?campaign=paid">
<label for="thanks-email">Your email</label>
<input id="thanks-email" name="email" type="email" autocomplete="email" required>
<button type="submit">Request a demo</button>
</form>

## A button that submits elsewhere

<form id="button" method="get" action="/thanks/">
<label for="button-q">Search</label>
<input id="button-q" name="q" value="boots">
<button type="submit" formaction="?campaign=paid">Search from the button</button>
</form>

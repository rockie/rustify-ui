---
title: Client-only counter
description: A declared fixed-size placeholder before the browser creates the view.
---

# Client-only counter

The server publishes the declaration's fixed-size placeholder. The same typed props and child Markdown
reach the real counter after attachment.

```rustify-component
name: BrowserCounter
props:
  initial: 2
children: |
  This **browser-created child** remains readable after mounting.
```

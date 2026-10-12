# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/theme.spec.ts >> reading quality >> text, links, alerts and actions reach AA contrast in both modes
- Location: tests/docs-ssg/theme.spec.ts:375:9

# Error details

```
Error: page.evaluate: TypeError: Failed to execute 'getComputedStyle' on 'Window': parameter 1 is not of type 'Element'.
    at eval (eval at evaluate (:311:30), <anonymous>:35:22)
    at UtilityScript.evaluate (<anonymous>:313:16)
    at UtilityScript.<anonymous> (<anonymous>:1:44)
```

# Page snapshot

```yaml
- generic [active] [ref=f1e1]:
  - link "Skip to content" [ref=f1e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f1e3]:
    - generic [ref=f1e4]:
      - link "Fixture Docs" [ref=f1e5] [cursor=pointer]:
        - /url: /
      - navigation "Site links" [ref=f1e7]:
        - list [ref=f1e8]:
          - listitem [ref=f1e9]:
            - link "Guide" [ref=f1e10] [cursor=pointer]:
              - /url: /guide/
          - listitem [ref=f1e11]:
            - link "GitHub" [ref=f1e12] [cursor=pointer]:
              - /url: https://github.com/example/fixture
        - list [ref=f1e13]:
          - listitem [ref=f1e14]:
            - link "Mastodon" [ref=f1e15] [cursor=pointer]:
              - /url: https://mastodon.example/@fixture
      - group "Theme" [ref=f1e16]:
        - button "System" [pressed] [ref=f1e17] [cursor=pointer]
        - button "Light" [ref=f1e23] [cursor=pointer]
        - button "Dark" [ref=f1e29] [cursor=pointer]
  - main [ref=f1e34]:
    - generic [ref=f1e35]:
      - heading "Fixture Docs" [level=1] [ref=f1e36]
      - paragraph [ref=f1e37]: Markdown in, a static site out.
      - paragraph [ref=f1e38]:
        - link "Get started" [ref=f1e39] [cursor=pointer]:
          - /url: /guide/
        - link "Reference" [ref=f1e40] [cursor=pointer]:
          - /url: /reference/api/
    - list [ref=f1e41]:
      - listitem [ref=f1e42]:
        - heading "Static" [level=2] [ref=f1e43]
        - paragraph [ref=f1e44]: Every page is complete HTML.
      - listitem [ref=f1e45]:
        - heading "Checked" [level=2] [ref=f1e46]
        - paragraph [ref=f1e47]: Every link is resolved before a build is published.
    - article [ref=f1e48]:
      - paragraph [ref=f1e49]:
        - text: Start with the
        - link "guide" [ref=f1e50] [cursor=pointer]:
          - /url: /guide/
        - text: or read the
        - link "API reference" [ref=f1e51] [cursor=pointer]:
          - /url: /reference/api/
        - text: .
  - paragraph [ref=f1e52]
```
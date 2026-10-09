# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-runtime.spec.ts >> a modified seed is rejected before installation and explicit retry keeps committed styles
- Location: tests/browser/utility-runtime.spec.ts:77:5

# Error details

```
Error: page.evaluate: SyntaxError: Unexpected token 'a', "asset dige"... is not valid JSON
    at JSON.parse (<anonymous>)
    at eval (eval at evaluate (:311:30), <anonymous>:8:22)
    at async <anonymous>:337:30
```

# Page snapshot

```yaml
- main [ref=e2]:
  - generic [ref=e3]:
    - heading "Utility Lab" [level=1] [ref=e4]
    - paragraph [ref=e5]: Apply complete class names to a live, isolated preview.
  - generic [ref=e6]:
    - generic [ref=e7]: Classes
    - textbox "Classes" [ref=e8]: w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]
    - generic [ref=e9]:
      - button "Apply classes" [ref=e10] [cursor=pointer]
      - button "Restart compiler" [ref=e11] [cursor=pointer]
      - button "Copy CSS" [ref=e12] [cursor=pointer]
    - generic [ref=e13]:
      - generic [ref=e14]:
        - checkbox "Apply supported GPU properties only" [ref=e15]
        - text: Apply supported GPU properties only
      - generic [ref=e16]:
        - checkbox "Disable sample button" [ref=e17]
        - text: Disable sample button
    - group [ref=e18]:
      - generic "Container, text and color block classes" [ref=e19] [cursor=pointer]
    - status [ref=e20]: Ready. Apply classes to update the preview.
  - group [ref=e21]:
    - generic "Scan source text" [ref=e22] [cursor=pointer]
  - generic [ref=e23]:
    - region [ref=e24]:
      - heading "DOM preview" [level=2] [ref=e25]
      - paragraph [ref=e26]: Updates after the generated stylesheet is installed.
      - generic [ref=e30]:
        - button "Live utility sample" [ref=e31]
        - generic [ref=e32]: Utility text
    - region [ref=e34]:
      - heading "GPU preview" [level=2] [ref=e35]
      - paragraph [ref=e36]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e38]:
        - status [ref=e40]: GPU ready
        - paragraph [ref=e41]: "Button: applied · applied 0 · drawn 0 · 0 clicks"
        - paragraph [ref=e42]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e43]:
          - generic "GPU values and diagnostics" [ref=e44] [cursor=pointer]
  - region [ref=e45]:
    - heading "Independent scope" [level=2] [ref=e46]
    - paragraph [ref=e47]: Uses its own compiler and stylesheet.
    - generic [ref=e50]:
      - button "Live utility sample" [ref=e51]
      - generic [ref=e52]: Utility text
  - group [ref=e54]:
    - generic "Generated CSS and diagnostics" [ref=e55] [cursor=pointer]
  - generic [ref=e56]: Outside every utility scope
```
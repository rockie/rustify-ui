# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> applied utility dependencies drive DOM and GPU and survive an installation failure
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: page.evaluate: SyntaxError: Unexpected token 'S', "SyntaxErro"... is not valid JSON
    at JSON.parse (<anonymous>)
    at eval (eval at evaluate (:311:30), <anonymous>:9:19)
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
  - generic [ref=e21]:
    - region [ref=e22]:
      - heading "DOM preview" [level=2] [ref=e23]
      - paragraph [ref=e24]: Updates after the generated stylesheet is installed.
      - generic [ref=e28]:
        - button "Live utility sample" [ref=e29]
        - generic [ref=e30]: Utility text
    - region [ref=e32]:
      - heading "GPU preview" [level=2] [ref=e33]
      - paragraph [ref=e34]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e36]:
        - status [ref=e38]: GPU ready
        - paragraph [ref=e39]: "Button: applied · applied 1 · drawn 1 · 0 clicks"
        - paragraph [ref=e40]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e41]:
          - generic "GPU values and diagnostics" [ref=e42] [cursor=pointer]
  - region [ref=e43]:
    - heading "Independent scope" [level=2] [ref=e44]
    - paragraph [ref=e45]: Uses its own compiler and stylesheet.
    - generic [ref=e48]:
      - button "Live utility sample" [ref=e49]
      - generic [ref=e50]: Utility text
  - group [ref=e52]:
    - generic "Generated CSS and diagnostics" [ref=e53] [cursor=pointer]
  - generic [ref=e54]: Outside every utility scope
```
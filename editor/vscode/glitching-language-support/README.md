# Glitching VS Code Language Support

This extension adds syntax highlighting and editor language configuration for
Glitching `.gl` files.

It is intentionally lightweight:

- `.gl` files are registered as the `glitching` language
- C# TextMate scopes are reused through the Microsoft C# extension
- Glitching-specific syntax is added on top for:
  - `fn`, `let`, `move`, `borrow`, `package`, `native`, `macro`
  - ownership wrappers such as `shared<T>` and `view<T>`
  - `#` comments used by some legacy `.gl` sources
  - upper-case macro calls such as `XUNIT_FACT(...)`
  - multiline `native " ... ";` blocks

## Local install

1. Open this folder in VS Code:

   `D:\Repos\Glitching\editor\vscode\glitching-language-support`

2. Run:

   `Extensions: Install from VSIX...`

If you want a packaged `.vsix`, build it with `vsce` from this directory.

## Scope

This is syntax highlighting only. It does not provide:

- Glitching diagnostics
- completion
- go to definition
- build tasks

Those need a Glitching language server or explicit VS Code task integration.

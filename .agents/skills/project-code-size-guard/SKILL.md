---
name: project-code-size-guard
description: "项目代码行数守卫。用于本 Tauri/Leptos 项目中任何修改 src 或 src-tauri/src 代码后的收尾检查；要求保持文件职责分明，检查每个源文件行数，超过 300 行提示考虑拆分，超过 500 行必须拆分后再完成。"
---

# Project Code Size Guard

## Core Rules

Use this skill after every code change that touches `src` or `src-tauri/src`.

- Keep files in `src` and `src-tauri/src` clearly separated by responsibility.
- Prefer small, purpose-specific modules over large mixed-responsibility files.
- Treat 300 lines as a soft warning: explain whether the file should be split now or why keeping it together is still acceptable.
- Treat 500 lines as a hard warning: split the file before calling the task complete.
- Run the bundled line-count checker after finishing edits and include the result in the final response.

## Required Check

From the project root, run:

```powershell
python .agents\skills\project-code-size-guard\scripts\check_line_counts.py
```

The checker scans only:

- `src`
- `src-tauri/src`

Exit behavior:

- `0`: no file exceeds 500 lines.
- `1`: at least one file exceeds 500 lines and must be split.

## Refactoring Guidance

When a file crosses 300 lines, inspect its responsibilities before deciding:

- Extract UI components, hooks/state helpers, view models, constants, or style/data definitions from frontend files.
- Extract Tauri commands, domain services, path/runtime helpers, DTOs, error types, and tests into focused Rust modules.
- Keep public APIs narrow and use local module names that describe the responsibility being extracted.

When a file crosses 500 lines, do not finish with only a note. Split it as part of the change, then rerun the checker.

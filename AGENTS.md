# AGENTS.md

## 项目代码行数守卫

- `src` 和 `src-tauri/src` 内的文件必须职责分明，避免把 UI、状态、命令、服务、路径、运行时等不同责任混在同一个大文件里。
- 每次代码改动完成后，必须运行 `.agents/skills/project-code-size-guard/scripts/check_line_counts.py` 检查 `src` 和 `src-tauri/src` 的文件行数。
- 超过 300 行是弱警告：需要判断是否有必要拆分，并在最终说明里交代处理结果。
- 超过 500 行是强警告：必须拆分后才能完成任务。

运行方式：

```powershell
python .agents\skills\project-code-size-guard\scripts\check_line_counts.py
```

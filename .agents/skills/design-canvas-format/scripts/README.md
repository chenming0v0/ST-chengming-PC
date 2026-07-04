# design-canvas-format scripts

维护 `chenmumu5-redesign-mobile/chenmumu5-redesign-mobile.design` 的 Python 工具。  
**逻辑真源**：`chenmumu5-redesign-mobile/interactions.md`

可从仓库任意目录运行；默认 `.design` 路径由 `design_common.default_design_path()` 解析。

## 文件

| 文件 | 说明 |
|------|------|
| `design_common.py` | 共享：读写 JSON、页面列表、`LEGACY_DOM_IDS`、`REMOVED_PAGE_IDS` |
| `dump_interactions.py` | 导出 interactions |
| `validate_interactions.py` | 校验（exit 0/1/2） |
| `strip_legacy_interactions.py` | 清理废弃 domId |
| `apply_canonical_code_interactions.py` | 重置 code 区六页 interactions |

## CLI

### dump_interactions.py

```
--design PATH   默认 chenmumu5-redesign-mobile/chenmumu5-redesign-mobile.design
--page PREFIX   只输出 page id 以 PREFIX 开头的页
--markdown      输出 Markdown 表格
```

### validate_interactions.py

```
--design PATH
```

### strip_legacy_interactions.py / apply_canonical_code_interactions.py

```
--design PATH
--dry-run       不写文件，只打印将发生的变更
```

## 推荐维护顺序（代码区）

```bash
python .agents/skills/design-canvas-format/scripts/strip_legacy_interactions.py
python .agents/skills/design-canvas-format/scripts/apply_canonical_code_interactions.py
python .agents/skills/design-canvas-format/scripts/validate_interactions.py
```

## 给 AI

- 改 domId 或新增页面：先更新 `interactions.md`，再改 HTML + `.design`，最后 `validate_interactions.py`。
- 不要为抽屉入口添加 `data-dom-id`（见 `interactions.md` §1.2）。

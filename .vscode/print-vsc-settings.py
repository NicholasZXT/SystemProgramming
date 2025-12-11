# .vscode/print-vs-settings.py
import json
import os
import sys
import re

# 自定义配置前缀
CUSTOM_PREFIX = "custom."

def remove_jsonc_comments(json_str):
    """移除 JSONC 中的行注释 (// ...) 和尾随逗号"""
    # 移除行注释（注意：不能破坏字符串内的 //）
    lines = json_str.splitlines()
    cleaned_lines = []
    for line in lines:
        # 找到第一个未被引号包围的 //
        in_string = False
        escape_next = False
        comment_start = -1
        for i, c in enumerate(line):
            if escape_next:
                escape_next = False
                continue
            if c == '\\':
                escape_next = True
            elif c == '"':
                in_string = not in_string
            elif not in_string and c == '/' and i + 1 < len(line) and line[i + 1] == '/':
                comment_start = i
                break
        if comment_start != -1:
            line = line[:comment_start]
        cleaned_lines.append(line)
    cleaned = '\n'.join(cleaned_lines)

    # 移除尾随逗号（在 ] 或 } 前的逗号）
    cleaned = re.sub(r',\s*([\]}])', r'\1', cleaned)
    return cleaned


def load_settings_json(settings_path):
    """安全加载 .vscode/settings.json（支持 JSONC）"""
    if not os.path.isfile(settings_path):
        print("❌ Error: .vscode/settings.json not found!", file=sys.stderr)
        print("💡 Please create it with your custom config, e.g.:")
        print('{\n  "myApp.name": "MyProject",\n  "myApp.version": "1.0.0"\n}')
        return None

    try:
        with open(settings_path, 'r', encoding='utf-8') as f:
            raw_content = f.read()
    except Exception as e:
        print(f"❌ Error reading file: {e}", file=sys.stderr)
        return None

    try:
        # 尝试直接解析（如果无注释）
        return json.loads(raw_content)
    except json.JSONDecodeError:
        pass  # 继续尝试清理后解析

    try:
        cleaned_content = remove_jsonc_comments(raw_content)
        return json.loads(cleaned_content)
    except json.JSONDecodeError as e:
        print(f"❌ Failed to parse settings.json: {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"💥 Unexpected error: {e}", file=sys.stderr)
        return None


def main():
    # 工作区根目录（tasks.json 运行时 cwd 是 workspaceFolder）
    workspace_root = os.getcwd()
    settings_path = os.path.join(workspace_root, '.vscode', 'settings.json')

    settings = load_settings_json(settings_path)
    if settings is None:
        sys.exit(1)

    # 提取以 CUSTOM_PREFIX 开头的键
    custom_settings = {}
    for key, value in settings.items():
        if isinstance(key, str) and key.startswith(CUSTOM_PREFIX):
            short_key = key[len(CUSTOM_PREFIX):]
            custom_settings[short_key] = value

    if not custom_settings:
        print(f'ℹ️ No settings found with prefix "{CUSTOM_PREFIX}"')
        print('Example usage in settings.json:')
        print(f'{{ "{CUSTOM_PREFIX}name": "MyProject" }}')
        return

    # 打印结果
    print("=== VS Code Custom Settings ===")
    for key in sorted(custom_settings.keys()):
        value = custom_settings[key]
        # 对字符串加引号，其他类型直接转字符串
        if isinstance(value, str):
            display_value = f'"{value}"'
        else:
            display_value = str(value)
        print(f"{key.ljust(12)}: {display_value}")


if __name__ == "__main__":
    main()

// .vscode/print-vs-settings.js
// 此脚本需要用户安装Node.js，不能使用VS-Code内置的node.js运行时环境执行
const fs = require('fs');
const path = require('path');

// 获取工作区根目录（VS Code 启动任务时，cwd 是 workspaceFolder）
const workspaceRoot = process.cwd();
const settingsPath = path.join(workspaceRoot, '.vscode', 'settings.json');

function printCustomSettings() {
  try {
    // 检查文件是否存在
    if (!fs.existsSync(settingsPath)) {
      console.error('❌ .vscode/settings.json not found!');
      console.log('💡 Please create it with your custom config, e.g.:');
      console.log('{\n  "myApp.name": "MyProject",\n  "myApp.version": "1.0.0"\n}');
      return;
    }

    // 读取并解析 JSON（支持注释的 JSONC）
    const rawContent = fs.readFileSync(settingsPath, 'utf8');
    
    // 简单移除注释和尾随逗号（VS Code 的 settings.json 是 JSONC）
    const jsonStr = rawContent
      .replace(/\/\/.*$/gm, '')          // 移除行注释
      .replace(/\/\*[\s\S]*?\*\//g, '')  // 移除块注释
      .replace(/,\s*([\]}])/g, '$1');    // 移除尾随逗号

    let settings;
    try {
      settings = JSON.parse(jsonStr);
    } catch (e) {
      console.error('❌ Failed to parse settings.json:', e.message);
      return;
    }

    // 自定义配置前缀
    const CUSTOM_PREFIX = 'custom.';

    // 过滤出以 CUSTOM_PREFIX 开头的配置项
    const customSettings = {};
    for (const key in settings) {
      if (key.startsWith(CUSTOM_PREFIX)) {
        const shortKey = key.substring(CUSTOM_PREFIX.length);
        customSettings[shortKey] = settings[key];
      }
    }

    if (Object.keys(customSettings).length === 0) {
      console.log(`ℹ️ No settings found with prefix "${CUSTOM_PREFIX}"`);
      console.log('Example usage in settings.json:');
      console.log(`{ "${CUSTOM_PREFIX}name": "MyProject" }`);
      return;
    }

    // 打印结果
    console.log('=== VS Code Custom Settings ===');
    for (const [key, value] of Object.entries(customSettings)) {
      // 对布尔值/数字做友好显示
      const displayValue = typeof value === 'string' ? `"${value}"` : String(value);
      console.log(`${key.padEnd(12)}: ${displayValue}`);
    }

  } catch (err) {
    console.error('💥 Unexpected error:', err.message);
  }
}

printCustomSettings();

# Tech Fortune 安装和设置指南 / Installation and Setup Guide

## 快速开始 / Quick Start

### 1. 安装工具 / Install Tool

工具已经安装到你的系统中，你可以直接使用：

The tool has been installed to your system, you can use it directly:

```bash
tech-fortune
```

### 2. 语言设置 / Language Setup

首次使用时，工具会提示你选择语言：

On first use, the tool will prompt you to select a language:

```bash
🌍 请选择语言 / Please select language:
1. 中文 (Chinese)
2. English

请输入选择 (Enter choice): 1
```

你也可以随时更改语言设置：

You can also change the language setting anytime:

```bash
# 设置默认语言 / Set default language
tech-fortune --set-language

# 临时使用不同语言 / Temporarily use different language
tech-fortune --language en
```

### 3. 设置自动显示（可选）/ Setup Auto Display (Optional)

如果你希望每次打开终端时自动显示今日运势，请按照以下步骤操作：

If you want to automatically display today's fortune every time you open the terminal, follow these steps:

#### 对于 zsh 用户（macOS 默认）/ For zsh users (macOS default)

1. 打开你的 zsh 配置文件：
   Open your zsh configuration file:
```bash
nano ~/.zshrc
```

2. 在文件末尾添加以下行：
   Add the following line at the end of the file:
```bash
# 自动显示技术运势 / Auto display tech fortune
source /Users/martin/Develop/findu/auto-fortune.sh
```

3. 保存并重新加载配置：
   Save and reload the configuration:
```bash
source ~/.zshrc
```

#### 对于 bash 用户 / For bash users

1. 打开你的 bash 配置文件：
   Open your bash configuration file:
```bash
nano ~/.bashrc
```

2. 在文件末尾添加以下行：
   Add the following line at the end of the file:
```bash
# 自动显示技术运势 / Auto display tech fortune
source /Users/martin/Develop/findu/auto-fortune.sh
```

3. 保存并重新加载配置：
   Save and reload the configuration:
```bash
source ~/.bashrc
```

### 4. 验证设置 / Verify Setup

重新打开终端，你应该会看到类似这样的输出：

Reopen the terminal, you should see output similar to this:

```
🎯 正在获取今日技术运势...
🎯 Getting today's tech fortune...

==================================================
🎯 今日技术工作运势
==================================================

📊 运势得分: 82
💬 运势评价: ✨ 今日工作运势很好
...
```

## 功能说明 / Feature Description

- **每日一次**: 脚本会记住今天已经显示过运势，避免重复显示
- **自动检测**: 每天第一次打开终端时会自动显示
- **手动调用**: 任何时候都可以手动运行 `findu` 查看运势
- **多语言支持**: 支持中文和英文，可以随时切换

- **Once per day**: The script remembers that fortune has been displayed today, avoiding duplicate display
- **Auto detection**: Automatically displays when opening terminal for the first time each day
- **Manual call**: Can manually run `findu` anytime to view fortune
- **Multi-language support**: Supports Chinese and English, can switch anytime

## 语言配置 / Language Configuration

语言设置保存在 `~/.findu/config.txt` 文件中：

Language settings are saved in `~/.findu/config.txt`:

- `zh` - 中文 / Chinese
- `en` - 英文 / English

你可以手动编辑这个文件来更改语言设置。

You can manually edit this file to change the language setting.

## 卸载 / Uninstall

如果你想停止自动显示：

If you want to stop auto display:

1. 编辑你的 shell 配置文件（~/.zshrc 或 ~/.bashrc）
   Edit your shell configuration file (~/.zshrc or ~/.bashrc)
2. 删除添加的 `source` 行
   Delete the added `source` line
3. 重新加载配置
   Reload the configuration

## 故障排除 / Troubleshooting

如果遇到问题：

If you encounter problems:

1. 确保 `findu` 命令可以正常运行
   Ensure the `findu` command can run normally
2. 检查脚本路径是否正确
   Check if the script path is correct
3. 确保脚本有执行权限：`chmod +x auto-fortune.sh`
   Ensure the script has execution permission: `chmod +x auto-fortune.sh`
4. 检查语言配置文件：`cat ~/.findu/config.txt`
   Check language configuration file: `cat ~/.findu/config.txt`

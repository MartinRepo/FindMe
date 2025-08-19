# Findu Installation and Setup Guide

[🇨🇳 简体中文安装指南 / Chinese Setup Guide](SETUP_CN.md)

## Quick Start

### 1. Install Tool

The tool has been installed to your system, you can use it directly:

```bash
findu
```

### 2. Language Setup

On first use, the tool will prompt you to select a language:

```bash
🌍 Please select language:
1. 中文 (Chinese)
2. English

Enter choice: 1
```

You can also change the language setting anytime:

```bash
# Set default language
findu --set-language

# Temporarily use different language
findu --language en
```

### 3. Setup Auto Display (Optional)

If you want to automatically display today's fortune every time you open the terminal, follow these steps:

#### For zsh users (macOS default)

1. Open your zsh configuration file:
```bash
nano ~/.zshrc
```

2. Add the following line at the end of the file:
```bash
# Auto display tech fortune
source /Users/martin/Develop/findu/auto-fortune.sh
```

3. Save and reload the configuration:
```bash
source ~/.zshrc
```

#### For bash users

1. Open your bash configuration file:
```bash
nano ~/.bashrc
```

2. Add the following line at the end of the file:
```bash
# Auto display tech fortune
source /Users/martin/Develop/findu/auto-fortune.sh
```

3. Save and reload the configuration:
```bash
source ~/.bashrc
```

### 4. Verify Setup

Reopen the terminal, you should see output similar to this:

```
🎯 Getting today's tech fortune...

==================================================
🎯 Today's Tech Work Fortune
==================================================

📊 Fortune Score: 82
💬 Fortune Message: ✨ Great work fortune today
...
```

## Feature Description

- **Once per day**: The script remembers that fortune has been displayed today, avoiding duplicate display
- **Auto detection**: Automatically displays when opening terminal for the first time each day
- **Manual call**: Can manually run `findu` anytime to view fortune
- **Multi-language support**: Supports Chinese and English, can switch anytime

## Language Configuration

Language settings are saved in `~/.findu/config.txt`:

- `zh` - Chinese
- `en` - English

You can manually edit this file to change the language setting.

## Uninstall

If you want to stop auto display:

1. Edit your shell configuration file (~/.zshrc or ~/.bashrc)
2. Delete the added `source` line
3. Reload the configuration

## Troubleshooting

If you encounter problems:

1. Ensure the `findu` command can run normally
2. Check if the script path is correct
3. Ensure the script has execution permission: `chmod +x auto-fortune.sh`
4. Check language configuration file: `cat ~/.findu/config.txt`

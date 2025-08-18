# Findu 🎯

一个有趣的终端命令行工具，为技术人员提供每日工作运势预测。

A fun terminal command-line tool that provides daily work fortune predictions for tech professionals.

## 功能特点 / Features

- 🎯 **技术工作运势预测** - 专门针对程序员和技术人员的工作运势
- 📊 **运势评分系统** - 30-95分的运势评分，不同分数对应不同的工作建议
- 🎨 **彩色终端输出** - 美观的彩色界面，提升用户体验
- 📅 **基于日期的随机性** - 每天运势不同，但同一天运势固定
- 💡 **实用工作建议** - 根据运势分数给出具体的技术工作建议
- 🌍 **多语言支持** - 支持中文和英文两种语言

- 🎯 **Tech Work Fortune Prediction** - Specifically designed for programmers and tech professionals
- 📊 **Fortune Scoring System** - 30-95 point scoring system with different work advice for each score
- 🎨 **Colored Terminal Output** - Beautiful colored interface for better user experience
- 📅 **Date-based Randomness** - Different fortune each day, but same fortune for the same day
- 💡 **Practical Work Advice** - Specific technical work advice based on fortune score
- 🌍 **Multi-language Support** - Support for Chinese and English

## 运势等级说明 / Fortune Level Description

| 分数范围 | 等级 | 描述 |
|---------|------|------|
| 90-95 | 🌟 极佳 | 适合进行重要的代码重构或系统升级 |
| 80-89 | ✨ 很好 | 代码质量会很高，bug率很低 |
| 70-79 | 👍 不错 | 编程效率较高，但要注意代码审查 |
| 60-69 | 😐 一般 | 写代码时要多测试，避免留下隐患 |
| 50-59 | ⚠️ 小心 | 代码容易出错，建议多写单元测试 |
| 40-49 | 😰 不佳 | 容易遇到技术难题，建议多查阅文档 |
| 30-39 | 🚨 挑战 | 代码容易出bug，建议专注于简单任务 |

| Score Range | Level | Description |
|-------------|-------|-------------|
| 90-95 | 🌟 Excellent | Suitable for important code refactoring or system upgrades |
| 80-89 | ✨ Great | Code quality will be high, low bug rate |
| 70-79 | 👍 Good | High programming efficiency, but pay attention to code review |
| 60-69 | 😐 Average | Write code carefully, avoid leaving hidden dangers |
| 50-59 | ⚠️ Careful | Code is prone to errors, suggest writing more unit tests |
| 40-49 | 😰 Poor | Easy to encounter technical difficulties, suggest consulting documentation |
| 30-39 | 🚨 Challenging | Code is prone to bugs, suggest focusing on simple tasks |

## 安装 / Installation

### 从源码编译 / From Source

```bash
git clone <repository-url>
cd findu
cargo build --release
```

### 安装到系统 / Install to System

```bash
cargo install --path .
```

## 使用方法 / Usage

### 基本使用 / Basic Usage

```bash
# 显示今日运势 / Show today's fortune
findu

# 显示详细信息和今日运势 / Show detailed information and today's fortune
findu --verbose

# 指定语言 / Specify language
findu --language zh  # 中文 / Chinese
findu --language en  # 英文 / English

# 设置默认语言 / Set default language
findu --set-language
```

### 语言设置 / Language Setup

首次使用时，工具会提示你选择语言。你也可以随时更改语言设置：

On first use, the tool will prompt you to select a language. You can also change the language setting anytime:

```bash
# 设置语言 / Set language
findu --set-language

# 临时使用不同语言 / Temporarily use different language
findu --language en
```

### 示例输出 / Example Output

#### 中文版本 / Chinese Version
```
==================================================
🎯 今日技术工作运势
==================================================

📊 运势得分: 85
💬 运势评价: ✨ 今日工作运势很好
💡 今日建议: 代码质量会很高，bug率很低，适合处理复杂的技术问题。
🎨 幸运颜色: 蓝色
⏰ 幸运时间: 下午2-4点

📋 今日工作建议:
✅ 编程效率很高，bug率低
✅ 适合处理重要的功能开发
✅ 代码质量会很好
⚠️ 注意保持代码规范

==================================================
```

#### English Version
```
==================================================
🎯 Today's Tech Work Fortune
==================================================

📊 Fortune Score: 85
💬 Fortune Message: ✨ Great work fortune today
💡 Today's Advice: Code quality will be high, low bug rate, suitable for handling complex technical issues.
🎨 Lucky Color: Blue
⏰ Lucky Time: 2-4 PM

📋 Today's Work Advice:
✅ High programming efficiency, low bug rate
✅ Suitable for important feature development
✅ Code quality will be excellent
⚠️ Pay attention to code standards

==================================================
```

## 技术实现 / Technical Implementation

- **Rust** - 高性能系统编程语言 / High-performance systems programming language
- **clap** - 命令行参数解析 / Command line argument parsing
- **chrono** - 日期时间处理 / Date and time handling
- **rand** - 随机数生成 / Random number generation
- **colored** - 终端颜色输出 / Terminal color output
- **dirs** - 配置文件路径管理 / Configuration file path management

## 运势算法 / Fortune Algorithm

运势基于当前日期生成，确保：
- 同一天多次运行得到相同结果
- 不同日期得到不同结果
- 运势分数在合理范围内分布

Fortune is generated based on current date, ensuring:
- Same result for multiple runs on the same day
- Different results for different dates
- Fortune scores distributed within reasonable range

## 配置文件 / Configuration

语言设置保存在 `~/.findu/config.txt` 文件中：
- `zh` - 中文 / Chinese
- `en` - 英文 / English

Language settings are saved in `~/.findu/config.txt`:
- `zh` - Chinese
- `en` - English

## 许可证 / License

MIT OR Apache-2.0

## 贡献 / Contributing

欢迎提交 Issue 和 Pull Request！

Welcome to submit Issues and Pull Requests!

---

**注意**: 这是一个娱乐性质的工具，运势预测仅供娱乐参考，请理性对待工作决策。

**Note**: This is an entertainment tool, fortune predictions are for entertainment purposes only, please make work decisions rationally.
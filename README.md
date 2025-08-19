# Findu 🎯

[🇨🇳 简体中文](README_CN.md)

A fun terminal command-line tool that provides daily work fortune predictions for tech professionals.

## Features

- 🎯 **Tech Work Fortune Prediction** - Specifically designed for programmers and tech professionals
- 📊 **Fortune Scoring System** - 30-95 point scoring system with different work advice for each score
- 🎨 **Colored Terminal Output** - Beautiful colored interface for better user experience
- 📅 **Date-based Randomness** - Different fortune each day, but same fortune for the same day
- 💡 **Practical Work Advice** - Specific technical work advice based on fortune score
- 🌍 **Multi-language Support** - Support for Chinese and English

## Fortune Level Description

| Score Range | Level | Description |
|-------------|-------|-------------|
| 90-95 | 🌟 Excellent | Suitable for important code refactoring or system upgrades |
| 80-89 | ✨ Great | Code quality will be high, low bug rate |
| 70-79 | 👍 Good | High programming efficiency, but pay attention to code review |
| 60-69 | 😐 Average | Write code carefully, avoid leaving hidden dangers |
| 50-59 | ⚠️ Careful | Code is prone to errors, suggest writing more unit tests |
| 40-49 | 😰 Poor | Easy to encounter technical difficulties, suggest consulting documentation |
| 30-39 | 🚨 Challenging | Code is prone to bugs, suggest focusing on simple tasks |

## Installation

### From Source

```bash
git clone <repository-url>
cd findu
cargo build --release
```

### Install to System

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Show today's fortune
findu

# Show detailed information and today's fortune
findu --verbose

# Specify language
findu --language zh  # Chinese
findu --language en  # English

# Set default language
findu --set-language
```

### Language Setup

On first use, the tool will prompt you to select a language. You can also change the language setting anytime:

```bash
# Set language
findu --set-language

# Temporarily use different language
findu --language en
```

### Example Output

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

## Technical Implementation

- **Rust** - High-performance systems programming language
- **clap** - Command line argument parsing
- **chrono** - Date and time handling
- **rand** - Random number generation
- **colored** - Terminal color output
- **dirs** - Configuration file path management

## Fortune Algorithm

Fortune is generated based on current date, ensuring:
- Same result for multiple runs on the same day
- Different results for different dates
- Fortune scores distributed within reasonable range

## Configuration

Language settings are saved in `~/.findu/config.txt`:
- `zh` - Chinese
- `en` - English

## License

MIT OR Apache-2.0

## Contributing

Welcome to submit Issues and Pull Requests!

---

**Note**: This is an entertainment tool, fortune predictions are for entertainment purposes only, please make work decisions rationally.
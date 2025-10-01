<h1 align="center">
🎯 FindMe
</h1>

<p align="center">
  <a href="https://github.com/MartinRepo/FindMe/releases/latest">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/MartinRepo/FindMe.svg">
  </a>
  <a href="https://github.com/MartinRepo/FindMe/issues">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/MartinRepo/FindMe?style=flat-square">
  </a>
  <a href="#">
    <img alt="GitHub stars" src="https://img.shields.io/github/stars/MartinRepo/FindMe?style=flat-square">
  </a>
  <a href="https://github.com/MartinRepo/FindMe/network">
    <img alt="GitHub forks" src="https://img.shields.io/github/forks/MartinRepo/FindMe?style=flat-square">
  </a>
</p>

A fun terminal command-line tool that provides daily decompression fortune predictions for programmers and tech professionals.

## Features

- 🎯 **Programmer's Daily Decompression Oracle** - Tech dimension analysis with deterministic daily variations
- 📊 **Tech Dimensions Scoring** - Five-dimensional analysis: Focus, Creativity, Debugging Touch, Collaboration Index, Risk Tolerance
- 💡 **Personalized Tech Advice** - Tailored recommendations based on your tech dimensions and current scenario
- 🔬 **Developer Pressure Index** - Analyzes local git/test/build data for risk and patience thresholds
- 🌍 **Multi-language Support** - Support for Chinese and English
- 💾 **Local Preferences Storage** - Remembers your saved language and birthday for future sessions

## Tech Dimensions

Findme analyzes your daily tech performance across five key dimensions:

| Dimension | Description |
|-----------|-------------|
| **Focus** | Concentration and attention to detail |
| **Creativity** | Innovation and problem-solving ability |
| **Debugging** | Troubleshooting and error-fixing skills |
| **Collaboration** | Teamwork and communication effectiveness |
| **Risk Tolerance** | Willingness to try new approaches |

## Installation

### From Source

```bash
git clone <repository-url>
cd findme
cargo build --release
```

### Install to System

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Show today's tech fortune
findme

# Show detailed information and today's fortune
findme --verbose

# Personalized analysis with birthday
findme --birthday "1990-05-15"
# The birthday will be cached locally for next time

# Show developer pressure index (analyzes local git/test/build data)
findme --pressure

# Combine features
findme --pressure --birthday "1990-05-15" --language zh

# Specify language
findme --language zh  # Chinese
findme --language en  # English

# Set default language
findme --set-language
```

### Language Setup

On first use, the tool will prompt you to select a language. You can also change the language setting anytime:

```bash
# Set language
findme --set-language

# Temporarily use different language
findme --language en
```

### Example Output
```
============================================================
2025-10-01 · 🎯 Developer's Daily Decompression Oracle
Welcome martin!
============================================================

📊 Overall Score: 66

🎯 Tech Dimensions
  🎯 Focus: ████████████░░░░░░░░  63
  💡 Creativity: ████████████░░░░░░░░  63
  🐛 Debugging: ████████████░░░░░░░░  63
  🤝 Collaboration: ███████████████░░░░░  78
  ⚡ Risk Tolerance: ██████░░░░░░░░░░░░░░  34

💬 Today's Status: 😐 Average state today
💡 Tech Advice: Write code carefully, avoid leaving hidden dangers.

🎨 Recommended Color: Green
⏰ Best Time: 2-4 PM

============================================================
```

#### With Developer Pressure Index
```
============================================================
2025-10-01 · 🎯 Developer's Daily Decompression Oracle
Welcome martin!
============================================================

📊 Overall Score: 66

🎯 Tech Dimensions
  🎯 Focus: ████████████░░░░░░░░  63
  💡 Creativity: ████████████░░░░░░░░  63
  🐛 Debugging: ████████████░░░░░░░░  63
  🤝 Collaboration: ███████████████░░░░░  78
  ⚡ Risk Tolerance: ██████░░░░░░░░░░░░░░  34

💬 Today's Status: 😐 Average state today
💡 Tech Advice: Write code carefully, avoid leaving hidden dangers.

🎨 Recommended Color: Green
⏰ Best Time: 2-4 PM

============================================================


============================================================
🔬 Developer Pressure Index
============================================================
🟠 Pressure Level High Pressure - Stressed

📊 Development Metrics
  📝 Git Diff Lines 180 lines
  🧪 Test Success Rate No tests yet
  ⚡ Build Time 0s

💭 Today's Advice
  ⚠️ High Risk Threshold - Prioritize fixing issues
  💡 Suggest fixing test failures and build issues first
============================================================
```

## Configuration

Preferences are saved in `~/.findme/config.txt` as simple key-value pairs:

```
language=en
birthday=1990-05-15
```

- `language` is stored when you run `findme --set-language`
- `birthday` is automatically saved the first time you pass `--birthday`

To customize the location (for example in scripts or automated tests), set the
`FINDME_CONFIG_DIR` environment variable to the directory where the config file
should live.

## License

[Apache-2.0](https://github.com/MartinRepo/FindMe/blob/main/LICENSE-APACHE)

## Contributing

Welcome to submit Issues and Pull Requests!

---

**Note**: This is an entertainment tool, fortune predictions are for entertainment purposes only, please make work decisions rationally.
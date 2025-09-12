use crate::utils::Language;
use std::collections::HashMap;

pub struct I18n {
    translations: HashMap<String, HashMap<Language, String>>,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = I18n {
            translations: HashMap::new(),
        };
        i18n.load_translations();
        i18n
    }

    pub fn t(&self, key: &str, lang: Language) -> String {
        self.translations
            .get(key)
            .and_then(|lang_map| lang_map.get(&lang))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    fn load_translations(&mut self) {
        self.add_translation("app.title", Language::Chinese, "🎯 程序员今日解压占卜");
        self.add_translation(
            "app.title",
            Language::English,
            "🎯 Developer's Daily Decompression Oracle",
        );

        // Scenario indicators
        self.add_translation(
            "scenario.workday",
            Language::Chinese,
            "工作日模式 - 执行/交付",
        );
        self.add_translation(
            "scenario.workday",
            Language::English,
            "Workday Mode - Execute/Deliver",
        );

        self.add_translation(
            "scenario.weekend",
            Language::Chinese,
            "周末模式 - 学习/探索",
        );
        self.add_translation(
            "scenario.weekend",
            Language::English,
            "Weekend Mode - Learn/Explore",
        );

        // 开发者体感气压
        self.add_translation("pressure.title", Language::Chinese, "🔬 开发者体感气压");
        self.add_translation(
            "pressure.title",
            Language::English,
            "🔬 Developer Pressure Index",
        );

        self.add_translation("pressure.level_label", Language::Chinese, "气压等级");
        self.add_translation("pressure.level_label", Language::English, "Pressure Level");

        self.add_translation("pressure.level.low", Language::Chinese, "低气压 - 轻松状态");
        self.add_translation(
            "pressure.level.low",
            Language::English,
            "Low Pressure - Relaxed",
        );

        self.add_translation(
            "pressure.level.medium",
            Language::Chinese,
            "中气压 - 正常状态",
        );
        self.add_translation(
            "pressure.level.medium",
            Language::English,
            "Medium Pressure - Normal",
        );

        self.add_translation(
            "pressure.level.high",
            Language::Chinese,
            "高气压 - 紧张状态",
        );
        self.add_translation(
            "pressure.level.high",
            Language::English,
            "High Pressure - Stressed",
        );

        self.add_translation(
            "pressure.level.critical",
            Language::Chinese,
            "临界气压 - 高压状态",
        );
        self.add_translation(
            "pressure.level.critical",
            Language::English,
            "Critical Pressure - Overwhelmed",
        );

        self.add_translation("pressure.metrics_label", Language::Chinese, "📊 开发指标");
        self.add_translation(
            "pressure.metrics_label",
            Language::English,
            "📊 Development Metrics",
        );

        self.add_translation("pressure.git_diff_label", Language::Chinese, "Git变更行数");
        self.add_translation(
            "pressure.git_diff_label",
            Language::English,
            "Git Diff Lines",
        );

        self.add_translation("pressure.lines", Language::Chinese, "行");
        self.add_translation("pressure.lines", Language::English, "lines");

        self.add_translation(
            "pressure.test_success_label",
            Language::Chinese,
            "测试成功率",
        );
        self.add_translation(
            "pressure.test_success_label",
            Language::English,
            "Test Success Rate",
        );

        self.add_translation("pressure.no_tests", Language::Chinese, "还没测试过");
        self.add_translation("pressure.no_tests", Language::English, "No tests yet");

        self.add_translation("pressure.build_time_label", Language::Chinese, "构建耗时");
        self.add_translation("pressure.build_time_label", Language::English, "Build Time");

        self.add_translation("pressure.advice_label", Language::Chinese, "💭 今日建议");
        self.add_translation(
            "pressure.advice_label",
            Language::English,
            "💭 Today's Advice",
        );

        self.add_translation(
            "pressure.advice.low.risk",
            Language::Chinese,
            "低风险阈值 - 可以尝试新功能",
        );
        self.add_translation(
            "pressure.advice.low.risk",
            Language::English,
            "Low Risk Threshold - Can try new features",
        );

        self.add_translation(
            "pressure.advice.low.patience",
            Language::Chinese,
            "保持当前节奏，适合探索性开发",
        );
        self.add_translation(
            "pressure.advice.low.patience",
            Language::English,
            "Maintain current pace, suitable for exploratory development",
        );

        self.add_translation(
            "pressure.advice.medium.risk",
            Language::Chinese,
            "中等风险阈值 - 谨慎推进",
        );
        self.add_translation(
            "pressure.advice.medium.risk",
            Language::English,
            "Medium Risk Threshold - Proceed with caution",
        );

        self.add_translation(
            "pressure.advice.medium.patience",
            Language::Chinese,
            "建议先完善现有功能，再考虑新特性",
        );
        self.add_translation(
            "pressure.advice.medium.patience",
            Language::English,
            "Suggest improving existing features first, then consider new ones",
        );

        self.add_translation(
            "pressure.advice.high.risk",
            Language::Chinese,
            "高风险阈值 - 优先修复问题",
        );
        self.add_translation(
            "pressure.advice.high.risk",
            Language::English,
            "High Risk Threshold - Prioritize fixing issues",
        );

        self.add_translation(
            "pressure.advice.high.patience",
            Language::Chinese,
            "建议先解决测试失败和构建问题",
        );
        self.add_translation(
            "pressure.advice.high.patience",
            Language::English,
            "Suggest fixing test failures and build issues first",
        );

        self.add_translation(
            "pressure.advice.critical.risk",
            Language::Chinese,
            "临界风险阈值 - 紧急修复模式",
        );
        self.add_translation(
            "pressure.advice.critical.risk",
            Language::English,
            "Critical Risk Threshold - Emergency fix mode",
        );

        self.add_translation(
            "pressure.advice.critical.patience",
            Language::Chinese,
            "建议暂停新功能开发，专注解决核心问题",
        );
        self.add_translation(
            "pressure.advice.critical.patience",
            Language::English,
            "Suggest pausing new feature development, focus on core issues",
        );

        self.add_translation(
            "fortune.overall_score_label",
            Language::Chinese,
            "📊 综合评分",
        );
        self.add_translation(
            "fortune.overall_score_label",
            Language::English,
            "📊 Overall Score",
        );

        self.add_translation("fortune.dimensions_label", Language::Chinese, "🎯 技术五维");
        self.add_translation(
            "fortune.dimensions_label",
            Language::English,
            "🎯 Tech Dimensions",
        );

        self.add_translation("fortune.focus_label", Language::Chinese, "🎯 专注力");
        self.add_translation("fortune.focus_label", Language::English, "🎯 Focus");

        self.add_translation("fortune.creativity_label", Language::Chinese, "💡 创造力");
        self.add_translation(
            "fortune.creativity_label",
            Language::English,
            "💡 Creativity",
        );

        self.add_translation("fortune.debugging_label", Language::Chinese, "🐛 调试手感");
        self.add_translation("fortune.debugging_label", Language::English, "🐛 Debugging");

        self.add_translation(
            "fortune.collaboration_label",
            Language::Chinese,
            "🤝 协作指数",
        );
        self.add_translation(
            "fortune.collaboration_label",
            Language::English,
            "🤝 Collaboration",
        );

        self.add_translation("fortune.risk_label", Language::Chinese, "⚡ 风险偏好");
        self.add_translation("fortune.risk_label", Language::English, "⚡ Risk Tolerance");

        self.add_translation("fortune.message_label", Language::Chinese, "💬 今日状态");
        self.add_translation(
            "fortune.message_label",
            Language::English,
            "💬 Today's Status",
        );

        self.add_translation("fortune.advice_label", Language::Chinese, "💡 技术建议");
        self.add_translation("fortune.advice_label", Language::English, "💡 Tech Advice");

        self.add_translation(
            "fortune.lucky_color_label",
            Language::Chinese,
            "🎨 推荐颜色",
        );
        self.add_translation(
            "fortune.lucky_color_label",
            Language::English,
            "🎨 Recommended Color",
        );

        self.add_translation("fortune.lucky_time_label", Language::Chinese, "⏰ 最佳时间");
        self.add_translation(
            "fortune.lucky_time_label",
            Language::English,
            "⏰ Best Time",
        );

        self.add_translation(
            "fortune.detailed_advice_title",
            Language::Chinese,
            "📋 今日工作建议",
        );
        self.add_translation(
            "fortune.detailed_advice_title",
            Language::English,
            "📋 Today's Work Advice",
        );

        self.add_translation(
            "language.choose",
            Language::Chinese,
            "🌍 请选择语言 / Please select language:",
        );
        self.add_translation(
            "language.choose",
            Language::English,
            "🌍 Please select language:",
        );

        self.add_translation("language.chinese", Language::Chinese, "1. 中文 (Chinese)");
        self.add_translation("language.chinese", Language::English, "1. Chinese");

        self.add_translation("language.english", Language::Chinese, "2. English");
        self.add_translation("language.english", Language::English, "2. English");

        self.add_translation(
            "language.enter_choice",
            Language::Chinese,
            "请输入选择 (Enter choice): ",
        );
        self.add_translation("language.enter_choice", Language::English, "Enter choice: ");

        self.add_translation(
            "language.invalid_choice",
            Language::Chinese,
            "❌ 无效选择，请输入 1 或 2 / Invalid choice, please enter 1 or 2",
        );
        self.add_translation(
            "language.invalid_choice",
            Language::English,
            "❌ Invalid choice, please enter 1 or 2",
        );

        self.add_translation(
            "language.set_chinese",
            Language::Chinese,
            "✅ 语言已设置为中文",
        );
        self.add_translation(
            "language.set_chinese",
            Language::English,
            "✅ Language set to Chinese",
        );

        self.add_translation(
            "language.set_english",
            Language::Chinese,
            "✅ Language set to English",
        );
        self.add_translation(
            "language.set_english",
            Language::English,
            "✅ Language set to English",
        );

        self.add_translation(
            "language.first_time",
            Language::Chinese,
            "🌍 首次使用，请选择语言 / First time use, please select language:",
        );
        self.add_translation(
            "language.first_time",
            Language::English,
            "🌍 First time use, please select language:",
        );

        self.add_translation(
            "language.cannot_save",
            Language::Chinese,
            "⚠️ 无法保存语言设置 / Cannot save language setting: {}",
        );
        self.add_translation(
            "language.cannot_save",
            Language::English,
            "⚠️ Cannot save language setting: {}",
        );

        self.add_translation(
            "language.invalid_option",
            Language::Chinese,
            "❌ 无效的语言选项 / Invalid language option: {}",
        );
        self.add_translation(
            "language.invalid_option",
            Language::English,
            "❌ Invalid language option: {}",
        );

        self.add_translation(
            "language.use_zh_en",
            Language::Chinese,
            "请使用: zh/en / Please use: zh/en",
        );
        self.add_translation("language.use_zh_en", Language::English, "Please use: zh/en");

        // Fortune messages - Excellent (90-100)
        self.add_translation("fortune.excellent", Language::Chinese, "🌟 今日状态爆表！");
        self.add_translation(
            "fortune.excellent",
            Language::English,
            "🌟 Today's state is off the charts!",
        );

        self.add_translation(
            "fortune.excellent.focus",
            Language::Chinese,
            "🎯 专注力拉满，代码如丝般顺滑",
        );
        self.add_translation(
            "fortune.excellent.focus",
            Language::English,
            "🎯 Focus maxed out, code flows like silk",
        );

        self.add_translation(
            "fortune.excellent.creativity",
            Language::Chinese,
            "💡 创造力爆棚，灵感如泉涌",
        );
        self.add_translation(
            "fortune.excellent.creativity",
            Language::English,
            "💡 Creativity overflowing, ideas gushing",
        );

        self.add_translation(
            "fortune.excellent.debugging",
            Language::Chinese,
            "🐛 调试手感绝佳，bug无处遁形",
        );
        self.add_translation(
            "fortune.excellent.debugging",
            Language::English,
            "🐛 Perfect debugging touch, bugs have nowhere to hide",
        );

        self.add_translation(
            "fortune.excellent.collaboration",
            Language::Chinese,
            "🤝 协作指数爆表，团队效率翻倍",
        );
        self.add_translation(
            "fortune.excellent.collaboration",
            Language::English,
            "🤝 Collaboration off the charts, team efficiency doubled",
        );

        self.add_translation(
            "fortune.excellent.risk",
            Language::Chinese,
            "⚡ 风险偏好拉满，敢于尝试新技术",
        );
        self.add_translation(
            "fortune.excellent.risk",
            Language::English,
            "⚡ Risk tolerance maxed, ready to try new tech",
        );

        // Fortune messages - Great (80-89)
        self.add_translation("fortune.great", Language::Chinese, "✨ 今日状态很棒");
        self.add_translation("fortune.great", Language::English, "✨ Great state today");

        self.add_translation(
            "fortune.great.focus",
            Language::Chinese,
            "🎯 专注力在线，代码质量有保障",
        );
        self.add_translation(
            "fortune.great.focus",
            Language::English,
            "🎯 Focus is on point, code quality guaranteed",
        );

        self.add_translation(
            "fortune.great.creativity",
            Language::Chinese,
            "💡 创造力在线，创新想法不断",
        );
        self.add_translation(
            "fortune.great.creativity",
            Language::English,
            "💡 Creativity is flowing, innovative ideas keep coming",
        );

        self.add_translation(
            "fortune.great.debugging",
            Language::Chinese,
            "🐛 调试手感不错，问题解决效率高",
        );
        self.add_translation(
            "fortune.great.debugging",
            Language::English,
            "🐛 Good debugging touch, high problem-solving efficiency",
        );

        self.add_translation(
            "fortune.great.collaboration",
            Language::Chinese,
            "🤝 协作指数不错，团队配合默契",
        );
        self.add_translation(
            "fortune.great.collaboration",
            Language::English,
            "🤝 Good collaboration, team synergy is strong",
        );

        self.add_translation(
            "fortune.great.risk",
            Language::Chinese,
            "⚡ 风险偏好适中，敢于适度冒险",
        );
        self.add_translation(
            "fortune.great.risk",
            Language::English,
            "⚡ Moderate risk tolerance, ready for calculated risks",
        );

        // Other fortune levels
        self.add_translation("fortune.good", Language::Chinese, "👍 今日状态不错");
        self.add_translation("fortune.good", Language::English, "👍 Good state today");

        self.add_translation("fortune.average", Language::Chinese, "😐 今日状态一般");
        self.add_translation(
            "fortune.average",
            Language::English,
            "😐 Average state today",
        );

        self.add_translation("fortune.careful", Language::Chinese, "⚠️ 今日需要小心");
        self.add_translation(
            "fortune.careful",
            Language::English,
            "⚠️ Need to be careful today",
        );

        self.add_translation("fortune.poor", Language::Chinese, "😰 今日状态不佳");
        self.add_translation("fortune.poor", Language::English, "😰 Poor state today");

        self.add_translation("fortune.challenging", Language::Chinese, "🚨 今日挑战很大");
        self.add_translation(
            "fortune.challenging",
            Language::English,
            "🚨 Major challenges today",
        );

        self.add_translation("fortune.unknown", Language::Chinese, "🤔 今日状态未知");
        self.add_translation(
            "fortune.unknown",
            Language::English,
            "🤔 Unknown state today",
        );

        // Fortune advice - Excellent (90-100)
        self.add_translation(
            "advice.excellent",
            Language::Chinese,
            "适合进行重要的代码重构或系统升级，一切都会很顺利。",
        );
        self.add_translation("advice.excellent", Language::English, "Perfect for important code refactoring or system upgrades, everything will go smoothly.");

        self.add_translation(
            "advice.excellent.focus",
            Language::Chinese,
            "专注力爆表，适合处理复杂的算法和架构设计，代码质量会很高。",
        );
        self.add_translation("advice.excellent.focus", Language::English, "Focus is maxed out, perfect for complex algorithms and architecture design, code quality will be excellent.");

        self.add_translation(
            "advice.excellent.creativity",
            Language::Chinese,
            "创造力爆棚，适合尝试新的技术栈、设计模式和创新解决方案。",
        );
        self.add_translation("advice.excellent.creativity", Language::English, "Creativity is overflowing, perfect for trying new tech stacks, design patterns, and innovative solutions.");

        self.add_translation(
            "advice.excellent.debugging",
            Language::Chinese,
            "调试手感绝佳，适合处理复杂的bug和性能优化，问题解决效率极高。",
        );
        self.add_translation("advice.excellent.debugging", Language::English, "Perfect debugging touch, ideal for complex bugs and performance optimization, problem-solving efficiency is extremely high.");

        self.add_translation(
            "advice.excellent.collaboration",
            Language::Chinese,
            "协作指数爆表，适合进行代码审查、技术分享和团队协作项目。",
        );
        self.add_translation("advice.excellent.collaboration", Language::English, "Collaboration is off the charts, perfect for code reviews, tech sharing, and team collaboration projects.");

        self.add_translation(
            "advice.excellent.risk",
            Language::Chinese,
            "风险偏好拉满，适合尝试新技术、重构遗留代码和探索前沿技术。",
        );
        self.add_translation("advice.excellent.risk", Language::English, "Risk tolerance is maxed, perfect for trying new technologies, refactoring legacy code, and exploring cutting-edge tech.");

        // Fortune advice - Great (80-89)
        self.add_translation(
            "advice.great",
            Language::Chinese,
            "代码质量会很高，bug率很低，适合处理复杂的技术问题。",
        );
        self.add_translation("advice.great", Language::English, "Code quality will be high, low bug rate, suitable for handling complex technical issues.");

        self.add_translation(
            "advice.great.focus",
            Language::Chinese,
            "专注力在线，适合处理需要深度思考的编程任务，代码质量有保障。",
        );
        self.add_translation("advice.great.focus", Language::English, "Focus is on point, suitable for programming tasks requiring deep thinking, code quality is guaranteed.");

        self.add_translation(
            "advice.great.creativity",
            Language::Chinese,
            "创造力在线，适合优化现有代码、设计新功能和探索创新方案。",
        );
        self.add_translation("advice.great.creativity", Language::English, "Creativity is flowing, suitable for optimizing existing code, designing new features, and exploring innovative solutions.");

        self.add_translation(
            "advice.great.debugging",
            Language::Chinese,
            "调试手感不错，适合处理中等复杂度的bug和代码优化任务。",
        );
        self.add_translation("advice.great.debugging", Language::English, "Good debugging touch, suitable for medium-complexity bugs and code optimization tasks.");

        self.add_translation(
            "advice.great.collaboration",
            Language::Chinese,
            "协作指数不错，适合参与团队项目、技术讨论和知识分享。",
        );
        self.add_translation("advice.great.collaboration", Language::English, "Good collaboration, suitable for team projects, technical discussions, and knowledge sharing.");

        self.add_translation(
            "advice.great.risk",
            Language::Chinese,
            "风险偏好适中，适合尝试新工具、优化现有流程和适度创新。",
        );
        self.add_translation("advice.great.risk", Language::English, "Moderate risk tolerance, suitable for trying new tools, optimizing existing processes, and moderate innovation.");

        // Other advice levels
        self.add_translation(
            "advice.good",
            Language::Chinese,
            "编程效率较高，但要注意代码审查，避免小错误。",
        );
        self.add_translation(
            "advice.good",
            Language::English,
            "High programming efficiency, but pay attention to code review, avoid small errors.",
        );

        self.add_translation(
            "advice.average",
            Language::Chinese,
            "写代码时要多测试，避免留下隐患。",
        );
        self.add_translation(
            "advice.average",
            Language::English,
            "Write code carefully, avoid leaving hidden dangers.",
        );

        self.add_translation(
            "advice.careful",
            Language::Chinese,
            "代码容易出错，建议多写单元测试，仔细检查逻辑。",
        );
        self.add_translation(
            "advice.careful",
            Language::English,
            "Code is prone to errors, suggest writing more unit tests, check logic carefully.",
        );

        self.add_translation(
            "advice.poor",
            Language::Chinese,
            "容易遇到技术难题，建议多查阅文档，不要急于求成。",
        );
        self.add_translation("advice.poor", Language::English, "Easy to encounter technical difficulties, suggest consulting documentation, don't rush.");

        self.add_translation(
            "advice.challenging",
            Language::Chinese,
            "代码容易出bug，建议专注于简单任务，避免复杂操作。",
        );
        self.add_translation(
            "advice.challenging",
            Language::English,
            "Code is prone to bugs, suggest focusing on simple tasks, avoid complex operations.",
        );

        self.add_translation(
            "advice.unknown",
            Language::Chinese,
            "保持平常心，专注工作即可。",
        );
        self.add_translation(
            "advice.unknown",
            Language::English,
            "Keep calm and focus on work.",
        );

        // CLI help text
        self.add_translation(
            "cli.about",
            Language::Chinese,
            "显示今日技术工作运势 / Show today's tech work fortune",
        );
        self.add_translation(
            "cli.about",
            Language::English,
            "Show today's tech work fortune",
        );

        self.add_translation(
            "cli.verbose_help",
            Language::Chinese,
            "显示详细帮助信息 / Show verbose help information",
        );
        self.add_translation(
            "cli.verbose_help",
            Language::English,
            "Show verbose help information",
        );

        self.add_translation(
            "cli.date_help",
            Language::Chinese,
            "指定日期 (格式: YYYY-MM-DD) / Specify date (format: YYYY-MM-DD)",
        );
        self.add_translation(
            "cli.date_help",
            Language::English,
            "Specify date (format: YYYY-MM-DD)",
        );

        self.add_translation(
            "cli.language_help",
            Language::Chinese,
            "语言选择 / Language selection (zh/en)",
        );
        self.add_translation(
            "cli.language_help",
            Language::English,
            "Language selection (zh/en)",
        );

        self.add_translation(
            "cli.set_language_help",
            Language::Chinese,
            "设置语言 / Set language",
        );
        self.add_translation("cli.set_language_help", Language::English, "Set language");

        // Comments
        self.add_translation(
            "comment.handle_language_setting",
            Language::Chinese,
            "处理语言设置",
        );
        self.add_translation(
            "comment.handle_language_setting",
            Language::English,
            "Handle language setting",
        );

        self.add_translation(
            "comment.determine_language",
            Language::Chinese,
            "确定使用的语言",
        );
        self.add_translation(
            "comment.determine_language",
            Language::English,
            "Determine language to use",
        );

        self.add_translation(
            "comment.generate_daily_fortune",
            Language::Chinese,
            "生成今日运势",
        );
        self.add_translation(
            "comment.generate_daily_fortune",
            Language::English,
            "Generate daily fortune",
        );

        self.add_translation("comment.display_fortune", Language::Chinese, "显示运势");
        self.add_translation(
            "comment.display_fortune",
            Language::English,
            "Display fortune",
        );

        // Lucky colors
        self.add_translation("color.blue", Language::Chinese, "蓝色");
        self.add_translation("color.blue", Language::English, "Blue");

        self.add_translation("color.green", Language::Chinese, "绿色");
        self.add_translation("color.green", Language::English, "Green");

        self.add_translation("color.purple", Language::Chinese, "紫色");
        self.add_translation("color.purple", Language::English, "Purple");

        self.add_translation("color.orange", Language::Chinese, "橙色");
        self.add_translation("color.orange", Language::English, "Orange");

        self.add_translation("color.red", Language::Chinese, "红色");
        self.add_translation("color.red", Language::English, "Red");

        self.add_translation("color.yellow", Language::Chinese, "黄色");
        self.add_translation("color.yellow", Language::English, "Yellow");

        self.add_translation("color.cyan", Language::Chinese, "青色");
        self.add_translation("color.cyan", Language::English, "Cyan");

        self.add_translation("color.pink", Language::Chinese, "粉色");
        self.add_translation("color.pink", Language::English, "Pink");

        // Lucky times
        self.add_translation("time.morning_9_11", Language::Chinese, "上午9-11点");
        self.add_translation("time.morning_9_11", Language::English, "9-11 AM");

        self.add_translation("time.afternoon_2_4", Language::Chinese, "下午2-4点");
        self.add_translation("time.afternoon_2_4", Language::English, "2-4 PM");

        self.add_translation("time.evening_7_9", Language::Chinese, "晚上7-9点");
        self.add_translation("time.evening_7_9", Language::English, "7-9 PM");

        self.add_translation("time.night_1_3", Language::Chinese, "凌晨1-3点");
        self.add_translation("time.night_1_3", Language::English, "1-3 AM");

        self.add_translation("time.noon_12_1", Language::Chinese, "中午12-1点");
        self.add_translation("time.noon_12_1", Language::English, "12-1 PM");

        self.add_translation("time.afternoon_5_6", Language::Chinese, "下午5-6点");
        self.add_translation("time.afternoon_5_6", Language::English, "5-6 PM");

        self.add_translation("time.evening_10_11", Language::Chinese, "晚上10-11点");
        self.add_translation("time.evening_10_11", Language::English, "10-11 PM");

        // Language parsing
        self.add_translation("lang.chinese", Language::Chinese, "中文");
        self.add_translation("lang.chinese", Language::English, "Chinese");

        self.add_translation("lang.english", Language::Chinese, "英文");
        self.add_translation("lang.english", Language::English, "English");

        // Detailed advice messages
        self.add_translation(
            "advice.excellent.refactor",
            Language::Chinese,
            "适合进行代码重构和系统优化",
        );
        self.add_translation(
            "advice.excellent.refactor",
            Language::English,
            "Suitable for code refactoring and system optimization",
        );

        self.add_translation(
            "advice.excellent.framework",
            Language::Chinese,
            "可以尝试新的技术栈或框架",
        );
        self.add_translation(
            "advice.excellent.framework",
            Language::English,
            "Can try new tech stacks or frameworks",
        );

        self.add_translation(
            "advice.excellent.review",
            Language::Chinese,
            "代码审查会非常顺利",
        );
        self.add_translation(
            "advice.excellent.review",
            Language::English,
            "Code review will be very smooth",
        );

        self.add_translation(
            "advice.excellent.problem",
            Language::Chinese,
            "适合解决复杂的技术难题",
        );
        self.add_translation(
            "advice.excellent.problem",
            Language::English,
            "Suitable for solving complex technical problems",
        );

        self.add_translation(
            "advice.great.efficiency",
            Language::Chinese,
            "编程效率很高，bug率低",
        );
        self.add_translation(
            "advice.great.efficiency",
            Language::English,
            "High programming efficiency, low bug rate",
        );

        self.add_translation(
            "advice.great.development",
            Language::Chinese,
            "适合处理重要的功能开发",
        );
        self.add_translation(
            "advice.great.development",
            Language::English,
            "Suitable for important feature development",
        );

        self.add_translation("advice.great.quality", Language::Chinese, "代码质量会很好");
        self.add_translation(
            "advice.great.quality",
            Language::English,
            "Code quality will be excellent",
        );

        self.add_translation(
            "advice.great.standards",
            Language::Chinese,
            "注意保持代码规范",
        );
        self.add_translation(
            "advice.great.standards",
            Language::English,
            "Pay attention to code standards",
        );

        self.add_translation(
            "advice.good.condition",
            Language::Chinese,
            "整体工作状态不错",
        );
        self.add_translation(
            "advice.good.condition",
            Language::English,
            "Overall work condition is good",
        );

        self.add_translation("advice.good.check", Language::Chinese, "写代码时要仔细检查");
        self.add_translation(
            "advice.good.check",
            Language::English,
            "Check code carefully when writing",
        );

        self.add_translation(
            "advice.good.documentation",
            Language::Chinese,
            "建议多写注释和文档",
        );
        self.add_translation(
            "advice.good.documentation",
            Language::English,
            "Suggest writing more comments and documentation",
        );

        self.add_translation(
            "advice.good.optimization",
            Language::Chinese,
            "避免过度优化",
        );
        self.add_translation(
            "advice.good.optimization",
            Language::English,
            "Avoid over-optimization",
        );

        self.add_translation(
            "advice.average.careful",
            Language::Chinese,
            "编程时要多加小心",
        );
        self.add_translation(
            "advice.average.careful",
            Language::English,
            "Be extra careful when programming",
        );

        self.add_translation(
            "advice.average.tests",
            Language::Chinese,
            "建议多写单元测试",
        );
        self.add_translation(
            "advice.average.tests",
            Language::English,
            "Suggest writing more unit tests",
        );

        self.add_translation(
            "advice.average.logic",
            Language::Chinese,
            "避免复杂的逻辑判断",
        );
        self.add_translation(
            "advice.average.logic",
            Language::English,
            "Avoid complex logic judgments",
        );

        self.add_translation("advice.average.review", Language::Chinese, "代码审查要仔细");
        self.add_translation(
            "advice.average.review",
            Language::English,
            "Code review should be thorough",
        );

        self.add_translation(
            "advice.careful.mistakes",
            Language::Chinese,
            "今日容易出错，需要格外小心",
        );
        self.add_translation(
            "advice.careful.mistakes",
            Language::English,
            "Easy to make mistakes today, need extra care",
        );

        self.add_translation(
            "advice.careful.simple",
            Language::Chinese,
            "建议专注于简单任务",
        );
        self.add_translation(
            "advice.careful.simple",
            Language::English,
            "Suggest focusing on simple tasks",
        );

        self.add_translation(
            "advice.careful.documentation",
            Language::Chinese,
            "多查阅文档和参考资料",
        );
        self.add_translation(
            "advice.careful.documentation",
            Language::English,
            "Consult documentation and references more",
        );

        self.add_translation("advice.careful.rush", Language::Chinese, "避免急于求成");
        self.add_translation(
            "advice.careful.rush",
            Language::English,
            "Avoid rushing things",
        );

        self.add_translation(
            "advice.poor.challenges",
            Language::Chinese,
            "技术挑战很大，建议谨慎行事",
        );
        self.add_translation(
            "advice.poor.challenges",
            Language::English,
            "Major technical challenges, suggest proceeding with caution",
        );

        self.add_translation(
            "advice.poor.complex",
            Language::Chinese,
            "避免处理复杂的技术问题",
        );
        self.add_translation(
            "advice.poor.complex",
            Language::English,
            "Avoid handling complex technical issues",
        );

        self.add_translation("advice.poor.help", Language::Chinese, "多向同事请教");
        self.add_translation(
            "advice.poor.help",
            Language::English,
            "Ask colleagues for help more",
        );

        self.add_translation(
            "advice.poor.patience",
            Language::Chinese,
            "保持耐心，不要急躁",
        );
        self.add_translation(
            "advice.poor.patience",
            Language::English,
            "Keep patience, don't be impatient",
        );

        self.add_translation(
            "advice.terrible.fortune",
            Language::Chinese,
            "今日技术运势很差，建议",
        );
        self.add_translation(
            "advice.terrible.fortune",
            Language::English,
            "Poor technical fortune today, suggest",
        );

        self.add_translation(
            "advice.terrible.learning",
            Language::Chinese,
            "专注于学习和文档阅读",
        );
        self.add_translation(
            "advice.terrible.learning",
            Language::English,
            "Focus on learning and documentation reading",
        );

        self.add_translation(
            "advice.terrible.avoid",
            Language::Chinese,
            "避免重要的代码修改",
        );
        self.add_translation(
            "advice.terrible.avoid",
            Language::English,
            "Avoid important code modifications",
        );

        self.add_translation(
            "advice.terrible.attitude",
            Language::Chinese,
            "保持积极心态，明天会更好",
        );
        self.add_translation(
            "advice.terrible.attitude",
            Language::English,
            "Keep positive attitude, tomorrow will be better",
        );
    }

    fn add_translation(&mut self, key: &str, lang: Language, value: &str) {
        self.translations
            .entry(key.to_string())
            .or_default()
            .insert(lang, value.to_string());
    }
}

pub fn i18n(key: &str, lang: Language) -> String {
    static I18N: std::sync::OnceLock<I18n> = std::sync::OnceLock::new();
    I18N.get_or_init(I18n::new).t(key, lang)
}

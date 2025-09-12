use std::collections::HashMap;
use crate::utils::Language;

pub struct I18n {
    translations: HashMap<String, HashMap<Language, String>>,
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
        self.add_translation("app.title", Language::Chinese, "🎯 今日技术工作运势");
        self.add_translation("app.title", Language::English, "🎯 Today's Tech Work Fortune");
        
        self.add_translation("app.birth_title", Language::Chinese, "🌟 出生日期运势分析");
        self.add_translation("app.birth_title", Language::English, "🌟 Birth Date Fortune Analysis");
        
        self.add_translation("fortune.score_label", Language::Chinese, "📊 运势得分");
        self.add_translation("fortune.score_label", Language::English, "📊 Fortune Score");
        
        self.add_translation("fortune.message_label", Language::Chinese, "💬 运势评价");
        self.add_translation("fortune.message_label", Language::English, "💬 Fortune Message");
        
        self.add_translation("fortune.advice_label", Language::Chinese, "💡 今日建议");
        self.add_translation("fortune.advice_label", Language::English, "💡 Today's Advice");
        
        self.add_translation("fortune.lucky_color_label", Language::Chinese, "🎨 幸运颜色");
        self.add_translation("fortune.lucky_color_label", Language::English, "🎨 Lucky Color");
        
        self.add_translation("fortune.lucky_time_label", Language::Chinese, "⏰ 幸运时间");
        self.add_translation("fortune.lucky_time_label", Language::English, "⏰ Lucky Time");
        
        self.add_translation("fortune.detailed_advice_title", Language::Chinese, "📋 今日工作建议");
        self.add_translation("fortune.detailed_advice_title", Language::English, "📋 Today's Work Advice");
        
        self.add_translation("birth.birth_date_label", Language::Chinese, "📅 出生日期");
        self.add_translation("birth.birth_date_label", Language::English, "📅 Birth Date");
        
        self.add_translation("birth.life_number_label", Language::Chinese, "🔢 生命数字");
        self.add_translation("birth.life_number_label", Language::English, "🔢 Life Number");
        
        self.add_translation("birth.zodiac_sign_label", Language::Chinese, "♈ 星座");
        self.add_translation("birth.zodiac_sign_label", Language::English, "♈ Zodiac Sign");
        
        self.add_translation("birth.personality_title", Language::Chinese, "👤 性格特征");
        self.add_translation("birth.personality_title", Language::English, "👤 Personality Traits");
        
        self.add_translation("birth.career_title", Language::Chinese, "💼 职业建议");
        self.add_translation("birth.career_title", Language::English, "💼 Career Advice");
        
        self.add_translation("birth.lucky_numbers_title", Language::Chinese, "🍀 幸运数字");
        self.add_translation("birth.lucky_numbers_title", Language::English, "🍀 Lucky Numbers");
        
        self.add_translation("birth.compatibility_title", Language::Chinese, "💕 星座配对");
        self.add_translation("birth.compatibility_title", Language::English, "💕 Zodiac Compatibility");
        
        self.add_translation("language.choose", Language::Chinese, "🌍 请选择语言 / Please select language:");
        self.add_translation("language.choose", Language::English, "🌍 Please select language:");
        
        self.add_translation("language.chinese", Language::Chinese, "1. 中文 (Chinese)");
        self.add_translation("language.chinese", Language::English, "1. Chinese");
        
        self.add_translation("language.english", Language::Chinese, "2. English");
        self.add_translation("language.english", Language::English, "2. English");
        
        self.add_translation("language.enter_choice", Language::Chinese, "请输入选择 (Enter choice): ");
        self.add_translation("language.enter_choice", Language::English, "Enter choice: ");
        
        self.add_translation("language.invalid_choice", Language::Chinese, "❌ 无效选择，请输入 1 或 2 / Invalid choice, please enter 1 or 2");
        self.add_translation("language.invalid_choice", Language::English, "❌ Invalid choice, please enter 1 or 2");
        
        self.add_translation("birth.enter_date", Language::Chinese, "请输入您的出生日期 (格式: YYYY-MM-DD): ");
        self.add_translation("birth.enter_date", Language::English, "Please enter your birth date (format: YYYY-MM-DD): ");
        
        self.add_translation("birth.invalid_date", Language::Chinese, "❌ 请输入有效的出生日期");
        self.add_translation("birth.invalid_date", Language::English, "❌ Please enter a valid birth date");
        
        self.add_translation("birth.date_format_error", Language::Chinese, "❌ 日期格式错误: {}。请使用 YYYY-MM-DD 格式");
        self.add_translation("birth.date_format_error", Language::English, "❌ Invalid date format: {}. Please use YYYY-MM-DD format");
        
        self.add_translation("birth.date_format_error_cli", Language::Chinese, "❌ 出生日期格式错误: {}。请使用 YYYY-MM-DD 格式");
        self.add_translation("birth.date_format_error_cli", Language::English, "❌ Invalid birth date format: {}. Please use YYYY-MM-DD format");
        
        self.add_translation("birth.processing_error", Language::Chinese, "❌ 处理出生日期运势时出错: {}");
        self.add_translation("birth.processing_error", Language::English, "❌ Error processing birth fortune: {}");
        
        self.add_translation("language.set_chinese", Language::Chinese, "✅ 语言已设置为中文");
        self.add_translation("language.set_chinese", Language::English, "✅ Language set to Chinese");
        
        self.add_translation("language.set_english", Language::Chinese, "✅ Language set to English");
        self.add_translation("language.set_english", Language::English, "✅ Language set to English");
        
        self.add_translation("language.first_time", Language::Chinese, "🌍 首次使用，请选择语言 / First time use, please select language:");
        self.add_translation("language.first_time", Language::English, "🌍 First time use, please select language:");
        
        self.add_translation("language.cannot_save", Language::Chinese, "⚠️ 无法保存语言设置 / Cannot save language setting: {}");
        self.add_translation("language.cannot_save", Language::English, "⚠️ Cannot save language setting: {}");
        
        self.add_translation("language.invalid_option", Language::Chinese, "❌ 无效的语言选项 / Invalid language option: {}");
        self.add_translation("language.invalid_option", Language::English, "❌ Invalid language option: {}");
        
        self.add_translation("language.use_zh_en", Language::Chinese, "请使用: zh/en / Please use: zh/en");
        self.add_translation("language.use_zh_en", Language::English, "Please use: zh/en");
        
        // Fortune messages
        self.add_translation("fortune.excellent", Language::Chinese, "🌟 今日技术运势极佳！");
        self.add_translation("fortune.excellent", Language::English, "🌟 Excellent tech fortune today!");
        
        self.add_translation("fortune.great", Language::Chinese, "✨ 今日工作运势很好");
        self.add_translation("fortune.great", Language::English, "✨ Great work fortune today");
        
        self.add_translation("fortune.good", Language::Chinese, "👍 今日运势不错");
        self.add_translation("fortune.good", Language::English, "👍 Good fortune today");
        
        self.add_translation("fortune.average", Language::Chinese, "😐 今日运势一般");
        self.add_translation("fortune.average", Language::English, "😐 Average fortune today");
        
        self.add_translation("fortune.careful", Language::Chinese, "⚠️ 今日需要小心");
        self.add_translation("fortune.careful", Language::English, "⚠️ Be careful today");
        
        self.add_translation("fortune.poor", Language::Chinese, "😰 今日运势不佳");
        self.add_translation("fortune.poor", Language::English, "😰 Poor fortune today");
        
        self.add_translation("fortune.challenging", Language::Chinese, "🚨 今日技术挑战很大");
        self.add_translation("fortune.challenging", Language::English, "🚨 Major technical challenges today");
        
        self.add_translation("fortune.unknown", Language::Chinese, "🤔 今日运势未知");
        self.add_translation("fortune.unknown", Language::English, "🤔 Unknown fortune today");
        
        // Fortune advice
        self.add_translation("fortune.advice.excellent", Language::Chinese, "适合进行重要的代码重构或系统升级，一切都会很顺利。");
        self.add_translation("fortune.advice.excellent", Language::English, "Perfect for important code refactoring or system upgrades, everything will go smoothly.");
        
        self.add_translation("fortune.advice.great", Language::Chinese, "代码质量会很高，bug率很低，适合处理复杂的技术问题。");
        self.add_translation("fortune.advice.great", Language::English, "Code quality will be high, low bug rate, suitable for handling complex technical issues.");
        
        self.add_translation("fortune.advice.good", Language::Chinese, "编程效率较高，但要注意代码审查，避免小错误。");
        self.add_translation("fortune.advice.good", Language::English, "High programming efficiency, but pay attention to code review, avoid small errors.");
        
        self.add_translation("fortune.advice.average", Language::Chinese, "写代码时要多测试，避免留下隐患。");
        self.add_translation("fortune.advice.average", Language::English, "Write code carefully, avoid leaving hidden dangers.");
        
        self.add_translation("fortune.advice.careful", Language::Chinese, "代码容易出错，建议多写单元测试，仔细检查逻辑。");
        self.add_translation("fortune.advice.careful", Language::English, "Code is prone to errors, suggest writing more unit tests, check logic carefully.");
        
        self.add_translation("fortune.advice.poor", Language::Chinese, "容易遇到技术难题，建议多查阅文档，不要急于求成。");
        self.add_translation("fortune.advice.poor", Language::English, "Easy to encounter technical difficulties, suggest consulting documentation, don't rush.");
        
        self.add_translation("fortune.advice.challenging", Language::Chinese, "代码容易出bug，建议专注于简单任务，避免复杂操作。");
        self.add_translation("fortune.advice.challenging", Language::English, "Code is prone to bugs, suggest focusing on simple tasks, avoid complex operations.");
        
        self.add_translation("fortune.advice.unknown", Language::Chinese, "保持平常心，专注工作即可。");
        self.add_translation("fortune.advice.unknown", Language::English, "Keep calm and focus on work.");
        
        // Personality traits
        self.add_translation("personality.leader", Language::Chinese, "领导能力强");
        self.add_translation("personality.leader", Language::English, "Natural leader");
        
        self.add_translation("personality.independent", Language::Chinese, "独立自主");
        self.add_translation("personality.independent", Language::English, "Independent");
        
        self.add_translation("personality.innovative", Language::Chinese, "创新思维");
        self.add_translation("personality.innovative", Language::English, "Innovative");
        
        self.add_translation("personality.cooperative", Language::Chinese, "合作精神好");
        self.add_translation("personality.cooperative", Language::English, "Cooperative");
        
        self.add_translation("personality.intuitive", Language::Chinese, "直觉敏锐");
        self.add_translation("personality.intuitive", Language::English, "Intuitive");
        
        self.add_translation("personality.diplomatic", Language::Chinese, "善于协调");
        self.add_translation("personality.diplomatic", Language::English, "Diplomatic");
        
        self.add_translation("personality.creative", Language::Chinese, "创造力丰富");
        self.add_translation("personality.creative", Language::English, "Creative");
        
        self.add_translation("personality.expressive", Language::Chinese, "表达能力强");
        self.add_translation("personality.expressive", Language::English, "Expressive");
        
        self.add_translation("personality.optimistic", Language::Chinese, "乐观开朗");
        self.add_translation("personality.optimistic", Language::English, "Optimistic");
        
        self.add_translation("personality.reliable", Language::Chinese, "踏实可靠");
        self.add_translation("personality.reliable", Language::English, "Reliable");
        
        self.add_translation("personality.organized", Language::Chinese, "组织能力强");
        self.add_translation("personality.organized", Language::English, "Organized");
        
        self.add_translation("personality.detail_oriented", Language::Chinese, "注重细节");
        self.add_translation("personality.detail_oriented", Language::English, "Detail-oriented");
        
        self.add_translation("personality.adventurous", Language::Chinese, "自由奔放");
        self.add_translation("personality.adventurous", Language::English, "Adventurous");
        
        self.add_translation("personality.adaptable", Language::Chinese, "适应力强");
        self.add_translation("personality.adaptable", Language::English, "Adaptable");
        
        self.add_translation("personality.curious", Language::Chinese, "好奇心重");
        self.add_translation("personality.curious", Language::English, "Curious");
        
        self.add_translation("personality.responsible", Language::Chinese, "责任心强");
        self.add_translation("personality.responsible", Language::English, "Responsible");
        
        self.add_translation("personality.caring", Language::Chinese, "关爱他人");
        self.add_translation("personality.caring", Language::English, "Caring");
        
        self.add_translation("personality.artistic", Language::Chinese, "艺术天赋");
        self.add_translation("personality.artistic", Language::English, "Artistic");
        
        self.add_translation("personality.analytical", Language::Chinese, "分析能力强");
        self.add_translation("personality.analytical", Language::English, "Analytical");
        
        self.add_translation("personality.truth_seeker", Language::Chinese, "追求真理");
        self.add_translation("personality.truth_seeker", Language::English, "Truth-seeker");
        
        self.add_translation("personality.business_minded", Language::Chinese, "商业头脑");
        self.add_translation("personality.business_minded", Language::English, "Business-minded");
        
        self.add_translation("personality.ambitious", Language::Chinese, "执行力强");
        self.add_translation("personality.ambitious", Language::English, "Ambitious");
        
        self.add_translation("personality.goal_oriented", Language::Chinese, "目标明确");
        self.add_translation("personality.goal_oriented", Language::English, "Goal-oriented");
        
        self.add_translation("personality.compassionate", Language::Chinese, "博爱精神");
        self.add_translation("personality.compassionate", Language::English, "Compassionate");
        
        self.add_translation("personality.wise", Language::Chinese, "智慧超群");
        self.add_translation("personality.wise", Language::English, "Wise");
        
        self.add_translation("personality.idealistic", Language::Chinese, "理想主义");
        self.add_translation("personality.idealistic", Language::English, "Idealistic");
        
        self.add_translation("personality.mysterious", Language::Chinese, "神秘莫测");
        self.add_translation("personality.mysterious", Language::English, "Mysterious");
        
        self.add_translation("personality.enigmatic", Language::Chinese, "深藏不露");
        self.add_translation("personality.enigmatic", Language::English, "Enigmatic");
        
        // Career advice
        self.add_translation("career.leadership", Language::Chinese, "适合担任领导职位，创业或管理岗位");
        self.add_translation("career.leadership", Language::English, "Suitable for leadership, entrepreneurship or management");
        
        self.add_translation("career.teamwork", Language::Chinese, "适合团队合作，咨询或协调工作");
        self.add_translation("career.teamwork", Language::English, "Suitable for teamwork, consulting or coordination");
        
        self.add_translation("career.creative", Language::Chinese, "适合创意行业，艺术或传媒领域");
        self.add_translation("career.creative", Language::English, "Suitable for creative industries, arts or media");
        
        self.add_translation("career.technical", Language::Chinese, "适合技术工作，工程或项目管理");
        self.add_translation("career.technical", Language::English, "Suitable for technical work, engineering or project management");
        
        self.add_translation("career.sales", Language::Chinese, "适合销售，市场或自由职业");
        self.add_translation("career.sales", Language::English, "Suitable for sales, marketing or freelancing");
        
        self.add_translation("career.service", Language::Chinese, "适合教育，医疗或服务行业");
        self.add_translation("career.service", Language::English, "Suitable for education, healthcare or service industries");
        
        self.add_translation("career.research", Language::Chinese, "适合研究，分析或技术开发");
        self.add_translation("career.research", Language::English, "Suitable for research, analysis or technical development");
        
        self.add_translation("career.finance", Language::Chinese, "适合金融，商业或投资领域");
        self.add_translation("career.finance", Language::English, "Suitable for finance, business or investment");
        
        self.add_translation("career.charity", Language::Chinese, "适合公益，教育或哲学研究");
        self.add_translation("career.charity", Language::English, "Suitable for charity, education or philosophy");
        
        self.add_translation("career.mysticism", Language::Chinese, "适合神秘学，心理学或咨询工作");
        self.add_translation("career.mysticism", Language::English, "Suitable for mysticism, psychology or counseling");
        
        // CLI help text
        self.add_translation("cli.about", Language::Chinese, "显示今日技术工作运势 / Show today's tech work fortune");
        self.add_translation("cli.about", Language::English, "Show today's tech work fortune");
        
        self.add_translation("cli.verbose_help", Language::Chinese, "显示详细帮助信息 / Show verbose help information");
        self.add_translation("cli.verbose_help", Language::English, "Show verbose help information");
        
        self.add_translation("cli.date_help", Language::Chinese, "指定日期 (格式: YYYY-MM-DD) / Specify date (format: YYYY-MM-DD)");
        self.add_translation("cli.date_help", Language::English, "Specify date (format: YYYY-MM-DD)");
        
        self.add_translation("cli.language_help", Language::Chinese, "语言选择 / Language selection (zh/en)");
        self.add_translation("cli.language_help", Language::English, "Language selection (zh/en)");
        
        self.add_translation("cli.set_language_help", Language::Chinese, "设置语言 / Set language");
        self.add_translation("cli.set_language_help", Language::English, "Set language");
        
        self.add_translation("cli.birth_help", Language::Chinese, "出生日期运势分析 / Birth date fortune analysis");
        self.add_translation("cli.birth_help", Language::English, "Birth date fortune analysis");
        
        self.add_translation("cli.birth_date_help", Language::Chinese, "出生日期 (格式: YYYY-MM-DD) / Birth date (format: YYYY-MM-DD)");
        self.add_translation("cli.birth_date_help", Language::English, "Birth date (format: YYYY-MM-DD)");
        
        // Comments
        self.add_translation("comment.handle_language_setting", Language::Chinese, "处理语言设置");
        self.add_translation("comment.handle_language_setting", Language::English, "Handle language setting");
        
        self.add_translation("comment.determine_language", Language::Chinese, "确定使用的语言");
        self.add_translation("comment.determine_language", Language::English, "Determine language to use");
        
        self.add_translation("comment.handle_birth_fortune", Language::Chinese, "处理出生日期运势分析");
        self.add_translation("comment.handle_birth_fortune", Language::English, "Handle birth date fortune analysis");
        
        self.add_translation("comment.generate_daily_fortune", Language::Chinese, "生成今日运势");
        self.add_translation("comment.generate_daily_fortune", Language::English, "Generate daily fortune");
        
        self.add_translation("comment.display_fortune", Language::Chinese, "显示运势");
        self.add_translation("comment.display_fortune", Language::English, "Display fortune");
        
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
        
        // Zodiac signs
        self.add_translation("zodiac.aries", Language::Chinese, "白羊座");
        self.add_translation("zodiac.aries", Language::English, "Aries");
        
        self.add_translation("zodiac.taurus", Language::Chinese, "金牛座");
        self.add_translation("zodiac.taurus", Language::English, "Taurus");
        
        self.add_translation("zodiac.gemini", Language::Chinese, "双子座");
        self.add_translation("zodiac.gemini", Language::English, "Gemini");
        
        self.add_translation("zodiac.cancer", Language::Chinese, "巨蟹座");
        self.add_translation("zodiac.cancer", Language::English, "Cancer");
        
        self.add_translation("zodiac.leo", Language::Chinese, "狮子座");
        self.add_translation("zodiac.leo", Language::English, "Leo");
        
        self.add_translation("zodiac.virgo", Language::Chinese, "处女座");
        self.add_translation("zodiac.virgo", Language::English, "Virgo");
        
        self.add_translation("zodiac.libra", Language::Chinese, "天秤座");
        self.add_translation("zodiac.libra", Language::English, "Libra");
        
        self.add_translation("zodiac.scorpio", Language::Chinese, "天蝎座");
        self.add_translation("zodiac.scorpio", Language::English, "Scorpio");
        
        self.add_translation("zodiac.sagittarius", Language::Chinese, "射手座");
        self.add_translation("zodiac.sagittarius", Language::English, "Sagittarius");
        
        self.add_translation("zodiac.capricorn", Language::Chinese, "摩羯座");
        self.add_translation("zodiac.capricorn", Language::English, "Capricorn");
        
        self.add_translation("zodiac.aquarius", Language::Chinese, "水瓶座");
        self.add_translation("zodiac.aquarius", Language::English, "Aquarius");
        
        self.add_translation("zodiac.pisces", Language::Chinese, "双鱼座");
        self.add_translation("zodiac.pisces", Language::English, "Pisces");
        
        self.add_translation("zodiac.unknown", Language::Chinese, "未知");
        self.add_translation("zodiac.unknown", Language::English, "Unknown");
        
        // Language parsing
        self.add_translation("lang.chinese", Language::Chinese, "中文");
        self.add_translation("lang.chinese", Language::English, "Chinese");
        
        self.add_translation("lang.english", Language::Chinese, "英文");
        self.add_translation("lang.english", Language::English, "English");
        
        // Detailed advice messages
        self.add_translation("advice.excellent.refactor", Language::Chinese, "适合进行代码重构和系统优化");
        self.add_translation("advice.excellent.refactor", Language::English, "Suitable for code refactoring and system optimization");
        
        self.add_translation("advice.excellent.framework", Language::Chinese, "可以尝试新的技术栈或框架");
        self.add_translation("advice.excellent.framework", Language::English, "Can try new tech stacks or frameworks");
        
        self.add_translation("advice.excellent.review", Language::Chinese, "代码审查会非常顺利");
        self.add_translation("advice.excellent.review", Language::English, "Code review will be very smooth");
        
        self.add_translation("advice.excellent.problem", Language::Chinese, "适合解决复杂的技术难题");
        self.add_translation("advice.excellent.problem", Language::English, "Suitable for solving complex technical problems");
        
        self.add_translation("advice.great.efficiency", Language::Chinese, "编程效率很高，bug率低");
        self.add_translation("advice.great.efficiency", Language::English, "High programming efficiency, low bug rate");
        
        self.add_translation("advice.great.development", Language::Chinese, "适合处理重要的功能开发");
        self.add_translation("advice.great.development", Language::English, "Suitable for important feature development");
        
        self.add_translation("advice.great.quality", Language::Chinese, "代码质量会很好");
        self.add_translation("advice.great.quality", Language::English, "Code quality will be excellent");
        
        self.add_translation("advice.great.standards", Language::Chinese, "注意保持代码规范");
        self.add_translation("advice.great.standards", Language::English, "Pay attention to code standards");
        
        self.add_translation("advice.good.condition", Language::Chinese, "整体工作状态不错");
        self.add_translation("advice.good.condition", Language::English, "Overall work condition is good");
        
        self.add_translation("advice.good.check", Language::Chinese, "写代码时要仔细检查");
        self.add_translation("advice.good.check", Language::English, "Check code carefully when writing");
        
        self.add_translation("advice.good.documentation", Language::Chinese, "建议多写注释和文档");
        self.add_translation("advice.good.documentation", Language::English, "Suggest writing more comments and documentation");
        
        self.add_translation("advice.good.optimization", Language::Chinese, "避免过度优化");
        self.add_translation("advice.good.optimization", Language::English, "Avoid over-optimization");
        
        self.add_translation("advice.average.careful", Language::Chinese, "编程时要多加小心");
        self.add_translation("advice.average.careful", Language::English, "Be extra careful when programming");
        
        self.add_translation("advice.average.tests", Language::Chinese, "建议多写单元测试");
        self.add_translation("advice.average.tests", Language::English, "Suggest writing more unit tests");
        
        self.add_translation("advice.average.logic", Language::Chinese, "避免复杂的逻辑判断");
        self.add_translation("advice.average.logic", Language::English, "Avoid complex logic judgments");
        
        self.add_translation("advice.average.review", Language::Chinese, "代码审查要仔细");
        self.add_translation("advice.average.review", Language::English, "Code review should be thorough");
        
        self.add_translation("advice.careful.mistakes", Language::Chinese, "今日容易出错，需要格外小心");
        self.add_translation("advice.careful.mistakes", Language::English, "Easy to make mistakes today, need extra care");
        
        self.add_translation("advice.careful.simple", Language::Chinese, "建议专注于简单任务");
        self.add_translation("advice.careful.simple", Language::English, "Suggest focusing on simple tasks");
        
        self.add_translation("advice.careful.documentation", Language::Chinese, "多查阅文档和参考资料");
        self.add_translation("advice.careful.documentation", Language::English, "Consult documentation and references more");
        
        self.add_translation("advice.careful.rush", Language::Chinese, "避免急于求成");
        self.add_translation("advice.careful.rush", Language::English, "Avoid rushing things");
        
        self.add_translation("advice.poor.challenges", Language::Chinese, "技术挑战很大，建议谨慎行事");
        self.add_translation("advice.poor.challenges", Language::English, "Major technical challenges, suggest proceeding with caution");
        
        self.add_translation("advice.poor.complex", Language::Chinese, "避免处理复杂的技术问题");
        self.add_translation("advice.poor.complex", Language::English, "Avoid handling complex technical issues");
        
        self.add_translation("advice.poor.help", Language::Chinese, "多向同事请教");
        self.add_translation("advice.poor.help", Language::English, "Ask colleagues for help more");
        
        self.add_translation("advice.poor.patience", Language::Chinese, "保持耐心，不要急躁");
        self.add_translation("advice.poor.patience", Language::English, "Keep patience, don't be impatient");
        
        self.add_translation("advice.terrible.fortune", Language::Chinese, "今日技术运势很差，建议");
        self.add_translation("advice.terrible.fortune", Language::English, "Poor technical fortune today, suggest");
        
        self.add_translation("advice.terrible.learning", Language::Chinese, "专注于学习和文档阅读");
        self.add_translation("advice.terrible.learning", Language::English, "Focus on learning and documentation reading");
        
        self.add_translation("advice.terrible.avoid", Language::Chinese, "避免重要的代码修改");
        self.add_translation("advice.terrible.avoid", Language::English, "Avoid important code modifications");
        
        self.add_translation("advice.terrible.attitude", Language::Chinese, "保持积极心态，明天会更好");
        self.add_translation("advice.terrible.attitude", Language::English, "Keep positive attitude, tomorrow will be better");
    }

    fn add_translation(&mut self, key: &str, lang: Language, value: &str) {
        self.translations
            .entry(key.to_string())
            .or_insert_with(HashMap::new)
            .insert(lang, value.to_string());
    }
}

pub fn i18n(key: &str, lang: Language) -> String {
    static I18N: std::sync::OnceLock<I18n> = std::sync::OnceLock::new();
    I18N.get_or_init(I18n::new).t(key, lang)
}

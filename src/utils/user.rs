use std::env;

/// Detects the display name for the current user.
///
/// Preference order:
/// 1. `FINDME_USER_NAME`
/// 2. Common OS specific username environment variables (`USER`, `USERNAME`, `LOGNAME`).
/// 3. Fallback to "Developer".
pub fn detect_user_name() -> String {
    const CANDIDATES: [&str; 4] = ["FINDME_USER_NAME", "USER", "USERNAME", "LOGNAME"];

    for key in CANDIDATES {
        if let Ok(value) = env::var(key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }

    "Developer".to_string()
}

use std::io::BufRead;

pub fn find_matches(reader: impl BufRead, pattern: &str, mut writer: impl std::io::Write) {
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                writeln!(writer, "{}", line).ok();
            }
        }
    }
}
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    // patternに合致する行をファイルの中から取得して表示
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

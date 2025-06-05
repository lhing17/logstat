/// 统计指定内容的行数
pub fn count_lines(content: &str, filters: &[String]) -> usize {
    if !filters.is_empty() {
        content
            .lines()
            .filter(|line| filters.iter().all(|f_str| line.contains(f_str)))
            .count()
    } else {
        content.lines().count()
    }
}

/// 从文件或标准输入读取内容
pub fn read_content(file_path: &str) -> anyhow::Result<String> {
    if file_path == "-" {
        std::io::read_to_string(std::io::stdin())
            .map_err(|e| anyhow::anyhow!("无法从标准输入读取: {}", e))
    } else {
        std::fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("无法读取文件 {}: {}", file_path, e))
    }
}

/// 获取输入文件，如果没有指定文件，则使用标准输入
pub fn get_input_files(files: &Vec<String>) -> Vec<String> {
    if files.is_empty() {
        vec!["-".to_string()]
    } else {
        files.clone()
    }
}
/// 统计指定内容的行数
fn count_lines(content: &str, filters: &[String]) -> usize {
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
fn read_content(file_path: &str) -> anyhow::Result<String> {
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

pub fn process_files<'a>(
    files: &'a [String],
    filters: &[String],
) -> anyhow::Result<(usize, usize, Vec<(&'a String, usize, usize)>)> {
    let mut total_all_lines = 0;
    let mut total_matched_lines = 0;
    let mut results = Vec::with_capacity(files.len());

    for file_path in files {
        // 读取文件内容，如果文件路径为"-"，则从标准输入读取
        let content = read_content(file_path)?;

        // 计算行数
        let all_lines = content.lines().count();
        let matched_lines = count_lines(&content, filters);

        // 累加总数
        total_all_lines += all_lines;
        total_matched_lines += matched_lines;

        // 保存每个文件的结果
        results.push((file_path, all_lines, matched_lines));
    }

    Ok((total_all_lines, total_matched_lines, results))
}

use std::io::BufRead;

use regex::Regex;

/// 从文件或标准输入读取内容并计算行数
fn process_content<R: BufRead>(
    reader: R,
    filters: &[String],
    patterns: &[Regex],
) -> (usize, usize) {
    let mut all_lines = 0;
    let mut matched_lines = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                all_lines += 1;
                let filter_match = filters.is_empty() || filters.iter().all(|f_str| line.contains(f_str));
                let pattern_match = patterns.is_empty() || patterns.iter().any(|p| p.is_match(&line));
                if filter_match && pattern_match {
                    matched_lines += 1;
                }
            }
            Err(_) => {
                continue; // 忽略读取错误的行
            }
        }
    }

    (all_lines, matched_lines)
}

fn process_single_file(
    file_path: &str,
    filters: &[String],
    patterns: &[Regex],
) -> anyhow::Result<(usize, usize)> {
    if file_path == "-" {
        // 处理标准输入
        let stdin = std::io::stdin();
        let reader = std::io::BufReader::new(stdin.lock());
        Ok(process_content(reader, filters, patterns))
    } else {
        // 处理普通文件
        let file = std::fs::File::open(file_path)
            .map_err(|e| anyhow::anyhow!("无法打开文件 {}: {}", file_path, e))?;
        let reader = std::io::BufReader::new(file);
        Ok(process_content(reader, filters, patterns))
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
    patterns: &[Regex],
) -> anyhow::Result<(usize, usize, Vec<(&'a String, usize, usize)>)> {
    let mut total_all_lines = 0;
    let mut total_matched_lines = 0;
    let mut results = Vec::with_capacity(files.len());

    for file_path in files {
        // 读取文件内容并计算行数
        let (all_lines, matched_lines) = process_single_file(file_path, filters, patterns)?;

        // 累加总数
        total_all_lines += all_lines;
        total_matched_lines += matched_lines;

        // 保存每个文件的结果
        results.push((file_path, all_lines, matched_lines));
    }

    Ok((total_all_lines, total_matched_lines, results))
}

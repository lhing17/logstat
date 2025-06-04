use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: Vec<String>,

    #[arg(short, long)]
    total: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short = 'F', long)]
    filter: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // 如果没有指定文件，则使用标准输入
    let files = if (&args.file).is_empty() {
        vec!["-".to_string()]
    } else {
        args.file
    };

    // 用于存储结果的结构
    let mut total_all_lines = 0;
    let mut total_matched_lines = 0;
    let mut results = Vec::with_capacity(files.len());

    for file_path in &files {
        // 读取文件内容，如果文件路径为"-"，则从标准输入读取
        let content = if file_path == "-" {
            std::io::read_to_string(std::io::stdin())
                .map_err(|e| anyhow::anyhow!("无法从标准输入读取: {}", e))?
        } else {
            std::fs::read_to_string(&file_path)
                .map_err(|e| anyhow::anyhow!("无法读取文件 {}: {}", file_path, e))?
        };

        // 计算行数
        let all_lines = content.lines().count();
        let matched_lines = count_lines(&content, &args.filter);

        // 累加总数
        total_all_lines += all_lines;
        total_matched_lines += matched_lines;

        // 保存每个文件的结果
        results.push((file_path, all_lines, matched_lines));
    }

     // 输出结果
     print_results(&files, &results, total_all_lines, total_matched_lines, args.verbose, args.total);

    Ok(())
}

// 打印结果的函数
fn print_results(
    files: &[String],
    results: &[(&String, usize, usize)],
    total_all_lines: usize,
    total_matched_lines: usize,
    verbose: bool,
    show_total: bool,
) {
    // 确定表头
    let header = if show_total {
        "文件路径\t总行数\t匹配行数"
    } else {
        "文件路径\t匹配行数"
    };
    
    if verbose {
        // 详细模式：显示每个文件的信息
        println!("{}", header);
        for (file_path, all_lines, matched_lines) in results {
            if show_total {
                println!("{}	{}	{}", file_path, all_lines, matched_lines);
            } else {
                println!("{}	{}", file_path, matched_lines);
            }
        }
    } else {
        // 简洁模式：只显示汇总信息
        let stats = if show_total {
            format!("{}	{}	{}", files.join(" "), total_all_lines, total_matched_lines)
        } else {
            format!("{}	{}", files.join(" "), total_matched_lines)
        };
        println!("{}
{}", header, stats);
    }
}


// 在文件中添加以下函数定义
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

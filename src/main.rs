use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: Vec<String>,

    #[arg(short, long)]
    total: bool,

    #[arg(short = 'F', long)]
    filter: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // 初始化总行数
    let mut total_lines = 0;
    let mut matched_lines = 0;

    let files = if (&args.file).is_empty() {
        vec!["-".to_string()]
    } else {
        args.file
    };

    for file_path in &files {
        // 读取文件内容，如果文件路径为"-"，则从标准输入读取
        let content = if file_path == "-" {
            std::io::read_to_string(std::io::stdin())
                .map_err(|e| anyhow::anyhow!("无法从标准输入读取: {}", e))?
        } else {
            std::fs::read_to_string(&file_path)
                .map_err(|e| anyhow::anyhow!("无法读取文件 {}: {}", file_path, e))?
        };

        // 累加总行数
        matched_lines += count_lines(&content, &args.filter);
        if args.total {
            total_lines += content.lines().count();
        }
    }

    if args.total {
        println!("文件路径\t总行数\t匹配行数");
        println!("{}\t{}\t{}", files.join(" "), total_lines, matched_lines);
    } else {
        println!("文件路径\t匹配行数");
        println!("{}\t{}", files.join(" "), matched_lines);
    }

    Ok(())
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

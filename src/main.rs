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

    // 初始化总行数
    let mut total_lines = 0;
    let mut matched_lines = 0;
    let mut output = String::new();

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
        let count_lines = count_lines(&content, &args.filter);
        
        if args.verbose {
            let count = content.lines().count();
            if args.total {
                output.push_str(&format!("{}\t{}\t{}\n", file_path, count, count_lines));
            } else {
                output.push_str(&format!("{}\t{}\n", file_path, count_lines));
            }
        }
        matched_lines += count_lines;
        if args.total {
            let count = content.lines().count();
            total_lines += count;
        }
    }

    let header = if args.total { "文件路径\t总行数\t匹配行数" } else { "文件路径\t匹配行数" };
    let stats = if args.total {
        format!("{}\t{}\t{}", files.join(" "), total_lines, matched_lines)
    } else {
        format!("{}\t{}", files.join(" "), matched_lines)
    };
    if args.verbose {
        println!("{}\n{}", header, output);
    } else {
        println!("{}\n{}", header, stats);
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

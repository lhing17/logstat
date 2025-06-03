use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: Vec<String>,

    #[arg(short = 'F', long)]
    filter: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut total_lines = 0; // 初始化总行数

    for file_path in args.file {
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| anyhow::anyhow!("无法读取文件 {}: {}", file_path, e))?;

        // 如果包含filter，则统计包含filter字符串的行数，否则统计所有行数
        let lines = if !args.filter.is_empty() {
            content
                .lines()
                .filter(|line| args.filter.iter().all(|f_str| line.contains(f_str)))
                .count()
        } else {
            content.lines().count()
        };
        total_lines += lines; // 累加总行数
    }

    println!("日志文件包含{}行", total_lines);

    test();

    Ok(())
}

fn test() {
    let s1 = String::from("hello");
    println!(" {}", s1);
}

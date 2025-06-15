mod output;
mod processing;

use clap::Parser;
use regex::Regex;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    file: Vec<String>,

    #[arg(short, long)]
    total: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_value = "text")]
    format: String,

    #[arg(short = 'F', long)]
    filter: Vec<String>,

    #[arg(short = 'P', long)]
    pattern: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let start_time = Instant::now(); // 记录程序开始时间
    let args = Cli::parse();

    // 将字符串模式编译为正则表达式
    let patterns: Vec<Regex> = args.pattern.iter()
    .filter_map(|p| match Regex::new(p) {
       Ok(re) => Some(re),
       Err(e) => {
           eprintln!("警告：无效的正则表达式: {} - {}", p, e);
           None
       }
    }).collect();

    // 如果没有指定文件，则使用标准输入
    let files = processing::get_input_files(&args.file);

    // 处理文件，统计总行数和匹配行数
    let (total_all_lines, total_matched_lines, results) = processing::process_files(&files, &args.filter, &patterns)?;

    // 输出结果
    output::print_results(
        &files,
        &results,
        total_all_lines,
        total_matched_lines,
        args.verbose,
        args.total,
        args.format.as_str(),
    );

    // 计算程序运行时间
    let elapsed_time = start_time.elapsed(); 

    // 将执行时间打印到标准错误输出。
    // 选择 eprintln! 而不是 println! 是为了将计时信息与程序的正常输出（行数统计结果）分开，
    // 这样即使程序输出被重定向到文件，计时信息仍然会显示在终端上。
    eprintln!("程序运行时间: {:?}", elapsed_time);

    Ok(())
}


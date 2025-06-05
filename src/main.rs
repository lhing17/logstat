mod output;
mod processing;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    file: Vec<String>,

    #[arg(short, long)]
    total: bool,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long, default_value = "normal")]
    format: String,

    #[arg(short = 'F', long)]
    filter: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // 如果没有指定文件，则使用标准输入
    let files = processing::get_input_files(&args.file);

    // 处理文件，统计总行数和匹配行数
    let (total_all_lines, total_matched_lines, results) = processing::process_files(&files, &args.filter)?;

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

    Ok(())
}


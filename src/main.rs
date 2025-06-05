mod processing;
mod output;

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

    // 用于存储结果的结构
    let mut total_all_lines = 0;
    let mut total_matched_lines = 0;
    let mut results = Vec::with_capacity(files.len());

    for file_path in &files {
        // 读取文件内容，如果文件路径为"-"，则从标准输入读取
        let content = processing::read_content(file_path)?;

        // 计算行数
        let all_lines = content.lines().count();
        let matched_lines = processing::count_lines(&content, &args.filter);

        // 累加总数
        total_all_lines += all_lines;
        total_matched_lines += matched_lines;

        // 保存每个文件的结果
        results.push((file_path, all_lines, matched_lines));
    }

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


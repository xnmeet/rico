use clap::Parser;
use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use thrift_parser::parser::Parser as ThriftParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path containing thrift files
    #[arg(short, long)]
    path: String,

    /// Output directory for JSON files
    #[arg(short, long, default_value = "output")]
    output: String,
}

/// 记录处理状态
#[derive(Default)]
struct ProcessStats {
    processed_count: AtomicUsize,
    failed_files: Mutex<Vec<(PathBuf, String)>>,
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();

    let input_path = Path::new(&args.path);
    let output_path = Path::new(&args.output);

    // 创建输出目录
    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Failed to create output directory");
    }

    // 查找所有 thrift 文件
    let thrift_files = collect_thrift_files(input_path);
    let total_files = thrift_files.len();
    println!(
        "{} {} thrift files found",
        "•".green().bold(),
        total_files.to_string().green().bold()
    );

    let stats = Arc::new(ProcessStats {
        processed_count: AtomicUsize::new(0),
        failed_files: Mutex::new(Vec::new()),
    });

    // 进度条设置
    let multi = MultiProgress::new();
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.magenta} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("#>-");

    let pb = multi.add(ProgressBar::new(total_files as u64));
    pb.set_style(progress_style);
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    // 并行处理文件
    thrift_files.par_iter().for_each(|file| {
        process_thrift_file(file, output_path, Arc::clone(&stats), &pb);
    });

    pb.finish_and_clear();

    let elapsed = start_time.elapsed().as_secs_f32();

    // 打印最终结果
    let failed_files = stats.failed_files.lock().unwrap();
    let success_count = total_files - failed_files.len();

    let summary_title = format!("{} Summary", "🚀".yellow());
    println!("{}", summary_title.bold().yellow());
    println!(
        "{} {} files processed in {:.2}s",
        "•".green().bold(),
        total_files.to_string().green().bold(),
        elapsed
    );
    println!(
        "{} {} succeeded",
        "•".green().bold(),
        success_count.to_string().green().bold()
    );

    if failed_files.is_empty() {
        println!("{} All files processed successfully!", "✓".green().bold());
    } else {
        print_errors_table(&failed_files);
    }
}

/// 深度遍历目录，收集 .thrift 文件
fn collect_thrift_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_thrift_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "thrift" {
                    files.push(path);
                }
            }
        }
    }
    files
}

/// 处理单个 thrift 文件
fn process_thrift_file(
    input_file: &Path,
    output_dir: &Path,
    stats: Arc<ProcessStats>,
    pb: &ProgressBar,
) {
    let file_stem = input_file.file_stem().unwrap().to_str().unwrap();
    let output_file = output_dir.join(format!("{}.json", file_stem));

    pb.set_message(format!(
        "Parsing {:?}",
        input_file.file_name().unwrap_or_default()
    ));

    let result = fs::read_to_string(input_file).and_then(|content| {
        let mut parser = ThriftParser::new(&content);
        parser
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            .and_then(|value| {
                serde_json::to_string_pretty(&value)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })
            .and_then(|json_output| fs::write(&output_file, json_output))
    });

    match result {
        Ok(_) => {
            stats.processed_count.fetch_add(1, Ordering::SeqCst);
        }
        Err(e) => {
            stats.processed_count.fetch_add(1, Ordering::SeqCst);
            stats
                .failed_files
                .lock()
                .unwrap()
                .push((input_file.to_path_buf(), e.to_string()));
        }
    }

    pb.inc(1);
}

/// 显示失败文件列表
fn print_errors_table(failed_files: &[(PathBuf, String)]) {
    if failed_files.is_empty() {
        return;
    }

    println!("{}", "\nFailed files:".red().bold());

    for (i, (file, error)) in failed_files.iter().enumerate() {
        println!(
            "{}{}",
            "┌─".bright_blue().bold(),
            file.display().to_string().red().bold()
        );
        println!("{}{}", "└─".bright_blue().bold(), error.bright_red());

        // 如果不是最后一个错误，添加空行
        if i < failed_files.len() - 1 {
            println!();
        }
    }
}

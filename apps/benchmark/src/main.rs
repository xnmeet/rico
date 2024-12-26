use colored::*;
use humantime::format_duration;
use indicatif::{ProgressBar, ProgressStyle};
use memory_stats::memory_stats;
use rico_parser::Parser;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

struct BenchmarkResult {
    file_name: String,
    file_size: u64,
    parse_time: Duration,
    memory_used: u64,
    success: bool,
    error: Option<String>,
}

struct ThriftBenchmark {
    results: Vec<BenchmarkResult>,
    total_time: Duration,
    total_memory: u64,
    success_count: usize,
}

impl ThriftBenchmark {
    fn new() -> Self {
        Self {
            results: Vec::new(),
            total_time: Duration::new(0, 0),
            total_memory: 0,
            success_count: 0,
        }
    }

    fn run_benchmark(&mut self, file_path: &Path) -> BenchmarkResult {
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let file_size = fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);

        let start_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);
        let start_time = Instant::now();

        let result = fs::read_to_string(file_path).and_then(|content| {
            let mut parser = Parser::new(&content);
            parser
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        });

        let parse_time = start_time.elapsed();
        let end_memory = memory_stats().map(|m| m.physical_mem).unwrap_or(0);
        let memory_used = end_memory.saturating_sub(start_memory);

        BenchmarkResult {
            file_name,
            file_size,
            parse_time,
            memory_used: memory_used.try_into().unwrap(),
            success: result.is_ok(),
            error: result.err().map(|e| e.to_string()),
        }
    }

    fn print_results(&self) {
        println!("\n{}", "Benchmark Results:".bold().yellow());
        println!("{}", "=================".yellow());

        for result in &self.results {
            let status = if result.success {
                "✓".green()
            } else {
                "✗".red()
            };

            println!(
                "{} {} ({} bytes)",
                status,
                result.file_name,
                result.file_size.to_string().cyan()
            );
            println!(
                "  Time: {}, Memory: {} KB",
                format_duration(result.parse_time).to_string().yellow(),
                (result.memory_used / 1024).to_string().cyan()
            );

            if let Some(error) = &result.error {
                println!("  Error: {}", error.red());
            }
        }

        println!("\n{}", "Summary:".bold().yellow());
        println!("Total files: {}", self.results.len());
        println!("Successful: {}", self.success_count.to_string().green());
        println!(
            "Failed: {}",
            (self.results.len() - self.success_count).to_string().red()
        );
        println!(
            "Total time: {}",
            format_duration(self.total_time).to_string().yellow()
        );
        println!(
            "Average memory per file: {} KB",
            (self.total_memory / self.results.len() as u64 / 1024)
                .to_string()
                .cyan()
        );
    }
}

fn get_tests_dir() -> String {
    // 首先尝试当前目录下的 tests
    if Path::new("tests").exists() {
        return "tests".to_string();
    }
    // 如果不存在，尝试上级目录
    "../tests".to_string()
}

fn main() {
    let test_files = vec!["struct", "service", "enum", "const", "typedef", "common"];
    let mut benchmark = ThriftBenchmark::new();
    let tests_dir = get_tests_dir();

    let pb = ProgressBar::new(test_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("#>-"),
    );

    for file_name in test_files {
        let input_path = format!("{}/{}.thrift", tests_dir, file_name);
        let result = benchmark.run_benchmark(Path::new(&input_path));

        if result.success {
            benchmark.success_count += 1;
        }

        benchmark.total_time += result.parse_time;
        benchmark.total_memory += result.memory_used;
        benchmark.results.push(result);

        pb.inc(1);
    }

    pb.finish_and_clear();
    benchmark.print_results();
}

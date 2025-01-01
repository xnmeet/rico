//! Rico-scan is a high-performance CLI tool for parsing and validating Thrift IDL files.
//!
//! # Features
//!
//! - Fast parallel processing of Thrift files
//! - Detailed error reporting with source context
//! - Optional JSON AST output
//! - Progress indication with ETA
//! - Colorful and informative terminal output
//!
//! # Usage
//!
//! ```bash
//! # Just validate Thrift files
//! rico-scan -p /path/to/thrift/files
//!
//! # Parse and output JSON AST
//! rico-scan -p /path/to/thrift/files -o /path/to/output
//! ```

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::{fs, io};

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{miette, NamedSource, Result};
use rayon::{current_num_threads, prelude::*};
use rico::parser::Parser as ThriftParser;

/// Command line arguments for rico-scan
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A high-performance CLI tool for parsing and validating Thrift IDL files"
)]
struct Args {
    /// Directory path containing thrift files
    #[arg(short, long)]
    path: PathBuf,

    /// Optional output directory for JSON AST files
    /// If not provided, files will only be validated without generating output
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Optional flag to enable pretty JSON output
    #[arg(long)]
    pretty: bool,
}

/// Statistics for tracking file processing progress
struct Stats {
    /// Number of files processed so far
    processed: AtomicUsize,
    /// Total number of files to process
    total: usize,
}

/// Sets up a progress bar with a custom style for file processing
fn setup_progress_bar(total: usize) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
}

/// Collects Thrift files based on the input path
/// If the path is a Thrift file, returns a vector with just that file
/// If the path is a directory, recursively collects all Thrift files
fn collect_thrift_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("thrift") {
            return Ok(vec![path.to_path_buf()]);
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("File '{}' is not a Thrift file", path.display()),
        ));
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_thrift_files(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("thrift") {
            files.push(path);
        }
    }
    Ok(files)
}

/// Writes the AST to a JSON file
/// # Arguments
///
/// * `ast` - The AST to serialize
/// * `output_path` - The path where to write the JSON file
fn write_output(ast: rico::ast::Document, output_path: &Path, pretty: bool) -> io::Result<()> {
    let json = if pretty {
        serde_json::to_string_pretty(&ast)?
    } else {
        serde_json::to_string(&ast)?
    };
    fs::write(output_path, json)
}

/// Processes a single Thrift file
///
/// This function will:
/// 1. Read the file content
/// 2. Parse it using the Rico parser
/// 3. Optionally write the AST as JSON if an output directory is provided
///
/// # Arguments
///
/// * `input` - Path to the input Thrift file
/// * `output_dir` - Optional output directory for JSON files
///
/// # Returns
///
/// * `Ok(())` if processing succeeded
/// * `Err` with a detailed error message if any step failed
fn process_file(input: &Path, output_dir: Option<&Path>, pretty: bool) -> Result<()> {
    let content = fs::read_to_string(input)
        .map_err(|e| miette!("Failed to read {}: {}", input.display(), e))?;
    let mut parser = ThriftParser::new(&content);

    let ast = parser.parse().map_err(|error| {
        miette::Error::new(error).with_source_code(NamedSource::new(
            input.display().to_string(),
            content.clone(),
        ))
    })?;

    if let Some(output_dir) = output_dir {
        {
            let file_name = input
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| miette!("Invalid file name"))?;
            let output_path = output_dir.join(format!("{}.json", file_name));
            write_output(ast, &output_path, pretty)
                .map_err(|e| miette!("Failed to write {}: {}", output_path.display(), e))?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let args = Args::parse();
    let thrift_files = collect_thrift_files(&args.path)
        .map_err(|e| miette!("Failed to collect Thrift files: {}", e))?;

    if thrift_files.is_empty() {
        println!(
            "{} {} {}",
            "!".yellow(),
            "No Thrift files found in".yellow(),
            args.path.display().to_string().yellow().underline()
        );
        return Ok(());
    }

    println!("📝 Found {} Thrift files", thrift_files.len());

    if let Some(output_dir) = &args.output {
        fs::create_dir_all(output_dir)
            .map_err(|e| miette!("Failed to create output directory: {}", e))?;
        println!("📁 Output directory: {}", output_dir.display());
    }

    let stats = Arc::new(Stats {
        processed: AtomicUsize::new(0),
        total: thrift_files.len(),
    });

    let pb = setup_progress_bar(stats.total);

    // Process files in parallel
    let results: Vec<_> = thrift_files
        .par_iter()
        .map(|file| {
            let result = process_file(file, args.output.as_deref(), args.pretty);
            stats.processed.fetch_add(1, Ordering::SeqCst);
            pb.inc(1);
            (file, result)
        })
        .collect();

    pb.finish_and_clear();

    // Report results
    let (success, failures): (Vec<_>, Vec<_>) =
        results.into_iter().partition(|(_, result)| result.is_ok());

    let elapsed = start_time.elapsed();
    let failures_count = failures.len();

    // Only show error details if there are failures
    if !failures.is_empty() {
        println!("\n");
        for (_, error) in failures {
            eprintln!("{:?}", error.err().unwrap());
        }
    }

    // Print summary in one line
    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {}",
        "Done!".bright_green(),
        "•".bright_black(),
        "✅".green(),
        format!("succeeded: {}", success.len()).green(),
        "•".bright_black(),
        "❌".red(),
        format!("failed: {}", failures_count).red(),
        "•".bright_black(),
        "⚡".cyan(),
        format!("threads: {}", current_num_threads()).cyan(),
        "•".bright_black(),
        "⏱".yellow(),
        format!("time: {:.3}s", elapsed.as_secs_f32()).yellow()
    );

    Ok(())
}

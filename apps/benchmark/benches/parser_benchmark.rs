use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rico::Parser;
use std::fs;
use std::path::Path;
use std::time::Duration;

fn get_test_files() -> Vec<(String, String)> {
    let fixtures_dir = Path::new("benches").join("fixtures");
    if !fixtures_dir.exists() {
        panic!("fixtures directory not found at benches/fixtures/. Please create it and add .thrift files.");
    }

    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(fixtures_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("thrift") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    files.push((name, content));
                }
            }
        }
    }

    if files.is_empty() {
        panic!("No .thrift files found in benches/fixtures/ directory.");
    }

    files
}

fn bench_parser(c: &mut Criterion) {
    let test_files = get_test_files();
    let mut group = c.benchmark_group("Parser");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    for (file_type, content) in test_files {
        group.bench_with_input(
            BenchmarkId::new("parse", file_type.clone()),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(content));
                    black_box(parser.parse().unwrap());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("json_output", file_type.clone()),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(content));
                    let ast = black_box(parser.parse().unwrap());
                    black_box(serde_json::to_string(&ast).unwrap());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("json_pretty_output", file_type.clone()),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(content));
                    let ast = black_box(parser.parse().unwrap());
                    black_box(serde_json::to_string_pretty(&ast).unwrap());
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .with_plots()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));
    targets = bench_parser
);
criterion_main!(benches);

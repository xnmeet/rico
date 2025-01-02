use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rico::Parser;
use std::fs;
use std::time::Duration;

fn create_test_content(base_content: &str, multiplier: usize) -> String {
    let mut result = String::with_capacity(base_content.len() * multiplier);
    for i in 0..multiplier {
        result.push_str(&format!("// File Section {}\n", i + 1));
        result.push_str(base_content);
    }
    result
}

fn parser_benchmark(c: &mut Criterion) {
    let base_content = fs::read_to_string("benches/fixtures/large.thrift").unwrap();
    let multipliers = [1, 10, 30, 50, 100];

    let mut group = c.benchmark_group("parser");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &m in &multipliers {
        let content = create_test_content(&base_content, m);
        let lines = content.lines().count();
        let size_label = format!("{}_lines", lines);

        // Basic parsing benchmark
        group.bench_with_input(
            BenchmarkId::new("parse", &size_label),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(content));
                    black_box(parser.parse().unwrap());
                });
            },
        );

        // JSON output benchmark
        group.bench_with_input(
            BenchmarkId::new("json_output", &size_label),
            &content,
            |b, content| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(content));
                    let ast = black_box(parser.parse().unwrap());
                    black_box(serde_json::to_string(&ast).unwrap());
                });
            },
        );

        // Pretty JSON output benchmark
        group.bench_with_input(
            BenchmarkId::new("json_pretty_output", &size_label),
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
        .sample_size(10)
        .measurement_time(Duration::from_secs(10));
    targets = parser_benchmark
);
criterion_main!(benches);

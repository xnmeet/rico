use insta::assert_snapshot;
use miette::NamedSource;
use rico::{Parser, Writer};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
pub fn test_writer_snapshots() {
    let tests_dir = get_tests_dir();
    let thrift_files = collect_thrift_files(&tests_dir);

    let snapshot_dir = std::env::current_dir().unwrap().join("snapshots/writer");

    fs::create_dir_all(&snapshot_dir).unwrap();

    for path in thrift_files {
        let content = fs::read_to_string(&path).unwrap();
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let mut parser = Parser::new(&content);
        let result = parser
            .parse()
            .map_err(|err| {
                println!(
                    "{:?}",
                    miette::Error::new(err).with_source_code(NamedSource::new(
                        path.display().to_string(),
                        content.clone(),
                    ))
                );
            })
            .unwrap();

        let mut writer = Writer::new();
        let thrift_code = writer.write(&result);

        insta::with_settings!({
            description => format!("Testing {} thrift file", file_name),
            omit_expression => true,
            snapshot_path => snapshot_dir.clone(),
        }, {
            assert_snapshot!(file_name, thrift_code);
        });
    }
}

fn get_tests_dir() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    if current_dir.join("tests").exists() {
        current_dir.join("tests/fixtures")
    } else {
        current_dir.parent().unwrap().join("tests/fixtures")
    }
}

fn collect_thrift_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("thrift") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

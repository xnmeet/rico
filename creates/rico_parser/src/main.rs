use rico_parser::parser::Parser;
use serde_json;
use std::fs;
use std::path::Path;

fn get_tests_dir() -> String {
    // 首先尝试当前目录下的 tests
    if Path::new("tests").exists() {
        return "tests".to_string();
    }
    // 如果不存在，尝试上级目录
    "../tests".to_string()
}

fn main() {
    let files = vec![
        "struct", "service", "enum", "const", "typedef", "common", "union", "header",
    ];
    let tests_dir = get_tests_dir();

    for file_name in files {
        let input_path = format!("{}/{}.thrift", tests_dir, file_name);
        let output_path = format!("{}/{}_expect.json", tests_dir, file_name);

        match fs::read_to_string(&input_path) {
            Ok(input) => {
                let mut parser = Parser::new(&input);
                match parser.parse() {
                    Ok(value) => {
                        // 将结果转换为格式化的 JSON
                        match serde_json::to_string_pretty(&value) {
                            Ok(json_output) => {
                                // 写入结果到输出文件
                                if let Err(e) = fs::write(&output_path, json_output) {
                                    println!("Error writing to {}: {}", output_path, e);
                                } else {
                                    println!(
                                        "Successfully processed {} -> {}",
                                        input_path, output_path
                                    );
                                }
                            }
                            Err(e) => println!("Error converting to JSON for {}: {}", file_name, e),
                        }
                    }
                    Err(e) => println!("Error parsing {}: {}", file_name, e),
                }
            }
            Err(e) => println!("Error reading {}: {}", input_path, e),
        }
    }
}

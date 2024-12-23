use serde_json;
use std::fs;
use thrift_parser::parser::Parser;

fn main() {
    // 预定义要处理的文件名列表
    let files = vec!["struct", "service", "enum", "const", "typedef"];

    for file_name in files {
        // 构造输入文件路径
        let input_path = format!("tests/{}.thrift", file_name);
        // 构造输出文件路径
        let output_path = format!("tests/{}_expect.json", file_name);

        // 读取输入文件
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

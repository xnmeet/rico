use thrift_parser::parser_next::Parser;

fn main() {
    let input = "include \"apps.thrift\" // 带空格 的 注释 \n namespace go lark.apaas.app // 带空格 的 注释";
    let mut temp = Parser::new(input);
    let result = temp.parse();

    match result {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("Error: {}", e),
    }
}

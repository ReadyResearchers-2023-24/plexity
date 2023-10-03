use tree_sitter::{Node, Parser, Tree};
use std::env;
use std::fs;
use std::process;


struct Config {
    filepath: String,
    language: String
}


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("both an input filepath and a programming language must be entered as CLI arguments.");
        }

        let filepath = args[1].clone();
        let language = args[2].clone();

        Ok(Config { filepath, language })
    }
}


fn read_file(filepath: String) -> String {
    println!();
    println!("Selected input file: {}", filepath);

    let file_contents: String = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    return file_contents;
}


fn select_parser(language: String) -> Parser {
    let mut parser: Parser = Parser::new();

    if language == "c" {
        parser.set_language(tree_sitter_c::language()).unwrap();
    } else if language == "java" {
        parser.set_language(tree_sitter_java::language()).unwrap();
    } else if language == "json" {
        parser.set_language(tree_sitter_json::language()).unwrap();
    } else if language == "python" {
        parser.set_language(tree_sitter_python::language()).unwrap();
    } else if language == "rust" {
        parser.set_language(tree_sitter_rust::language()).unwrap();
    }

    return parser;
}


fn build_tree(source_code: String, mut parser: Parser) {
    // Much of the code in this function is adapted from a parse program by Haobo Gu:
    // https://haobogu.github.io/posts/code-intelligence/tree-sitter/

    // Build a syntax tree based on source code stored in a string.
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();  

    // Get the root node of the syntax tree.
    let root_node: Node = parse_tree.root_node();

    // Print the syntax tree as an S-expression.
    let s_expression: String = root_node.to_sexp();
    println!();
    println!("Syntax tree: {}", s_expression);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_contents: String = read_file(config.filepath);
    let parser: Parser = select_parser(config.language);

    build_tree(file_contents, parser);
}
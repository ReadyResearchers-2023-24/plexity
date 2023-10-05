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

    match language.as_str() {
        "c" => parser.set_language(tree_sitter_c::language()).unwrap(),
        "javascript" => parser.set_language(tree_sitter_javascript::language()).unwrap(),
        "json" => parser.set_language(tree_sitter_json::language()).unwrap(),
        "python" => parser.set_language(tree_sitter_python::language()).unwrap(),
        "rust" => parser.set_language(tree_sitter_rust::language()).unwrap(),
        // Need to do something about this "wildcard" match statement (required by compiler)
        &_ => parser.set_language(tree_sitter_rust::language()).unwrap()
    }

    return parser;
}


fn build_tree(source_code: String, mut parser: Parser) -> String {
    // Much of the code in this function is adapted from a parse program by Haobo Gu:
    // https://haobogu.github.io/posts/code-intelligence/tree-sitter/

    // Build a syntax tree based on source code stored in a string.
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();  

    // Get the root node of the syntax tree.
    let root_node: Node = parse_tree.root_node();

    // Convert the syntax tree to an S-expression.
    let s_expression: String = root_node.to_sexp();

    return s_expression;
}


fn pretty_print(s_expression: String) {
    let mut depth: i32 = 0;
    for character in s_expression.chars() {
        if character == '(' && depth == 0 {
            depth += 1;
            print!("{}", character)
        } else if character == '(' {
            depth += 1;
            println!("");
            for _n in 1..depth {
                print!("    ");
            }
            print!("{}", character);
        } else if character == ')' {
            depth -= 1;
            print!("{}", character);
            println!("");
            for _n in 1..depth {
                print!("    ");
            }
        } else {
            print!("{}", character);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_contents: String = read_file(config.filepath);
    let parser: Parser = select_parser(config.language);

    let s_expression: String = build_tree(file_contents, parser);
    pretty_print(s_expression);
}
use tree_sitter::{Node, Parser, Range, Tree};
use std::env;
use std::fs;
use std::process;





struct Config {
    filepath: String,
    language: String,
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

    let file_contents: String =
        fs::read_to_string(filepath).expect("Should have been able to read the file");

    return file_contents;
}

fn select_parser(language: String) -> Parser {
    let mut parser: Parser = Parser::new();

    match language.as_str() {
        "c" => parser.set_language(tree_sitter_c::language()).unwrap(),
        "javascript" => parser
            .set_language(tree_sitter_javascript::language())
            .unwrap(),
        "json" => parser.set_language(tree_sitter_json::language()).unwrap(),
        "python" => parser.set_language(tree_sitter_python::language()).unwrap(),
        "rust" => parser.set_language(tree_sitter_rust::language()).unwrap(),
        // Need to do something about this "wildcard" match statement (required by compiler)
        &_ => parser.set_language(tree_sitter_rust::language()).unwrap(),
    }

    return parser;
}

fn traverse_tree(source_code: String, mut parser: Parser) {
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();
    let root_node: Node = parse_tree.root_node();

    // Unpack nodes recursively, starting with the root node
    let current_depth: i32 = 0;
    let starting_maximum_depth: i32 = 0;
    let concluding_maximum_depth: i32 =
        unpack_node(root_node, current_depth, starting_maximum_depth);
    println!("");
    println!(
        "MAXIMUM DEPTH CALCULATED BY NODE UNPACK IS: {}",
        concluding_maximum_depth
    );
}

fn unpack_node(node: Node, current_depth: i32, mut maximum_depth: i32) -> i32 {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let child_range: Range = child.range();

        if current_depth > maximum_depth {
            maximum_depth = current_depth;
        };

        // Print node ranges (a sanity check to ultimately be removed)
        println!(
            "{}/{}: {} {} {}",
            current_depth,
            maximum_depth,
            child_range.start_point,
            child_range.end_point,
            child.to_sexp()
        );

        maximum_depth = unpack_node(child, current_depth + 1, maximum_depth);
    }

    return maximum_depth;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_contents: String = read_file(config.filepath);
    let parser: Parser = select_parser(config.language);

    traverse_tree(file_contents, parser);
}

use tree_sitter::{Node, Parser, Tree, Range, Point};
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


fn build_tree(source_code: String, mut parser: Parser) {
    // Build a syntax tree based on source code stored in a string.
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();

    // Get the root node of the syntax tree.
    let root_node: Node = parse_tree.root_node();

    // Convert the syntax tree to an S-expression (sanity check; will ultimately remove).
    let s_expression: String = root_node.to_sexp();

    // Pretty print the S-expression (sanity check; will ultimately remove)
    println!("");
    pretty_print(s_expression);
    println!("");

    // Unpack nodes recursively, starting with the root node
    let mut current_depth = 0;
    let mut maximum_depth = 0;
    let maximum_depth: i32 = unpack_node(root_node, current_depth, maximum_depth);
    println!("");
    println!("MAXIMUM DEPTH CALCULATED BY NODE UNPACK IS: {}", maximum_depth);
}


fn unpack_node(node: Node, mut current_depth: i32, mut maximum_depth: i32) -> i32 {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let child_range: Range = child.range();

        // Print node ranges (a sanity check to ultimately be removed)
        println!("{} {} {} {}", current_depth, child_range.start_point, child_range.end_point, child.to_sexp());
        current_depth += 1;

        if current_depth > maximum_depth {
            maximum_depth = current_depth;
        };

        unpack_node(child, current_depth, maximum_depth);
    }

    return maximum_depth;
    
}


fn pretty_print(s_expression: String) {
    let mut max_depth: i32 = 0;
    let mut current_depth: i32 = 0;
    for character in s_expression.chars() {

        if character == '(' && current_depth == 0 {
            current_depth += 1;
        } else if character == '(' {
            current_depth += 1;
            println!("");
            for _n in 1..current_depth {
                print!("----");
            }
        } else if character == ')' {
            current_depth -= 1;
        } else {
            print!("{}", character);
        }

        if max_depth < current_depth {
            max_depth = current_depth;
        }
    }

    println!();
    println!();
    println!("MAXIMUM NESTED DEPTH OF SOURCE PROGRAM (VIA S-EXPRESSION) IS {}", max_depth);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_contents: String = read_file(config.filepath);
    let parser: Parser = select_parser(config.language);

    build_tree(file_contents, parser);
}
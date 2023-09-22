use tree_sitter::{Node, Parser, Tree};
use std::env;
use std::fs;

fn read_input_file() -> String {
    let arguments: Vec<String> = env::args().collect();
    let input_filepath = &arguments[1];

    println!();
    println!("Selected input file: {}", input_filepath);

    let contents = fs::read_to_string(input_filepath)
        .expect("Should have been able to read the file");

    contents
}

fn build_tree(input_file_contents: String) {
    // Much of the code in this function is adapted from a parse program by Haobo Gu:
    // https://haobogu.github.io/posts/code-intelligence/tree-sitter/

    // Create a parser
    let mut parser: Parser = Parser::new();

    // Set the parser's language (JSON in this case)
    parser.set_language(tree_sitter_json::language()).unwrap();

    // Build a syntax tree based on source code stored in a string.
    let source_code = input_file_contents;
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();  

    // Get the root node of the syntax tree.
    let root_node: Node = parse_tree.root_node();

    // Get some child nodes (useful for assertion statements; based on blog post)
    //let array_node: Node = root_node.named_child(0).unwrap();
    //let number_node: Node = array_node.named_child(0).unwrap();

    // Print the syntax tree as an S-expression.
    let s_expression = root_node.to_sexp();
    println!();
    println!("Syntax tree: {}", s_expression);
}

fn main() {
    let input_file_contents = read_input_file();
    build_tree(input_file_contents);
}
// Sample parse program from Haobo Gu, used to demonstrate AST parsing feasibility:
// https://haobogu.github.io/posts/code-intelligence/tree-sitter/

use tree_sitter::{Node, Parser, Tree};
fn main() {
    // Create a parser
    let mut parser: Parser = Parser::new();

    // Set the parser's language (JSON in this case)
    parser.set_language(tree_sitter_json::language()).unwrap();

    // Build a syntax tree based on source code stored in a string.
    let source_code = "[1, null]";
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();

    // Get the root node of the syntax tree.
    let root_node: Node = parse_tree.root_node();

    // Get some child nodes.
    let array_node: Node = root_node.named_child(0).unwrap();
    let number_node: Node = array_node.named_child(0).unwrap();

    // Check that the nodes have the expected types.
    assert_eq!(root_node.kind(), "document");
    assert_eq!(array_node.kind(), "array");
    assert_eq!(number_node.kind(), "number");

    // Check that the nodes have the expected child counts.
    assert_eq!(root_node.child_count(), 1);
    assert_eq!(array_node.child_count(), 5);
    assert_eq!(array_node.named_child_count(), 2);
    assert_eq!(number_node.child_count(), 0);

    // Print the syntax tree as an S-expression.
    let s_expression = root_node.to_sexp();
    println!("Syntax tree: {}", s_expression);
}
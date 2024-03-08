/*  A module for creating an abstract syntax tree from input source code
and evaluating the resultant tree's complexity.  */

use std::env;
use std::fs;
use std::process;
use tree_sitter::{Node, Parser, Range, Tree};

struct Config {
    filepath: String,
    language: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Both an input filepath and a programming language must be entered as CLI arguments.");
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

    println!("Selected programming language grammar: {}", language);

    match language.as_str() {
        "c" => parser.set_language(tree_sitter_c::language()).unwrap(),
        "cpp" => parser.set_language(tree_sitter_cpp::language()).unwrap(),
        "css" => parser.set_language(tree_sitter_css::language()).unwrap(),
        "dockerfile" => parser
            .set_language(tree_sitter_dockerfile::language())
            .unwrap(),
        "gitattributes" => parser.set_language(tree_sitter_gitattributes::language()).unwrap(),
        "html" => parser.set_language(tree_sitter_html::language()).unwrap(),
        "java" => parser.set_language(tree_sitter_java::language()).unwrap(),
        "javascript" => parser
            .set_language(tree_sitter_javascript::language())
            .unwrap(),
        "json" => parser.set_language(tree_sitter_json::language()).unwrap(),
        "markdown" => parser.set_language(tree_sitter_md::language()).unwrap(),
        "php" => parser.set_language(tree_sitter_php::language_php()).unwrap(),
        "python" => parser.set_language(tree_sitter_python::language()).unwrap(),
        "rust" => parser.set_language(tree_sitter_rust::language()).unwrap(),
        "toml" => parser.set_language(tree_sitter_toml::language()).unwrap(),
        "typescript" => parser.set_language(tree_sitter_typescript::language_typescript()).unwrap(),
        // Need to do something about this "wildcard" match statement (required by compiler)
        &_ => parser.set_language(tree_sitter_python::language()).unwrap(),
    }
    return parser;
}

fn traverse_tree(source_code: String, mut parser: Parser) {
    let parse_tree: Tree = parser.parse(source_code, None).unwrap();
    let root_node: Node = parse_tree.root_node();

    // Unpack nodes recursively, starting with the root node
    let mut node_count: i32 = 0;
    let mut plexity_score: i32 = 0;
    let current_depth: i32 = 0;
    let starting_maximum_depth: i32 = 0;
    let concluding_maximum_depth: i32;
    let mut cyclomatic_count: i32 = 0;
    (
        node_count,
        concluding_maximum_depth,
        plexity_score,
        cyclomatic_count,
    ) = unpack_node(
        root_node,
        node_count,
        current_depth,
        starting_maximum_depth,
        plexity_score,
        cyclomatic_count,
    );
    println!("\n\n============ PLEXITY SCORECARD ============\n");
    println!("  - Number of nodes found in tree: {}", node_count);
    println!(
        "  - Maximum depth of syntax tree: {}",
        concluding_maximum_depth
    );
    println!("  - Combined weights of all nodes: {}", plexity_score);
    let plexity_score_float = plexity_score as f32;
    let node_count_float = node_count as f32;
    println!(
        "  - Average depth across syntax tree: {:.2}",
        plexity_score_float / node_count_float
    );
    println!("  - Cyclomatic complexity: {}", cyclomatic_count + 1);
}

fn unpack_node(
    node: Node,
    mut node_count: i32,
    current_depth: i32,
    mut maximum_depth: i32,
    mut plexity_score: i32,
    mut cyclomatic_count: i32,
) -> (i32, i32, i32, i32) {
    for i in 0..node.child_count() {
        node_count += 1;
        let child = node.child(i).unwrap();
        let child_range: Range = child.range();

        if current_depth > maximum_depth {
            maximum_depth = current_depth;
        };

        let is_cyclomatic = cyclomatic_check(child.to_sexp());

        if is_cyclomatic {
            cyclomatic_count += 1;
        }

        // Print node ranges (a sanity check to ultimately be removed)
        println!(
            "#{} | depth:{}/{} | beg:{} end:{} | s-exp: {} | cyclo?: {}",
            node_count,
            current_depth,
            maximum_depth,
            child_range.start_point,
            child_range.end_point,
            child.to_sexp(),
            is_cyclomatic
        );

        plexity_score += current_depth;

        (node_count, maximum_depth, plexity_score, cyclomatic_count) = unpack_node(
            child,
            node_count,
            current_depth + 1,
            maximum_depth,
            plexity_score,
            cyclomatic_count,
        );
    }

    return (node_count, maximum_depth, plexity_score, cyclomatic_count);
}

fn cyclomatic_check(s_expression: String) -> bool {
    return s_expression.starts_with("(if_statement")
        || s_expression.starts_with("(elif_clause")
        || s_expression.starts_with("(for_statement")
        || s_expression.starts_with("(while_statement")
        || s_expression.starts_with("(except_clause")
        || s_expression.starts_with("(with_statement")
        || s_expression.starts_with("(assert_statement")
        || s_expression.starts_with("(list_comprehension")
        || s_expression.starts_with("(set_comprehension")
        || s_expression.starts_with("(dictionary_comprehension")
        || s_expression.starts_with("(boolean_operator");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let full_result = read_file("src/main.rs".to_string());
        let partial_result = &full_result[0..49];
        assert_eq!(
            partial_result,
            "/*  A module for creating an abstract syntax tree"
        );
    }

    #[test]
    fn test_select_parser_c() {
        let input_language = "c".to_string();
        let result_language = &Some(tree_sitter_c::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_dockerfile() {
        let input_language = "dockerfile".to_string();
        let result_language = &Some(tree_sitter_dockerfile::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_java() {
        let input_language = "java".to_string();
        let result_language = &Some(tree_sitter_java::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_javascript() {
        let input_language = "javascript".to_string();
        let result_language = &Some(tree_sitter_javascript::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_json() {
        let input_language = "json".to_string();
        let result_language = &Some(tree_sitter_json::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_markdown() {
        let input_language = "markdown".to_string();
        let result_language = &Some(tree_sitter_md::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_python() {
        let input_language = "python".to_string();
        let result_language = &Some(tree_sitter_python::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_rust() {
        let input_language = "rust".to_string();
        let result_language = &Some(tree_sitter_rust::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }

    #[test]
    fn test_select_parser_toml() {
        let input_language = "toml".to_string();
        let result_language = &Some(tree_sitter_toml::language());
        assert!(select_parser(input_language).language().eq(result_language))
    }
}

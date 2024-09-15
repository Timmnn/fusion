use crate::lang_parser::Rule;
use pest::iterators::Pair;

pub fn print_parse_tree(pair: Pair<Rule>, depth: usize) {
    // Indentation for the current level of the tree
    let indent = "    ".repeat(depth);

    // Print the rule and the corresponding matched string (the span)
    println!(
        "{}Rule: {:?}, Text: {:?}",
        indent,
        pair.as_rule(),
        pair.as_str()
    );

    // Recursively process inner pairs (children of the current node)
    for inner_pair in pair.into_inner() {
        print_parse_tree(inner_pair, depth + 1);
    }
}

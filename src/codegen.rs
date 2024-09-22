use std::fmt::format;

use crate::lang_parser::{JPPParser, Rule};
use pest::iterators::Pair;

pub fn generate_c_code(tree: Pair<Rule>) -> String {
    println!("Generating C Code...");

    // Initialize a mutable String to hold the generated program code
    let program_code;

    // Match the root rule to ensure we are dealing with a program
    match tree.as_rule() {
        Rule::program => {
            program_code = walk_program(tree);
        }
        _ => panic!("Invalid Top-Level Node: {:?}", tree.as_rule()),
    }

    // Return the generated C code, wrapping it in an `int main()` function
    return format!(
        "#include <stdio.h>\nint main() {{\n{}\nreturn 0;\n}}",
        program_code
    );
}

fn walk_program(program: Pair<Rule>) -> String {
    let mut c_code = String::new();

    for child in program.into_inner() {
        match child.as_rule() {
            Rule::statement => {
                let statement_code = walk_statement(child);
                c_code.push_str(&statement_code);
                c_code.push('\n');
            }
            Rule::EOI => continue,
            _ => panic!("Invalid Node in program: {:?}", child.as_rule()),
        }
    }

    c_code
}

fn walk_statement(statement: Pair<Rule>) -> String {
    for child in statement.into_inner() {
        match child.as_rule() {
            Rule::assignment => {
                return walk_assignment(child); // Handle assignment and return C code
            }
            Rule::declaration => {
                return walk_declaration(child); // Handle declaration and return C code
            }
            Rule::expression => {
                return walk_expression(child); // Handle expression and return C code
            }
            Rule::function_call => {
                return walk_function_call(child); // Handle function call and return C code
            }
            Rule::if_statement => {
                return walk_if_statement(child);
            }
            Rule::while_loop => {
                return walk_while_loop(child);
            }
            _ => panic!("Invalid Node in statement: {:?}", child.as_rule()),
        }
    }

    String::new() // Return empty if no valid statement found
}

fn walk_if_statement(if_statement: Pair<Rule>) -> String {
    let code: String;

    let mut inner_pairs = if_statement.into_inner();

    let binary_expression = inner_pairs.next().unwrap();

    let block = inner_pairs.next().unwrap();

    return format!(
        "if({}){{{}}}",
        walk_boolean_expression(binary_expression),
        walk_block(block)
    );
}

fn walk_while_loop(while_loop: Pair<Rule>) -> String {
    let code: String;

    let mut inner_pairs = while_loop.into_inner();

    let binary_expression = inner_pairs.next().unwrap();

    let block = inner_pairs.next().unwrap();

    return format!(
        "while({}){{{}}}",
        walk_boolean_expression(binary_expression),
        walk_block(block)
    );
}
fn walk_boolean_expression(boolean_expression: Pair<Rule>) -> String {
    let mut inner_pairs = boolean_expression.into_inner();

    let left_side = inner_pairs.next().unwrap();
    let comparison = inner_pairs.next().unwrap();
    let right_side = inner_pairs.next().unwrap();

    return format!(
        "{} {} {}",
        walk_expression(left_side),
        walk_number_comparison(comparison),
        walk_expression(right_side),
    );
}

fn walk_block(block: Pair<Rule>) -> String {
    let mut code = String::new(); // Initialize `code` to an empty String

    let inner_pairs = block.into_inner(); // mutable iterator for inner pairs

    // Use a for loop to iterate over each statement
    for statement in inner_pairs {
        code += &walk_statement(statement)
    }

    return code;
}

fn walk_number_comparison(comparison: Pair<Rule>) -> String {
    return comparison.as_str().to_string();
}

fn walk_assignment(assignment: Pair<Rule>) -> String {
    let mut inner_pairs = assignment.into_inner();

    let identifier_pair = inner_pairs.next().unwrap();
    assert_eq!(identifier_pair.as_rule(), Rule::identifier);

    let variable_name = identifier_pair.as_str();
    println!("Variable name: {}", variable_name);

    // The second inner pair should be the value (numeric or otherwise)
    let value_pair = inner_pairs.next().unwrap();
    assert_eq!(value_pair.as_rule(), Rule::expression);

    let value = value_pair.as_str();
    println!("Assigned value: {}", value);

    // Generate the C code for assignment
    return format!("{} = {};", variable_name, value);
}

fn walk_declaration(declaration: Pair<Rule>) -> String {
    let mut inner_pairs = declaration.into_inner();

    // Extract the variable type
    let type_pair = inner_pairs.next().unwrap();
    assert_eq!(type_pair.as_rule(), Rule::r#type);

    let var_type = type_pair.as_str();
    println!("Variable type: {}", var_type);

    // Extract the variable name (identifier)
    let identifier_pair = inner_pairs.next().unwrap();
    assert_eq!(identifier_pair.as_rule(), Rule::identifier);

    let variable_name = identifier_pair.as_str();
    println!("Variable name: {}", variable_name);

    // Generate the C code for declaration (type + assignment)
    return format!("{} {};", var_type, variable_name);
}

fn walk_expression(expression: Pair<Rule>) -> String {
    // Get the inner pairs of the expression
    let mut inner_pairs = expression.clone().into_inner();

    // Get the first child, if any
    let first_child = inner_pairs.next();

    match first_child {
        None => {
            // Handle the case where no child is present
            println!("No child found, remaining text: {}", expression.as_str());
            return expression.as_str().to_string();
        }
        Some(child) => match child.as_rule() {
            Rule::number => format!("{}", child.as_str()), // For a number, return the number as a string
            Rule::identifier => format!("{}", child.as_str()), // For an identifier, return the identifier as a string
            Rule::function_call => walk_function_call(child), // Delegate to `walk_function_call` for function calls
            _ => panic!("Unexpected rule in expression: {:?}", child.as_rule()), // Handle unexpected cases
        },
    }
}

fn walk_function_call(function_call: Pair<Rule>) -> String {
    let mut inner_pairs = function_call.into_inner();

    // The first pair is the function identifier
    let function_name = inner_pairs.next().unwrap().as_str();
    println!("Function name: {}", function_name);

    // The second part is the parameter list, we need to process that
    let params_list = inner_pairs.next().unwrap();
    let params = walk_params_list(params_list);

    // Generate the C code for the function call
    return format!("{}({});", function_name, params);
}

fn walk_params_list(params_list: Pair<Rule>) -> String {
    let mut params = Vec::new();

    // Iterate over each parameter in the parameter list
    for param in params_list.into_inner() {
        params.push(walk_expression(param));
    }

    // Join the parameters with commas, as they appear in C function calls
    return params.join(", ");
}

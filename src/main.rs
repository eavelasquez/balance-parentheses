// Rust Program to find minimum number of '(' or ')'
// must be added to make parentheses string valid.
// For example, "((()))" is valid but "(()))" is not valid.
// Input: "())"
// Output: 1

// This function is used to return required minimum
// number of '(' or ')' to make string valid.
fn min_parentheses(input: &mut String) -> i8 {
    let mut balance: i8 = 0; // Maintain balance of parentheses.
    let mut minimum: i8 = 0; // To store minimum number of '(' or ')'.

    // Traverse the input string.
    for c in input.chars() {
        match c {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => println!("Unknown character"), // This should not happen.
        }

        // If balance becomes negative then set balance to 0 and increment minimum.
        if balance < 0 {
            balance = 0;
            minimum += 1;
        }
    }

    minimum += balance;
    minimum
}

// Driver code to test above function and print result.
fn main() {
    let mut input: String = String::from("((()()())()()()())))((((()))))))((((())");
    println!("Input: {}", input);
    println!("Minimum number of '(' or ')' to make string valid: {}", min_parentheses(&mut input));
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars().filter(is_bracket) {
        if is_closing(c) {
            if is_paired(&stack[..], c) {
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}

fn is_bracket(c: &char) -> bool {
    *c == '(' || *c == ')' || *c == '{' || *c == '}' || *c == '[' || *c == ']'
}

fn is_closing(c: char) -> bool {
    c == ')' || c == ']' || c == '}'
}

fn is_paired(stack: &[char], c: char) -> bool {
    if stack.is_empty() {
        return false;
    }
    match c {
        ')' => stack[stack.len() - 1] == '(',
        ']' => stack[stack.len() - 1] == '[',
        '}' => stack[stack.len() - 1] == '{',
        _ => false,
    }
}

fn some(i: i32) {
    println!("Some function");
}

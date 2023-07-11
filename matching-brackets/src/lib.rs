static OPENING: &[char] = &['(', '[', '{'];
static CLOSING: &[char] = &[')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        if OPENING.contains(&c) {
            stack.push(c);
        } else if CLOSING.contains(&c) {
            match (c, stack.pop()) {
                (')', Some('(')) => continue,
                (']', Some('[')) => continue,
                ('}', Some('{')) => continue,
                _ => return false,
            }
        }
    }
    stack.is_empty()
}

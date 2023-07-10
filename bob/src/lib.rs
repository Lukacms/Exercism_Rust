pub fn reply(message: &str) -> &str {
    let is_empty = message.trim() == "";
    let has_letter = message.chars().any(char::is_alphabetic);
    let is_all_capital = has_letter && message.to_uppercase().as_str() == message;
    let is_question = message.trim().ends_with('?');

    match (is_empty, is_all_capital, is_question) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, false) => "Whoa, chill out!",
        (_, false, true) => "Sure.",
        _ => "Whatever.",
    }
}

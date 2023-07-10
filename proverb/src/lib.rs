pub fn build_proverb(list: &[&str]) -> String {
    let mut res: String = String::new();

    for i in 1..list.len() {
        res.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i - 1],
            list[i]
        ));
    }
    if !list.is_empty() {
        res.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    res
}

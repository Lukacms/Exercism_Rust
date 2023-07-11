pub fn raindrops(n: u32) -> String {
    let mut rain: String = String::new();

    if n % 3 == 0 {
        rain.push_str("Pling");
    }
    if n % 5 == 0 {
        rain.push_str("Plang");
    }
    if n % 7 == 0 {
        rain.push_str("Plong");
    }
    if rain.is_empty() {
        n.to_string()
    } else {
        rain
    }
}

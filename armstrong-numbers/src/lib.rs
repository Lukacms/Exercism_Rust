pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .as_str()
        .bytes()
        .map(|b| (b - b'0') as u32)
        .collect();
    let len = num.to_string().len();
    let mut increment_num: u32 = 0;

    for digit in digits {
        if increment_num.checked_add(digit.pow(len as u32)).is_none() {
            return false;
        }
        increment_num += digit.pow(len as u32);
    }
    increment_num == num
}

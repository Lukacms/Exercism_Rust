//
// EPITECH PROJECT, 2022
// exercices_rust
// File description:
// lib
//

fn production_by_hour(speed_range: u8) -> f64 {
    let default_hour: f64 = 221.0;
    let result;

    if speed_range <= 4 {
        result = speed_range as f64 * default_hour * 1.0;
    } else if speed_range <= 8 {
        result = speed_range as f64 * default_hour * 0.9;
    } else {
        result = speed_range as f64 * default_hour * 0.77;
    }
    return result;
}

fn working_items_per_minute(speed: u8) -> u32 {
    let hour: f64 = production_by_hour(speed);
    let result: u32 = hour as u32 / 60;

    return result;
}

fn main() {
    println!("{}", working_items_per_minute(6));
}

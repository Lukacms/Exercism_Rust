//
// EPITECH PROJECT, 2022
// exercices_rust
// File description:
// lib
//

fn expected_minutes_in_oven() -> i32 {
    return 40;
}

fn remaining_minutes_in_oven(time_passed: i32) -> i32 {
    let expected = expected_minutes_in_oven();

    return expected - time_passed;
}

fn preparation_time_in_minutes(nb_of_layers: i32) -> i32 {
    let time_by_layer = 2;

    return nb_of_layers * time_by_layer;
}
fn elapsed_time_in_minutes(nb_of_layers: i32, time_in_over: i32) -> i32 {
    return preparation_time_in_minutes(nb_of_layers) + time_in_over
}

fn main() {
    println!("Time passed: {}", elapsed_time_in_minutes(5, 42))
}

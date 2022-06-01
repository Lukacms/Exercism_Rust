//
// EPITECH PROJECT, 2022
// exercices_rust
// File description:
// lib
//

use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    let value: usize = _color as usize;

    return value;
}

fn from_resistor_to_str(value: ResistorColor) -> String {
    match value {
        ResistorColor::Black => "Black".to_string(),
        ResistorColor::Blue => "Blue".to_string(),
        ResistorColor::Brown => "Brown".to_string(),
        ResistorColor::Green => "Green".to_string(),
        ResistorColor::Grey => "Grey".to_string(),
        ResistorColor::Orange => "Orange".to_string(),
        ResistorColor::Red => "Red".to_string(),
        ResistorColor::Violet => "Violet".to_string(),
        ResistorColor::White => "White".to_string(),
        ResistorColor::Yellow => "Yellow".to_string(),
        _ => panic!("Unknow value."),
    }
}

fn from_usize_to_resistor(value: usize) -> ResistorColor {
    match value {
        0 => ResistorColor::Black,
        6 => ResistorColor::Blue,
        1 => ResistorColor::Brown,
        5 => ResistorColor::Green,
        8 => ResistorColor::Grey,
        3 => ResistorColor::Orange,
        2 => ResistorColor::Red,
        7 => ResistorColor::Violet,
        9 => ResistorColor::White,
        4 => ResistorColor::Yellow,
        _ => panic!("Unknow value {}", value),
    }
}

pub fn value_to_color_string(value: usize) -> String {
    let resistor: ResistorColor = from_usize_to_resistor(value);
    let color = from_resistor_to_str(resistor);

    return color;
}

pub fn colors() -> Vec<ResistorColor> {
    let mut order: Vec<ResistorColor> = Vec::new();

    for i in 0..10 {
        order.push(from_usize_to_resistor(i));
    }
    return order;
}

pub fn main() {
    let order: Vec<ResistorColor> = colors();

    for i in 0..10 {
        println!("Color: {}", from_resistor_to_str(order[i]));
    }
}

use std::cmp::Ordering;

static STUDENTS: &[&str] = &[
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

fn plant_type(ptype: char) -> &'static str {
    match ptype {
        'V' => "violets",
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        _ => "",
    }
}

pub fn plants(_diagram: &'static str, _student: &str) -> Vec<&'static str> {
    let mut res: Vec<&'static str> = vec![];
    let split_diagram: Vec<&str> = _diagram.lines().collect();

    (0..STUDENTS.len()).for_each(|i| {
        if STUDENTS[i].cmp(_student) == Ordering::Equal {
            res.push(plant_type(split_diagram[0].chars().nth(i * 2).unwrap()));
            res.push(plant_type(split_diagram[0].chars().nth(i * 2 + 1).unwrap()));
            res.push(plant_type(split_diagram[1].chars().nth(i * 2).unwrap()));
            res.push(plant_type(split_diagram[1].chars().nth(i * 2 + 1).unwrap()));
        }
    });
    res
}

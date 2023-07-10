#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (_, 0) => Comparison::Superlist,
        (0, _) => Comparison::Sublist,
        (m, n) if m > n => {
            if first_list.windows(n).any(|v| v == second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m < n => {
            if second_list.windows(m).any(|v| v == first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

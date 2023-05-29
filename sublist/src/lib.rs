#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (_, 0) => Comparison::Superlist,
        (0, _) => Comparison::Sublist,
        (m, n) if m > n => {
            if _first_list.windows(n).any(|w| w == _second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if n > m => {
            if _second_list.windows(m).any(|w| w == _first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

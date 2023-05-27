use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Equal;
    }
    if _first_list.is_empty() && !_second_list.is_empty() {
        return Comparison::Sublist;
    }
    if !_first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Superlist;
    }
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let first: &[T];
    let second: &[T];
    let mut reversed = false;

    if _first_list.len() >= _second_list.len() {
        first = _first_list;
        second = _second_list;
    } else {
        first = _second_list;
        second = _first_list;
        reversed = true;
    }

    let mut fp = 0;
    let mut fp2 = 0;
    let mut sp = 0;
    let mut fe;
    let mut se;

    while fp < first.len() {
        fe = first.get(fp2).unwrap();
        se = second.get(sp).unwrap();

        while fe == se && sp < second.len() {
            if sp == second.len() - 1 {
                if reversed {
                    return Comparison::Sublist;
                } else {
                    return Comparison::Superlist;
                }
            }

            fp2 = fp2 + 1;
            sp = sp + 1;

            if fp2 == first.len() || sp == second.len() {
                break;
            }

            fe = first.get(fp2).unwrap();
            se = second.get(sp).unwrap();
        }

        sp = 0;
        fp = fp + 1;
        fp2 = fp;
    }

    Comparison::Unequal
}

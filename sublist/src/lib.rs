#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.len() > _second_list.len() && is_sublist(_first_list, _second_list) {
        return Comparison::Superlist;
    } else if _second_list.len() > _first_list.len() && is_sublist(_second_list, _first_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

pub fn is_sublist<T: PartialEq>(longer: &[T], shorter: &[T]) -> bool {
    if shorter.is_empty() || longer.is_empty() {
        return true;
    }
    for i in 0..=longer.len() - shorter.len() as usize {
        if longer[i] == shorter[0] && longer[i..i + shorter.len()] == *shorter {
            return true;
        }
    }
    false
}

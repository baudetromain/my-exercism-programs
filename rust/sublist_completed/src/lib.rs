use core::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// solution proposed by someone else
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison
{
    match (_first_list.len(), _second_list.len())
    {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m < n => if _second_list.windows(_first_list.len()).any(|elem| elem.eq(_first_list)) { Comparison::Sublist } else { Comparison::Unequal },
        (m, n) if m > n => if _first_list.windows(_second_list.len()).any(|elem| elem.eq(_second_list)) { Comparison::Superlist } else { Comparison::Unequal },
        (_, _) => if _first_list.eq(_second_list) { Comparison::Equal } else { Comparison::Unequal }
    }
}

pub fn old_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison
{
    match _first_list.len() == _second_list.len()
    {
        true =>
        {
            match _first_list.eq(_second_list)
            {
                true => Comparison::Equal,
                false => Comparison::Unequal
            }
        },
        false =>
        {
            match _first_list.len().cmp(&_second_list.len())
            {
                Ordering::Less =>
                {
                    let mut index: usize = 0;
                    let mut index_second_list: usize = 0;
                    let mut match_start: isize = -1;

                    while index_second_list < _second_list.len()
                    {
                        let element_second_list: &T = _second_list.get(index_second_list).unwrap();
                        match _first_list.get(index)
                        {
                            Some(_element) =>
                            {
                                match _element.eq(element_second_list)
                                {
                                    true =>
                                    {
                                        index += 1;
                                        match match_start
                                        {
                                            -1 => match_start = index_second_list as isize,
                                            _ => {}
                                        }
                                    }
                                    false =>
                                    {
                                        index = 0;
                                        match match_start
                                        {
                                            -1 => {}
                                            _index =>
                                            {
                                                index_second_list = _index as usize;
                                                match_start = -1;
                                            }
                                        }
                                    }
                                }
                            },
                            None => return Comparison::Sublist
                        }

                        index_second_list += 1;
                    }

                    match _first_list.len().eq(&index)
                    {
                        true => Comparison::Sublist,
                        false => Comparison::Unequal
                    }
                },
                Ordering::Greater =>
                {
                    let mut index: usize = 0;
                    let mut index_first_list: usize = 0;
                    let mut match_start: isize = -1;

                    while index_first_list < _first_list.len()
                    {
                        let element_first_list: &T = _first_list.get(index_first_list).unwrap();
                        match _second_list.get(index)
                        {
                            Some(_element) =>
                                {
                                    match _element.eq(element_first_list)
                                    {
                                        true =>
                                            {
                                                index += 1;
                                                match match_start
                                                {
                                                    -1 => match_start = index_first_list as isize,
                                                    _ => {}
                                                }
                                            }
                                        false =>
                                            {
                                                index = 0;
                                                match match_start
                                                {
                                                    -1 => {}
                                                    _index =>
                                                        {
                                                            index_first_list = _index as usize;
                                                            match_start = -1;
                                                        }
                                                }
                                            }
                                    }
                                },
                            None => return Comparison::Superlist
                        }

                        index_first_list += 1;
                    }

                    match _second_list.len().eq(&index)
                    {
                        true => Comparison::Superlist,
                        false => Comparison::Unequal
                    }
                },
                _ => unreachable!()
            }
        }
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool
{
    match code.to_owned()
        .as_bytes()
        .iter()
        .filter(|_char| _char.is_ascii_digit())
        .count()
    {
        1 => false,
        len => match code.to_owned()
            .as_bytes()
            .iter()
            .filter(|_char| !_char.is_ascii_digit() && _char != &&b' ')
            .count()
        {
            0 =>
            {
                match code.to_owned()
                    .as_bytes()
                    .iter()
                    .filter(|_char| _char.is_ascii_digit())
                    .map(|elem| elem - 48)
                    .enumerate()
                    .map(|(i, elem)|
                        {
                            match (len - i) % 2
                            {
                                0 =>
                                    {
                                        match elem * 2
                                        {
                                            elem_times_2 if elem_times_2 > 9 => elem_times_2 - 9,
                                            elem_times_2 => elem_times_2
                                        }
                                    },
                                _ => elem
                            }
                        })
                    .reduce(|a, b| a + b)
                    .unwrap()
                    % 10
                {
                    0 => true,
                    _ => false
                }
            },
            _ => false
        }
    }
}

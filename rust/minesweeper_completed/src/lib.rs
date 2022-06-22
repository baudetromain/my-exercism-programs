use std::char::from_digit;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<String> = Vec::new();

    for i in 0..minefield.len()
    {
        let mut line: String = String::new();

        for j in 0..minefield.get(i).unwrap().len()
        {
            match minefield.get(i).unwrap().as_bytes()[j]
            {
                b'*' => {line.push('*');}
                b' ' =>
                {
                    let mut count: u8 = 0;

                    for di in [-1, 0, 1] as [isize; 3]
                    {
                        for dj in [-1, 0, 1] as [isize; 3]
                        {
                            match (di, dj)
                            {
                                (0, 0) => {},
                                (di, dj) if i as isize + di >= 0
                                    && i as isize + di < minefield.len() as isize
                                    && j as isize + dj >= 0
                                    && j as isize + dj < minefield.get(i).unwrap().len() as isize =>
                                {
                                    match minefield.get((i as isize + di) as usize)
                                        .unwrap()
                                        .as_bytes()[(j as isize + dj) as usize]
                                    {
                                        b'.' => {},
                                        b'*' => {count += 1;},
                                        _ => {}
                                    }
                                },
                                (_, _) => {}
                            }
                        }
                    }

                    line.push(match count
                    {
                        0 => ' ',
                        count => from_digit(count as u32, 10).unwrap()
                    });
                },
                _ => {}
            }
        }

        board.push(line);
    }

    board
}

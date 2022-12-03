use super::*;

pub struct Win {
    pub table: Trans,
    pub marked_nums: Vec<usize>,
    pub last_idx: usize,
    pub last_num: usize,
}

impl Win {
    pub fn get_score(&self) -> usize {
        let marked_sum: usize = self.marked_nums.iter().sum();
        let board_sum: usize = self.table.get_table().get_vec().iter().sum();
        let score = (board_sum - marked_sum) * self.last_num;

        score
    }
}

pub fn get_winner_brd(tables: Vec<Trans>) -> Option<Win> {
    if tables.is_empty() {
        panic!("No header in tables.");
    }

    let mut iter = tables.into_iter();
    let header = iter.next().unwrap();
    let mut wins: Vec<Win> = Vec::new();
    let mut tbl_idx = 0;

    while let Some(curr_tbl) = iter.next() {
        tbl_idx += 1;
        let def = curr_tbl.get_def();
        if def.row_count != def.col_count {
            panic!(
                "Table on index {} has different row and col length.",
                tbl_idx
            );
        }

        let identity = get_win_idx(&header, &curr_tbl);
        let curr_tbl = curr_tbl.invert();
        let inverse = get_win_idx(&header, &curr_tbl);

        let last_idx = match (identity, inverse) {
            (None, None) => None,
            (iden, inv) => {
               if iden.unwrap_or(0) > inv.unwrap_or(0) {
                   Some(inv.unwrap_or(0))
               } else {
                   Some(iden.unwrap_or(0))
               }
            }
        };

        if let Some(last_idx) = last_idx {
            let win_head_nums = header.get_table().get_vec()[0..last_idx + 1].to_vec();
            let marked_nums: Vec<usize> = curr_tbl.to_vec()
                .into_iter()
                .filter(|m| win_head_nums.contains(m))
                .collect();

            wins.push(Win {
                table: curr_tbl,
                marked_nums: marked_nums,
                last_idx: last_idx,
                last_num: header.get_table().get_vec()[last_idx],
            });
        }
    }

    if wins.is_empty() {
        return None;
    }

    let win = wins.into_iter().fold(
        Win {
            table: Trans::Identity(Table::new(None)),
            marked_nums: vec![],
            last_idx: usize::max_value(),
            last_num: 0usize,
        },
        |acc, curr| match curr.last_idx < acc.last_idx {
            true => curr,
            false => acc,
        },
    );

    Some(win)
}

// oh this is awful, refactor it
fn get_win_idx(header: &Trans, curr_tbl: &Trans) -> Option<usize> {
    let curr_def = curr_tbl.get_def();

    for row in 0..curr_def.row_count {
        let mut row_found = true;
        let mut last_idx = 0usize;
        for col in 0..curr_def.col_count {
            let mut col_found = false;
            for (head_idx, curr) in header.get_table().get_vec().iter().enumerate() {
                let num = *curr;
                if num == curr_tbl.get_num(row, col).unwrap() {
                    col_found = true;
                    last_idx = if head_idx > last_idx {
                        head_idx
                    } else {
                        last_idx
                    };
                    break;
                }
            }

            if !col_found {
                row_found = false;
                break;
            }
        }

        if row_found {
            return Some(last_idx);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c() {
        let input = build_input();
        let tables = parse_tables(input);

        let tables = tables.into_iter().map(Trans::Identity).collect();
        let winner = get_winner_brd(tables).unwrap();

        assert_eq!(14usize, winner.table.to_vec()[0]);
        assert_eq!(11, winner.last_idx);
        assert_eq!(4512, winner.get_score());
    }

    fn build_input() -> &'static str {
        r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 "#
    }
}

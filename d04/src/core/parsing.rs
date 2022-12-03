use std::{str::Chars, iter::Peekable};

#[derive(Copy, Clone)]
pub struct TableDef {
    pub row_count: usize,
    pub col_count: usize,
}

pub struct Table {
    pub def: Option<TableDef>,
    grid: Vec<usize>,
}

impl Table {
    pub fn new(def: Option<TableDef>) -> Table {
        match def {
            Some(def) => Table {
                grid: vec![],
                def: Some(def),
            },
            None => Table {
                grid: vec![],
                def: None,
            },
        }
    }

    pub fn push_num(&mut self, num: usize) {
        self.grid.push(num);
    }

    pub fn get_vec(&self) -> &Vec<usize> {
        &self.grid
    }
}

pub fn parse_tables(input: &str) -> Vec<Table> {
    let mut iter = input.chars().peekable();
    let mut tables: Vec<Table> = vec![];

    let header = parse_header(&mut iter);
    tables.push(header);

    while let Some(table) = parse_table_internal(
        Some(TableDef {
            row_count: 5,
            col_count: 5,
        }),
        &mut iter,
    ) {
        tables.push(table);
    }

    tables
}

pub fn parse_header(iter: &mut Peekable<Chars>) -> Table {
    parse_table_internal(Option::None, iter).unwrap()
}

#[cfg(test)]
pub fn parse_table(def: Option<TableDef>, input: &str) -> Option<Table> {
    let mut iter = input.chars().peekable();
    parse_table_internal(def, &mut iter)
}

fn parse_table_internal(def: Option<TableDef>, iter: &mut Peekable<Chars>) -> Option<Table> {
    let mut num: Option<usize> = None;
    let mut table = Table::new(def);

    if iter.peek() == None {
        return None;
    }

    loop {
        let current = iter.next();

        match current {
            Some('\n') => {
                table.push_num(num.take().unwrap());

                if iter.peek() == Some(&'\n') {
                    iter.next();
                    break;
                }
            }
            Some(',') | Some(' ') => {
                if iter.peek() == Some(&' ') {
                    iter.next();
                }

                if num.is_none() {
                    continue;
                }

                table.push_num(num.take().unwrap());
            }
            Some(current) => match current.to_digit(10) {
                Some(digit) => num = Some(num.take().unwrap_or(0) * 10 + digit as usize),
                None => panic!("Unhandled char found: '{}'", current),
            },
            None => {
                break;
            }
        };
    }

    Some(table)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_head_line_nn() {
        let input = "1 11 22\n\n";
        let tables: Vec<Table> = parse_tables(input);

        assert_eq!(1, tables.len());
        assert_seqeq(&vec![1, 11, 22], tables[0].get_vec());
    }

    #[test]
    fn parse_head_and_table() {
        let mut input = String::new();
        input.push_str("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n");
        input.push('\n');
        input.push_str("22 13 17 11  0\n");
        input.push_str(" 8  2 23  4 24\n");
        input.push_str("21  9 14 16  7\n");
        input.push_str(" 6 10  3 18  5\n");
        input.push_str(" 1 12 20 15 19\n");
        input.push('\n');

        let tables: Vec<Table> = parse_tables(&input);

        assert_eq!(2, tables.len());
        assert_seqeq(
            &vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            tables[0].get_vec(),
        );

        let expected = &vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];

        assert_seqeq(expected, tables[1].get_vec());
    }

    #[test]
    fn parse_table_no_head() {
        let mut input = String::new();
        input.push_str("1 1 1 1 1\n");
        input.push_str("2 2 2 2 2\n");
        input.push_str("3 3 3 3 3\n");
        input.push_str("4 4 4 4 4\n");
        input.push_str("5 5 5 5 5\n");
        input.push('\n');

        let table = parse_table(
            Some(TableDef {
                col_count: 5,
                row_count: 5,
            }),
            &input,
        );

        let expected = &vec![
            1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5,
        ];

        assert_seqeq(expected, table.unwrap().get_vec());
    }

    // todo make this to a public helper method or macro
    fn assert_seqeq(left: &Vec<u32>, right: &Vec<usize>) {
        let seq_eq = left.iter().zip(right).all(|(a, b)| {
            assert_eq!(*a as usize, *b);
            true
        });

        assert!(seq_eq);
        assert_eq!(left.len(), right.len())
    }
}

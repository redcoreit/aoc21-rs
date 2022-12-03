use super::*;

pub enum Trans {
    Identity(Table),
    Inverse(Table),
}

impl Trans {
    pub fn get_num(&self, row_idx: usize, col_idx: usize) -> Option<usize> {
        let def = self.get_def();

        let idx = match self {
            Trans::Identity(_) => def.row_count * row_idx + col_idx,
            Trans::Inverse(_) => def.col_count * col_idx + row_idx,
        };

        let grid = self.get_table().get_vec();

        if idx < grid.len() {
            Some(grid[idx])
        } else {
            None
        }
    }

    pub fn get_table(&self) -> &Table {
        match self {
            Trans::Identity(tbl) => tbl,
            Trans::Inverse(tbl) => tbl,
        }
    }

    pub fn get_def(&self) -> TableDef {
        let tbl = self.get_table();
        match tbl.def {
            Some(def) => def.to_owned(),
            None => TableDef {
                row_count: 1,
                col_count: tbl.get_vec().len(),
            },
        }
    }

    pub fn invert(self) -> Trans {
        match self {
            Trans::Identity(tbl) => Trans::Inverse(tbl),
            Trans::Inverse(tbl) => Trans::Identity(tbl),
        }
    }

    pub fn to_vec(&self) -> Vec<usize> {
        let def = self.get_def();
        let mut result: Vec<usize> = vec![0usize; def.row_count * def.col_count];
        let mut idx = 0;
        for row_idx in 0..def.row_count {
            for col_idx in 0..def.col_count {
                let num = self.get_num(row_idx, col_idx).unwrap();
                result[idx] = num;
                idx += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trans_table_inverse() {
        let input = build_table();
        let table = parse_table(Some(TableDef { row_count: 5, col_count: 5 }), &input);

        assert!(table.is_some());

        let trans = Trans::Inverse(table.unwrap());
        let expected : &Vec<usize> = &vec![
            1,2,3,4,5,
            1,2,3,4,5,
            1,2,3,4,5,
            1,2,3,4,5,
            1,2,3,4,5,
        ];

        assert_seqeq(expected, &trans.to_vec());
    }

    fn build_table() -> String {
        let mut input = String::new();
        input.push_str("1 1 1 1 1\n");
        input.push_str("2 2 2 2 2\n");
        input.push_str("3 3 3 3 3\n");
        input.push_str("4 4 4 4 4\n");
        input.push_str("5 5 5 5 5\n");
        input.push('\n');

        input
    }

    // todo make this to a public helper method or macro
    fn assert_seqeq(left: &Vec<usize>, right: &Vec<usize>) {
        let seq_eq = left.iter().zip(right).all(|(a, b)| {
            assert_eq!(a, b);
            true
        });

        assert!(seq_eq);
        assert_eq!(left.len(), right.len())
    }
}


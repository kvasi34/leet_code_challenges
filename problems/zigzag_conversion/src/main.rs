pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let mut rows: Vec<Vec<char>> = vec![vec![]; num_rows as usize];
    let mut going_down = true;
    let mut row = 0usize;

    for c in s.chars() {
        rows[row].push(c);
        if row == num_rows as usize - 1 {
            going_down = false;
        } else if row == 0 {
            going_down = true;
        }

        if going_down {
            row += 1;
        } else {
            row -= 1;
        }
    }

    rows.iter().flat_map(|r| r.iter()).collect()
}

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn three_rows() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn four_rows() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn single_char() {
        assert_eq!(convert("A".to_string(), 1), "A");
    }

    #[test]
    fn single_row() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 1), "PAYPALISHIRING");
    }
}

fn main() {}

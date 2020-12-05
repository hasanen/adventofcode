use core::cmp::Ordering;
use std::fmt;

pub struct Seat {
    row: u8,
    column: u8,
    pub id: u32
}
impl Seat {
    pub fn from_str(line: &str) -> Seat {
        let (row, column) = line.trim().split_at(7);
        let row_number: u8 = Seat::resolve(row, 0, 127, 'F', 'B');
        let column_number: u8 = Seat::resolve(column, 0, 8, 'L', 'R');

        return Self{
            row: row_number, 
            column: column_number,
            id: row_number as u32 * 8 + column_number as u32
        };
    }

    fn resolve(string: &str, initial_min: u8, initial_max: u8, lower_char: char, upper_char: char) -> u8{
        let mut min: u8 = initial_min;
        let mut max: u8 = initial_max;
        
        for letter in string.chars() {
            if letter as char == lower_char { // lower
                max = ((max - 1 - min) / 2) + min;
            } else if letter as char == upper_char { // upper
                min = ((max + 1 - min) / 2) + min;
            }
        }
        return min;
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.id).cmp(&(other.id))
    }
}
impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        (self.id) == (other.id)
    }
}

impl Eq for Seat { }

impl fmt::Display for Seat{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Row: {}, Column: {}, ID: {}", self.row, self.column, self.id)
    }
}
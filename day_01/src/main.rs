// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?

use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_to_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn cal_val(line: String) -> Option<u32> {
    let mut chars = line.chars();
    let l_num = chars.find(|&c| c.is_numeric()).and_then(char_to_digit);
    let r_num = chars.rfind(|&c| c.is_numeric()).and_then(char_to_digit);
    match (l_num, r_num) {
        (Some(l), Some(r)) => Some(l * 10 + r),
        (Some(l), None) => Some(l * 10 + l),
        (None, Some(r)) => Some(r * 10 + 1),
        _ => None,
    }
}

fn main() {
    let value = File::open("input.txt").map(BufReader::new).map(|reader| {
        reader
            .lines()
            .map_while(Result::ok)
            .filter_map(cal_val)
            .sum::<u32>()
    });
    println!("{:?}", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let ex_1 = "1abc2";
        let ex_2 = "pqr3stu8vwx";
        let ex_3 = "a1b2c3d4e5f";
        let ex_4 = "treb7uchet";
        assert_eq!(cal_val(ex_1.to_string()), Some(12));
        assert_eq!(cal_val(ex_2.to_string()), Some(38));
        assert_eq!(cal_val(ex_3.to_string()), Some(15));
        assert_eq!(cal_val(ex_4.to_string()), Some(77));
    }
}

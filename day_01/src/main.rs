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

use std::fs::File;
use std::io::{BufRead, BufReader};

const CHAR_DIGITS_THREE: [&str; 3] = ["one", "two", "six"];
const CHAR_DIGITS_FOUR: [&str; 3] = ["four", "five", "nine"];
const CHAR_DIGITS_FIVE: [&str; 3] = ["three", "seven", "eight"];
enum Number {
    Character(String),
    Digit(u32),
}

impl Number {
    fn to_u32(&self) -> Option<u32> {
        match self {
            Number::Digit(num) => Some(*num),
            Number::Character(char) => Some(match char.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => return None,
            }),
        }
    }
}

fn cal_val(line: String) -> Option<u32> {
    let mut l_chars = line.char_indices().peekable();
    let mut r_chars = line.char_indices().rev().peekable();
    let mut l_num: Option<u32> = None;
    let mut r_num: Option<u32> = None;

    let check_slice = |i: usize, len: usize, array: &[&str; 3]| -> Option<u32> {
        match line.get(i..i + len) {
            Some(slice) => {
                if array.contains(&slice) {
                    return Number::Character(slice.to_string()).to_u32();
                }
            }
            _ => return None,
        }
        None
    };

    while let Some((i, c)) = l_chars.next_if(|_| l_num.is_none()) {
        if let Some(digit) = c.to_digit(10) {
            if l_num.is_none() {
                l_num = Some(digit);
            }
        }
        let val = check_slice(i, 3, &CHAR_DIGITS_THREE)
            .or_else(|| check_slice(i, 4, &CHAR_DIGITS_FOUR))
            .or_else(|| check_slice(i, 5, &CHAR_DIGITS_FIVE));
        if l_num.is_none() {
            l_num = val;
        }
    }
    if l_num.is_some() && l_chars.next().is_some() {
        while let Some((i, c)) = r_chars.next_if(|_| r_num.is_none()) {
            if let Some(digit) = c.to_digit(10) {
                if r_num.is_none() {
                    r_num = Some(digit)
                }
            }
            let val = i.checked_sub(2)
                .and_then(|new_i| check_slice(new_i, 3, &CHAR_DIGITS_THREE))
                .or_else(|| i.checked_sub(3)
                    .and_then(|new_i| check_slice(new_i, 4, &CHAR_DIGITS_FOUR)))
                .or_else(|| i.checked_sub(4)
                    .and_then(|new_i| check_slice(new_i, 5, &CHAR_DIGITS_FIVE)));
            if r_num.is_none() {
                r_num = val;
            }
        }
    }

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
    println!("{:?}", value);
    let value2 = File::open("input2.txt").map(BufReader::new).map(|reader| {
        reader
            .lines()
            .map_while(Result::ok)
            .filter_map(cal_val)
            .sum::<u32>()
    });
    println!("{:?}", value2);
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

    #[test]
    fn test_2() {
        let ex_1 = "two1nine";
        let ex_2 = "eightwothree";
        let ex_3 = "abcone2threexyz";
        let ex_4 = "xtwone3four";
        let ex_5 = "4nineeightseven2";
        let ex_6 = "zoneight234";
        let ex_7 = "7pqrstsixteen";
        assert_eq!(cal_val(ex_1.to_string()), Some(29));
        assert_eq!(cal_val(ex_2.to_string()), Some(83));
        assert_eq!(cal_val(ex_3.to_string()), Some(13));
        assert_eq!(cal_val(ex_4.to_string()), Some(24));
        assert_eq!(cal_val(ex_5.to_string()), Some(42));
        assert_eq!(cal_val(ex_6.to_string()), Some(14));
        assert_eq!(cal_val(ex_7.to_string()), Some(76));
    }
}

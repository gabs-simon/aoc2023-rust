use super::common::parse_lines;
use regex::Regex;

pub fn problem_1() -> i32 {
  let mut digits: Vec<i32> = [].to_vec();

  let line_fn = |str: String| {
    let reg = Regex::new(r"\d").unwrap();
    let matches: Vec<i32> = reg.find_iter(&str).map(|m| {
      return m.as_str().parse::<i32>().unwrap();
    }).collect();

    let Some(first) = matches.first() else { panic!("No first number") };
    let Some(last) = matches.last() else { panic!("No last number") };

    let calculation = 10 * first + last;
    digits.push(calculation);
  };

  parse_lines("day1.txt", line_fn);

  let mut sum: i32 = 0;
  for calc in digits.iter() {
    sum = sum + calc;
  }

  return sum;
}

pub fn problem_2() -> i32 {
  let mut digits: Vec<i32> = [].to_vec();

  // Directly converts strings to numbers, handling the weird edge cases
  fn convert_to_number(n: &str) -> String {
    return n
      .replace("oneight", "18")
      .replace("twone", "21")
      .replace("threeight", "38")
      .replace("fiveight", "58")
      .replace("eightwo", "82")
      .replace("nineight", "98")
      .replace("one", "1")
      .replace("two", "2")
      .replace("three", "3")
      .replace("four", "4")
      .replace("five", "5")
      .replace("six", "6")
      .replace("seven", "7")
      .replace("eight", "8")
      .replace("nine", "9");
  }

  let line_fn = |str: String| {
    let reg = Regex::new(r"\d").unwrap();
    let number_string = convert_to_number(&str);
    let matches: Vec<i32> = reg.find_iter(number_string.as_str()).map(|m| {
      return m.as_str().parse::<i32>().unwrap();
    }).collect();

    let Some(first) = matches.first() else { panic!("No first number") };
    let Some(last) = matches.last() else { panic!("No last number") };

    let calculation = 10 * first + last;
    digits.push(calculation);
  };

  parse_lines("day1.txt", line_fn);

  let mut sum: i32 = 0;
  for calc in digits.iter() {
    sum = sum + calc;
  }

  return sum;
}
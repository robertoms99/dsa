use std::str::Chars;

pub fn hash(key: &str)-> usize {
  let chars = key.chars();
  let mut codes_sum = 0;

  for char in chars {
    codes_sum += char as usize;
  }

  return codes_sum;
}

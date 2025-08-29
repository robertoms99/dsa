use std::error::Error;

pub fn get_user_input() -> Result<String,Box<dyn Error>>{
  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;
  return Ok(input);
}

pub fn check_ms(message: &str) -> Result<&str, &str>  {
 if message.contains("stupid") ||  message.is_empty(){
   Err("ERROR: illegal")
 }else{
  Ok(message)
 }
}
pub struct App {
  data: String,
}

impl App {
  pub fn string_lexer(str: &str) -> Vec<String> {
    shlex::split(str).unwrap_or_default()
  }
}

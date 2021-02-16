pub struct Line {
  pub index: u64,
  pub content: String,
}

impl Line {
  pub fn get_index(&self) -> u64 {
    self.index
  }
  pub fn get_content(&self) -> &String {
    &self.content
  }
}

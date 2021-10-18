pub struct FindByIdOptions {
  pub(crate) with_relations: bool,
}

impl Default for FindByIdOptions {
  fn default() -> Self {
    FindByIdOptions {
      with_relations: false,
    }
  }
}

impl FindByIdOptions {
  pub fn build() -> Self {
    FindByIdOptions::default()
  }

  pub fn with_relations(mut self, status: bool) -> Self {
    self.with_relations = status;
    self
  }
}

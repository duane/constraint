use std::string::ToString;

pub struct Namer {
  prefix: &'static str,
  count: usize
}

impl Namer {
  pub fn new(prefix: &'static str) -> Namer {
    Namer{
      prefix: prefix,
      count: 1
    }
  }

  pub fn vend(&mut self) -> String {
    let name = self.prefix.to_string() + self.count.to_string().as_ref();
    self.count += 1;
    name
  }
}


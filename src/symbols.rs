pub trait Symbol {
  fn symbol(&self) -> &str;
}

#[macro_export]
macro_rules! symbols_str {
  ( $( $sym:expr ),* ) => {
    {
      let mut string = String::new();
			$(
				string += $sym.symbol();
			)*
      string
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn string_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1에 & 붙이면 에러: Add trait가 `&self`에 대해서 구현되어 있지 않음
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                        // https://learning.oreilly.com/library/view/the-rust-programming/9781492067665/xhtml/ch08.xhtml#ch08
                        // deref coercion
  }
}

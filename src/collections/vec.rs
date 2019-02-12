pub fn add(x: u32, y: u32) -> u32 {
  return x + y;
}

#[allow(dead_code)]
#[derive(Debug)]
struct My {
  num: i32,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn add_works() {
    assert_eq!(add(2, 2), 4);
    println!("asdf")
  }

  #[test]
  fn vec_get() {
    let v = vec![1, 2, 3];
    let el = v.get(1);
    if let Some(v) = el {
      assert_eq!(v, &2);
    } else {
      panic!("")
    }
    assert_eq!(el, Some(&2))
  }

  #[test]
  fn vec_mut_el() {
    let mut v = vec![100, 32, 57];
    // 인덱스를 통한 변경
    v[0] = 0;
    // 참조를 통한 변경
    for i in &mut v {
      *i += 50;
    }
    assert_eq!(v.get(0), Some(&50));
  }

  #[test]
  fn vec_ownership() {
    let _v = vec![My { num: 30 }];
    // cannot move out of borrowed content
    // let _n = _v[0];
  }
}

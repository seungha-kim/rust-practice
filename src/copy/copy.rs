// Drop, Copy가 동시에 적용될 수 없다.
#[derive(Clone, Copy, Debug)]
struct MyCopiable {
  num: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct NotCopiable {
  num: u32,
}

#[cfg(test)]
mod tests {
  use super::*;
  /// ownership move가 일어나는 경우
  #[test]
  fn drop_works() {
    let v = vec![1, 2, 3];
    let _v2 = v;
    // 아래 주석을 해제하면 컴파일이 안됨.
    // println!("{:?}", v); //  value borrowed here after move
  }

  /// ownership move가 일어나지 않는 경우
  #[test]
  fn copy_works() {
    let x = MyCopiable { num: 3 };
    // 위에서 Copy를 빼면 동작하지 않는다.
    let _y = x;
    println!("{:?}", x);
  }

  fn accept_only_copy<T>(_t: T)
  where
    T: Copy,
  {
  }

  #[test]
  fn reference_is_copiable() {
    // 참조는 Copy 된다!
    // 아래에서 & 지우면 컴파일 에러.
    let n = &NotCopiable { num: 3 };
    let _m = n;
    println!("{:?}", n);
  }

  #[test]
  fn reference_is_copiable2() {
    let n = &NotCopiable { num: 3 };
    // reference는 Copy trait의 구현체다!
    // https://medium.com/@vikram.fugro/references-and-copy-trait-in-rust-fc6495d70a20
    accept_only_copy(n);
  }
}

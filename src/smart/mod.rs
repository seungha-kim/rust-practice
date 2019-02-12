pub mod list;

use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::MyBox;
    use std::ops::Deref;
    #[test]
    fn test() {
        let x = MyBox::new(5);
        // Without the Deref trait, the compiler can dereference only & references.
        // 아래 두 줄이 같은 의미
        assert_eq!(*x, 5);
        assert_eq!(*(x.deref()), 5);
    }

    #[test]
    fn deref_coercion() {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }
        let x = MyBox::new(String::from("haha"));
        // &MyBox<String> -> &String
        hello(&x);

        // 위와 같은 의미
        hello(&(*x)[..]);
    }
}

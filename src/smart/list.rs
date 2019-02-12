use std::rc::Rc;

pub enum Elem<T> {
    Cons(T, Rc<Elem<T>>),
    Nil,
}

pub struct List<T> {
    head: Rc<Elem<T>>,
}

use Elem::*;

impl<T> Elem<T> {
    pub fn len(&self) -> usize {
        match self {
            // 원래는 이것 처럼 써야 하지만, 편의 문법이 추가됐다.
            // &Cons(_, ref next) => 1 + next.len(),
            Cons(_, ref next) => 1 + next.len(),
            Nil => 0,
        }
    }

    pub fn next_elem(&self) -> Option<&Elem<T>> {
        match self {
            Cons(_, ref next) => Some(next),
            Nil => None,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: Rc::new(Nil) }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        let cloned = Rc::clone(&self.head);
        List {
            head: Rc::new(Cons(value, cloned)),
        }
    }

    pub fn len(&self) -> usize {
        self.head.len()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            // deref coercion!
            current: &self.head,
            // current: self.head.borrow() <- 됨.
            // current: self.head <- 안됨. deref coercion이 일어나려면 반드시 reference여야 함.
        }
    }
}

pub struct Iter<'a, T> {
    current: &'a Elem<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            Cons(ref value, ref next) => {
                // deref coercion!
                self.current = next;
                Some(value)
            }
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list_works() {
        let list: List<u32> = List::new().prepend(1).prepend(2);
        let list2 = list.prepend(3).prepend(4);
        let list3 = list.prepend(5);
        assert_eq!(list.len(), 2);
        assert_eq!(list2.len(), 4);
        assert_eq!(list3.len(), 3);

        let mut iter = list.iter();
        let v = iter.next();
        assert_eq!(v, Some(&2));
        let v = iter.next();
        assert_eq!(v, Some(&1));
        let v = iter.next();
        assert_eq!(v, None);

        // structural sharing!
        assert!(std::ptr::eq(
            list2.head.next_elem().unwrap().next_elem().unwrap(),
            list3.head.next_elem().unwrap()
        ));

        for v in list.iter() {
            println!("{}", v);
        }
    }
}

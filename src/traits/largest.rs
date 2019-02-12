#[allow(dead_code)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_works() {
        let arr = [1, 2, 3];
        let l = largest(&arr);
        assert_eq!(l, 3);
    }

    #[test]
    fn largest_ref_works() {
        let arr = [1, 2, 3];
        let l = largest_ref(&arr);
        assert_eq!(l, &3);
    }
}

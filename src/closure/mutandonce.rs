#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn works() {
        let a = 1;
        let mut b = 1;
        let mut f = |x| b += x;
        f(a);
        assert_eq!(b, 2);
    }

    #[test]
    fn works2() {
        let x = String::from("hello");
        let f = || {
            // 아래 &를 빼면 move 일어나서 컴파일 안됨.
            let _y = &x;
        };
        f();
        println!("{}", x);
    }

    #[test]
    fn memo() {
        let mut memo: HashMap<u32, u32> = HashMap::new();
        let result = fibo(&mut memo, 10);
        assert_eq!(result, 55);
        let result = fibo(&mut memo, 20);
        assert_eq!(result, 6765);
    }

    fn fibo(cache: &mut HashMap<u32, u32>, n: u32) -> u32 {
        match (n, cache.get(&n)) {
            (0, _) => 0,
            (1, _) => 1,
            (_, Some(c)) => *c,
            (n, None) => {
                let result = fibo(cache, n - 1) + fibo(cache, n - 2);
                cache.insert(n, result);
                result
            }
        }
    }

    // use std::hash::Hash;

    // struct Memo<'a, T, U, F>
    // where
    //     F: Fn(&T) -> U,
    // {
    //     pub memo: HashMap<&'a T, U>,
    //     pub calculator: F,
    // }

    // impl<'a, T, U, F> Memo<'a, T, U, F>
    // where
    //     F: &'a Fn(&T) -> U,
    //     T: Eq + Hash,
    // {
    //     fn new(calculator: F) -> Memo<'a, T, U, F> {
    //         Memo {
    //             calculator,
    //             memo: HashMap::new(),
    //         }
    //     }
    //     fn get(&mut self, item: &'a T) -> &U {
    //         match self.memo.get(item) {
    //             Some(v) => v,
    //             None => {
    //                 let result = (self.calculator)(item);
    //                 self.memo.insert(item, result);
    //                 &result
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn memo2() {
    //     let mut fiboMemo = Memo::new(|n: &i32| -> i32 {
    //         match n {
    //             &0 => 0,
    //             &1 => 1,
    //             &n => fiboMemo.calculator(n - 1) + (fiboMemo.calculator)(n - 2),
    //         }
    //     });
    //     assert_eq!(fiboMemo.get(10), 55);
    // }

    use std::hash::Hash;

    struct Memo<K, V, F>
    where
        K: Eq + Hash,
        // F가 K를 소비해버리면 안되니깐..
        F: Fn(&K) -> V,
    {
        memo: HashMap<K, V>,
        calc: F,
    }

    impl<K, V, F> Memo<K, V, F>
    where
        K: Eq + Hash,
        F: Fn(&K) -> V,
    {
        fn new(calc: F) -> Memo<K, V, F> {
            Memo {
                memo: HashMap::new(),
                calc,
            }
        }
    }

    impl<K, V, F> Memo<K, V, F>
    where
        K: Eq + Hash,
        V: Copy,
        F: Fn(&K) -> V,
    {
        fn value(&mut self, k: K) -> V {
            match self.memo.get(&k) {
                Some(v) => *v,
                None => {
                    let v = (self.calc)(&k);
                    self.memo.insert(k, v);
                    v.clone()
                }
            }
        }
    }

    // // ??
    // impl<K, V, F> Memo<K, V, F>
    // where
    //     K: Eq + Hash,
    //     F: Fn(&K) -> V,
    // {
    //     // &V 가 &self 보다 짧아야 한다.
    //     fn value_ref(&mut self, k: K) -> &V {
    //         if !self.memo.contains_key(&k) {
    //             let v = (self.calc)(&k);
    //             self.memo.insert(k, v);
    //         }
    //         self.memo.get(&k).unwrap()
    //     }
    // }

    #[test]
    fn memo_copy_works() {
        // 근데 type inference 대단하네..
        let mut m = Memo::new(|x| x * 2);
        assert_eq!(m.value(2), 4);
    }

    #[test]
    fn memo_drop_works1() {
        let mut m = Memo::new(|s: &String| s.len());
        assert_eq!(m.value(String::from("haha")), 4);
    }

    #[test]
    fn memo_drop_works2() {
        let mut m = Memo::new(|s: &&str| s.len());
        assert_eq!(m.value("haha"), 4);

        // deref coercion
        assert_eq!(m.value(&String::from("haha")), 4);
    }

    #[test]
    fn capture_test() {
        fn _len(s: String) -> usize {
            s.len()
        }
        let name = String::from("kim");
        // 책에는 'borrowing' 이라고 나오긴 하는데....뭐 어떻게 되는 걸까.
        // 말은 borrowing이지만, reference는 아니고, 클로저 실행 시 '소유권이 넘어갔다가 다시 돌아온다' 이런 식으로 생각하면 되려나.
        let _f = || println!("{}", name);
        // 아래 주석을 푸는 것만으로도 move가 일어난다.
        // let _f = || _len(name);
        println!("{}", name);
    }
}

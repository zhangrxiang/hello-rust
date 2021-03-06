pub mod fn_test {
    pub mod test {
        use std::fmt::Display;
        use std::ops::Mul;

        //利用 Raw identifier 将语言关键字用作函数名
        fn r#match(needle: &str, haystack: &str) -> bool {
            haystack.contains(needle)
        }

        #[test]
        fn test_match() {
            assert!(r#match("foo", "foobar"))
        }

        fn modify(mut v: Vec<i32>) -> Vec<i32> {
            v.push(100);
            v
        }

        fn modify_ref(v: &mut Vec<i32>) {
            v.reverse();
        }

        #[test]
        fn test_modify() {
            let v = vec![1];
            let mut v2 = modify(v);
            println!("{:?}", v2);
            let v3 = v2.as_mut();
            modify_ref(v3);
            println!("{:?}", v3)
        }

        fn square<T: Mul<T, Output = T>>(x: T, y: T) -> T {
            x * y
        }

        #[test]
        fn test_square() {
            assert_eq!(square(2, 3), 6);
            assert_eq!(square::<i32>(2, 3), 6);
            assert_eq!(square(37.2, 41.1), 1528.92);
        }

        fn hello() {
            println!("hello")
        }

        fn one(i: i32) {
            println!("{}", i)
        }

        fn two(i: i32, str: &str) {
            println!("{} {}", i, str)
        }

        fn three<T: Ord + Display>(i: T) {
            println!("{}", i);
        }

        pub mod type_point {
            type MathOp = fn(i32, i32) -> i32;

            fn math(op: &str) -> MathOp {
                fn sum(a: i32, b: i32) -> i32 {
                    a + b
                }
                fn product(a: i32, b: i32) -> i32 {
                    a * b
                }
                match op {
                    "sum" => sum,
                    "product" => product,
                    _ => panic!("err"),
                }
            }

            fn math2(op: &str, a: i32, b: i32) -> i32 {
                math(op)(a, b)
            }

            #[test]
            fn test() {
                let sum = math("sum");
                println!("{}", sum(1, 2));
                let product = math("product");
                println!("{}", product(2, 3));
                println!("{}", math2("product", 2, 4))
            }
        }

        #[test]
        fn test_point_fn() {
            let f1: fn() = hello;
            f1();

            let f2: fn(i32) = one;
            f2(1000);

            let f3: fn(i32, &str) = two;
            f3(1, "hello");

            let f4: fn(i32) = three;
            f4(1);

            let f4: fn(String) = three;
            f4("hello".to_owned());

            let f5: fn(i32, i32) -> i32 = square;
            println!("{}", f5(2, 3));

            let f5: fn(f32, f32) -> f32 = square;
            println!("{}", f5(2.1, 3.1));
        }
    }

    pub mod closures {
        use rand::Rng;

        fn counter(i: i32) -> impl Fn(i32) -> i32 {
            move |n: i32| n + i
        }

        #[test]
        fn common() {
            let f = counter(3);
            assert_eq!(4, f(1));
            fn function(i: i32) -> i32 {
                i + 1
            }
            let closure_annotated = |i: i32| -> i32 { i + 1 };
            let closure_inferred = |i| i + 1;

            let i = 1;
            println!("function: {}", function(i));
            println!("closure_annotated: {}", closure_annotated(i));
            println!("closure_inferred: {}", closure_inferred(i));

            let one = || 1;
            println!("closure returning one: {}", one());
            println!("{}", { || 1 }());
            println!("{}", { || "string" }());
            println!("{}", { || "string".to_uppercase() }());
        }

        #[test]
        fn capture() {
            let name = "zing";
            let print = || println!("name is {}", name);
            print();

            let _name2 = name;
            print();

            let _name3 = &name;
            print();

            println!("name is {}", name);

            let mut count = 0;
            let mut inc = || {
                count += 1;
                println!("count {}", count);
            };
            inc();

            //^^^^^^ immutable borrow occurs here
            //let _reborrow = &count;
            inc();
            let _reborrow = &mut count;
            //inc();
        }

        // A function which takes a closure as an argument and calls it.
        // <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F)
        where
            // The closure takes no input and returns nothing.
            F: FnOnce(),
            // F: Fn(),
        {
            // ^ TODO: Try changing this to `Fn` or `FnMut`.
            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32
        where
            // The closure takes an `i32` and returns an `i32`.
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        #[test]
        fn input_parameters() {
            use std::mem;
            let greeting = "hello";
            // A non-copy type.
            // `to_owned` creates owned data from borrowed one
            let mut farewell = "goodbye".to_owned();

            // Capture 2 variables: `greeting` by reference and
            // `farewell` by value.
            let diary = || {
                // `greeting` is by reference: requires `Fn`.
                println!("I said {}", greeting);

                // Mutation forces `farewell` to be captured by
                // mutable reference. Now requires `FnMut`.
                farewell.push_str("!!!");
                println!("Then I screamed {}.", farewell);
                println!("Now I can sleep. zzzzz");

                // Manually calling drop forces `farewell` to
                // be captured by value. Now requires `FnOnce`.
                mem::drop(farewell);
            };

            // Call the function which applies the closure.
            apply(diary);

            // `double` satisfies `apply_to_3`'s trait bound
            let double = |x| 2 * x;

            println!("3 doubled: {}", apply_to_3(double));
        }

        mod anonymity {
            // `F` must implement `Fn` for a closure which takes no
            // inputs and returns nothing - exactly what is required
            // for `print`.
            fn apply<F>(f: F)
            where
                F: Fn(),
            {
                f();
            }

            fn apply2<F>(f: F)
            where
                F: FnOnce(),
            {
                f();
            }

            fn apply3<F>(mut f: F)
            where
                F: FnMut() -> i8,
            {
                println!("{}", f());
            }

            #[test]
            fn main() {
                let x = 7;

                // Capture `x` into an anonymous type and implement
                // `Fn` for it. Store it in `print`.
                let print = || println!("{}", x);

                apply(print);
                apply2(print);

                let a = 10;
                let f = || a + 2;
                apply3(f);
            }
        }

        mod input_functions {
            // Define a function which takes a generic `F` argument
            // bounded by `Fn`, and calls it
            fn call_me<F: Fn()>(f: F) {
                f();
            }

            // Define a wrapper function satisfying the `Fn` bound
            fn function() {
                println!("I'm a function!");
            }

            #[test]
            fn main() {
                // Define a closure satisfying the `Fn` bound
                let closure = || println!("I'm a closure!");
                call_me(closure);
                call_me(function);
            }

            fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
                let s = "second";
                c.push(Box::new(|| println!("first")));
                c.push(Box::new(move || println!("{}", s)));
                c.push(Box::new(|| println!("third")));
            }

            #[test]
            fn test_boxed_closure() {
                let mut c: Vec<Box<dyn Fn()>> = vec![];
                boxed_closure(&mut c);
                for f in c {
                    f();
                }
            }

            pub mod p {
                trait Any {
                    fn any<F>(&self, f: F) -> bool
                    where
                        Self: Sized,
                        F: Fn(u32) -> bool;
                }

                impl Any for Vec<u32> {
                    fn any<F>(&self, f: F) -> bool
                    where
                        Self: Sized,
                        F: Fn(u32) -> bool,
                    {
                        for &x in self {
                            if f(x) {
                                return true;
                            }
                        }
                        false
                    }
                }
                #[test]
                fn run() {
                    let v = vec![1, 2, 3];
                    let b = v.any(|x| x == 3);
                    println!("{}", b);
                }

                fn call<F>(f: F) -> i32
                where
                    F: Fn(i32) -> i32,
                {
                    f(10)
                }
                fn call2<F>(f: F, i: i32) -> i32
                where
                    F: Fn(i32) -> i32,
                {
                    f(i)
                }
                fn counter(i: i32) -> i32 {
                    i + 1
                }
                #[test]
                fn r2() {
                    let res = call(counter);
                    println!("{:?}", res);
                    let res = call2(counter, 10);
                    println!("{:?}", res);
                }
            }
        }

        mod output_parameters {
            fn create_fn() -> impl Fn() {
                let text = "Fn".to_owned();

                move || println!("This is a: {}", text)
            }

            fn create_fn_mut() -> impl FnMut() {
                let text = "FnMut".to_owned();

                move || println!("This is a: {}", text)
            }

            fn create_fn_once() -> impl FnOnce() {
                let text = "FnOnce".to_owned();

                move || println!("This is a: {}", text)
            }

            #[test]
            fn main() {
                let fn_plain = create_fn();
                let mut fn_mut = create_fn_mut();
                let fn_once = create_fn_once();

                fn_plain();
                fn_mut();
                fn_once();
            }

            #[test]
            fn run() {
                let v = vec!["563913.060", "563913.080"];
                let x: Vec<f32> = v.iter().map(|f| f.parse::<f32>().unwrap()).collect();
                println!("{:?}", x);
                println!("{}", "563913.060".parse::<f32>().unwrap());
                println!("{}", "563913.090".parse::<f32>().unwrap());
                println!("{}", "563913.050".parse::<f32>().unwrap());
                println!("{}", "563913.100".parse::<f32>().unwrap());
                println!("{}", "563913.060".parse::<f64>().unwrap());
                println!("{}", "563913.090".parse::<f64>().unwrap());
                println!("{}", "1.0001".parse::<f32>().unwrap());
                println!("{}", "123456789.0001".parse::<f32>().unwrap());
            }
        }
    }

    // • FnOnce , 表示闭包通过转移所有权来捕获环境中的自由变量 ,同时意味着该闭包没
    // 有改变环境的能力,只能调用一次 ,因为该闭包会消耗自身。对应self。
    // ·对于移动语义类型,执行移动语义 ,转移所有权来进行捕获。
    mod fn_once {
        use std::fmt::{Debug, Display};

        #[test]
        fn test_fn_once() {
            let s: String = "hello world".to_string();
            println!("{}", s);
            let f = || s;
            f();
            //error[E0382]: borrow of moved value: `s`
            // println!("{}", s);
        }

        fn consume_with_relish<F: FnOnce() -> String>(func: F) {
            // `func` consumes its captured variables, so it cannot be run more
            // than once.
            println!("Consumed: {}", func());
            println!("Delicious!");
            // Attempting to invoke `func()` again will throw a `use of moved
            // value` error for `func`.
            //func();
        }

        fn func<A, F>(f: F)
        where
            A: Display + Debug,
            F: FnOnce() -> A,
        {
            println!("{:?}", f());
            //println!("{:?}", f());
        }

        #[test]
        fn test() {
            let x = String::from("x");
            let consume_and_return_x = move || x;
            consume_with_relish(consume_and_return_x);

            let i = 10;
            let f = || i * i;
            func(f);

            let t = (10, 20);
            let f2 = || t.0 * t.1;
            func(f2);

            let t2 = ("hello", " ", "word");
            let f3 = move || t2.0.to_owned() + t2.1 + t2.2;
            func(f3);

            let x = String::from("x");
            let f = move || x;
            func(f);
        }
    }

    // • FnMut ,表示闭包以可变借用的方式来捕获环境中的自由变量 ,同时意味着该闭包有
    // 改变环境的能力 ,也可以多次调用 。 对应&mut self。
    // ·对于可变绑定,并且在闭包中包含对其进行修改的操作,则以可变引用(&mut T)来进行捕获 。
    mod fn_mut {
        #[test]
        fn test_fn_mut() {
            let mut s: &str = "string";
            println!("{}", s);
            let mut f = || s = "other";
            f();
            println!("{}", s);
            let mut f = || s = "other";
            f();
            f();
            println!("{}", s);
        }

        fn do_twice<F>(mut func: F)
        where
            F: FnMut(),
        {
            func();
            func();
        }

        #[test]
        fn test() {
            let mut x: usize = 1;
            {
                // --> src\test\fn_test.rs:280:44
                // |
                // 280 |                 let add_two_to_y = move || y += 2;
                // |                                            ^
                // |
                // = note: `#[warn(unused_variables)]` on by default
                // = help: did you mean to capture by reference instead?

                let mut y: usize = 1;
                let add_two_to_x = || x += 2;
                let add_two_to_y = || y += 2;
                do_twice(add_two_to_x);
                assert_eq!(x, 5);
                println!("{}", x);
                do_twice(add_two_to_y);
                println!("{}", y);
            }

            x = 72;
            let mut string = String::new();
            let str = || {
                // string.insert_str(x, "hello ");
                //string.add("hello ");
                string.push(x as u8 as char);
                x += 1;
            };
            do_twice(str);
            //do_twice(str);
            println!("{}", string);
        }
    }

    // Fn ,表示闭包以不可变借用的方式来捕获环境中的自由变量 ,同时也表示该闭包没有
    // 改变环境的能力 , 并且可以多次调用。对应 &self。
    // ·对于复制语义类型 ,以不可变引用 (&T)来进行捕获 。
    mod fn_test {
        #[test]
        fn test_fn() {
            let s = "hello";
            let f = move || println!("{:?}", s);
            f();
            f();
            f();
            println!("{}", s);
        }

        fn call_with_one<F>(func: F, size: usize) -> usize
        where
            F: Fn(usize) -> usize,
        {
            func(size)
        }

        #[test]
        fn test() {
            let double = |x| x * 2;
            assert_eq!(call_with_one(double, 1), 2);
            println!("{}", call_with_one(double, 2))
        }
    }
}

// 如果闭包中没有捕获任何环境变量,则默认自动实现 Fn
// · 如果闭包中捕获了复制语义类型的环境变量,则 :
//      〉如果不需要修改环境变量,无论是否使用 move 关键字,均会自动实现 Fn 。
//      〉如果需要修改环境变量 ,则自动实现 FnMut。
// · 如果闭包中捕获了移动语义类型的环境变量,则 :
//      〉如果不需要修改环境变量,且没有使用 move 关键字,则自动实现 FnOnce 。
//      〉如果不需要修改环境变量,且使用了 move关键字,则自动实现 Fn 。
//      〉如果需要修改环境变量,则自动实现 FnMut 。
// · 使用 move 关键字,如果捕获的变量是复制语义类型的 ,则闭包会自动实现
// Copy/Clone , 否则不会自动实现 Copy/Clone 。

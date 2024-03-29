struct User {
    active: bool,
    username: String,
    age: u64,
}

fn main() {
    five();
    for_l();
    heap_copy();
    ownership();
    ref_r();
    mut_r();
    ref2();
    dangling_p();

    let k = String::from("first word");
    let t: &str = first_word(&k);

    println!("t is {t}");

    let user = User {
        active: true,
        username: String::from("ijat"),
        age: 27,
    };

    let name = user.username;
    let student = user.age;

    println!("username {name} age {student}");

    if user.active {
        println!("user is active");
    }

    let mut user2 = User {
        username: String::from("jackjack"),
        ..user
    };

    user2.username = String::from("jackie");

    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 1, 2);

    struct Rect {
        width: u32,
        height: u32,
    }

    let act_rect = Rect {
        width: 10,
        height: 20,
    };

    println!("The area of the rectangle is {}", rect(&act_rect));

    fn rect(dimension: &Rect) -> u32 {
        dimension.width * dimension.height
    }

    let act_rect2 = (30, 50);

    println!("the area of rectangle 2 is {}", rect2(act_rect2));

    fn rect2(dimension: (u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rectangle = Rectangle {
        width: 20,
        height: 30,
    };

    println!("rectangle is {:?}", rectangle);
    println!("rectangle printing using # {:#?}", rectangle);

    let scale = 2;
    let rect_debug = Rectangle {
        width: dbg!(20 * scale),
        height: 50,
    };

    dbg!(rect_debug);

    // EOF struct

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect_method = Rectangle {
        width: 11,
        height: 12,
    };

    println!("rect_method = {}", rect_method.area());

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(20);

    println!("Square dimension is - {:?}", sq);

    #[derive(Debug)]
    #[allow(dead_code)]
    enum People {
        Name(String),
        Age(u32),
    }

    let people_name = People::Name(String::from("Johnson"));
    let people_age = People::Age(28);

    println!("name = {:?}, age = {:?} ", people_name, people_age);

    let some_number = Some(5);
    let some_char = Some("e");

    println!("some_number {:?}, some_char {:?}", some_number, some_char);

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
    }

    #[allow(dead_code)]
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny");
                1
            }
            Coin::Nickel => 5,
        }
    }

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The max config is {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("The max config is {max}")
    }

    let dice_roll = 9;
    match dice_roll {
        3 => println!("its 3"),
        7 => println!("its 7"),
        _ => println!("neither"),
    }
<<<<<<< HEAD
=======

    // VECTOR

    // let mut v = Vec::new();

    // v.push(1);
    // v.push(2);
    // v.push(3);

    let vconst = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vconst[2];
    println!("The third is {third}");

    let third2: Option<&i32> = vconst.get(2);
    match third2 {
        Some(third) => println!("The third element is {third}"),
        None => println!("No third element"),
    }
>>>>>>> b9adc24 (More study -2)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn dangling_p() {
    let reference_to_nothing = dangle();

    println!("reference to nothing {reference_to_nothing}");
}

#[allow(clippy::let_and_return)]
fn dangle() -> String {
    let s = String::from("dangle");

    s
}

fn ref2() {
    let mut s = String::from("ref2");

    let r1 = &s;
    let r2 = &s;

    println!("ref2 = {r1} {r2}");
    let r3 = &mut s;

    println!("ref2 after = {r3}");
}

fn mut_r() {
    let mut s = String::from("mutable str");

    change(&mut s);

    println!("s in mutr is {s}");
}

fn change(some_str: &mut String) {
    some_str.push_str(" by change fn");
}

fn ref_r() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("length of {s1} is {len}");
}

fn calc_len(str: &str) -> usize {
    str.len()
}

fn five() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn for_l() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    for element in arr {
        println!("element: {element}");
    }

    for number in (1..4).rev() {
        println!("number: {number}");
    }
}

fn heap_copy() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1} -- s2 = {s2}");
}

fn ownership() {
    let s = String::from("test");

    take_own(s.clone());

    println!("s is {s}");

    let x = 5;

    make_copy(x);
}

fn take_own(str: String) {
    println!("str is {str}");
}

fn make_copy(i: u32) {
    println!("i is {i}");
}

use std::fmt::format;

const MAX_CONCURRENCY_NUM: u32 = 1_000;


fn do_match_value() {


    // match array
    do_match_array();
    // match enums
    do_match_enum();
    // pointer/ref
    do_match_ref();
    // structs
    do_match_struct();

    // basic match
    do_basic_match();
}

enum ColorSetting {
    RED,
    YELLOW,
    BLUE,
    GREEN,
}

fn name_in_color_setting(s: ColorSetting) -> &'static str {
    match s {
        ColorSetting::RED => "red",
        ColorSetting::YELLOW => "yellow",
        ColorSetting::BLUE => "blue",
        ColorSetting::GREEN => "green",
    }
}

fn get_number(n: i32) -> &'static str {
    match n {
        1 => "ONE",
        2 | 3 => "TWO OR THREE",
        4..=10 => "BETWEEN 4 AND 10",
        _ => "Something else number",
    }
}


fn do_basic_match() {
    println!("color: {} ", name_in_color_setting(ColorSetting::RED));
    println!("color: {} ", name_in_color_setting(ColorSetting::GREEN));

    do_match_binding();
    println!("Nested: {}", get_str_option(Some(Some(100))));
    println!("Nested: {}", get_str_option(Some(None)));
    println!("Nested: {}", get_str_option(None));

    let op = Some(9);
    let number = match op {
        Some(i) => i,
        _ => 0
    };
    println!("match some n: {}", number);
    do_if_let();
    do_let_else();
    do_while_let();

    fn get_value_or_default(opt: Option<i32>) -> i32 {
        opt.unwrap_or_else(|| 0)
    }
    println!(">>> {}", get_value_or_default(Some(1)));

    do_methods();
}

fn do_methods() {
    struct Counter {
        count: Option<i32>,
    }

    impl Counter {
        // Instance method to get count or default
        fn get_count(&self) -> i32 {
            self.count.unwrap_or_else(|| 0)
        }
    }
    let counter1 = Counter { count: Some(5) };
    let counter2 = Counter { count: None };
    println!("counter 1: {}", counter1.get_count());
    println!("counter 2: {}", counter2.get_count());

    do_closures();
}

fn do_closures() {
    let process_option = |opt: Option<i32>| -> i32 {
        match opt {
            Some(value) => value * 2,
            None => 0,
        }
    };

    let some = Some(9);
    let none: Option<i32> = None;
    println!("Some: {}", process_option(some)); // Output: Some: 10
    println!("None: {}", process_option(none)); // Output: None: 0

    struct Processing {
        value: Option<i32>,
    }

    impl Processing {
        fn apply<F>(&self, f: F) -> i32
        where
            F: Fn(Option<i32>) -> i32,
        {
            f(self.value)
        }
    }

    let processing = Processing { value: Some(9) };
    let double = |opt: Option<i32>| match opt {
        Some(v) => v * 2,
        None => 0,
    };
    println!("processor result: {}", processing.apply(double));

    let mut opt = Some(String::from("Rust"));
    let consume = |opt: Option<String>| {
        let Some(s) = opt else { return "No string".to_string(); };
        format!("Consumed: {}", s)
    };

    println!("Result: {}", consume(opt)); // Output: Result: Consumed: Hello
    // println!("Data: {:?}", opt); // Error: opt was moved into closure

    let opt_some = vec![Some(1), None, Some(3)];
    let process = |opt: Option<i32>| opt.unwrap_or(0);
    let r: Vec<i32> = opt_some.into_iter().map(process).collect();
    println!("vec numbers: {:?}", r); // Output: Results: [1, 0, 3]

    fn apply_to_option<T, F>(opt: Option<T>, f: F) -> Option<T>
    where
        F: Fn(T) -> T,
    {
        opt.map(f)
    }

    let double = |x: i32| x * 2;
    let result = apply_to_option(Some(9), double);
    println!("Result: {:?}", result); // Output: Result: Some(10)
    let none: Option<i32> = None;
    println!("None: {:?}", apply_to_option(none, double)); // Output: None: None

    // map method
    let numbers = vec![10, 20, 30];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("map vec values: {:?}", doubled); // Outputs: [20, 40, 60]

    let mut numbers1 = vec![11, 12, 13, 14];
    let evens: Vec<i32> = numbers1.into_iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", evens); // Outputs: [12, 14]

    let numbers2 = vec![11, 12, 13];
    let sum = numbers2.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum); // Outputs: 36

    let numbers3 = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = numbers3
        .into_iter()
        .filter(|x| x % 2 == 0) // Keep even numbers
        .map(|x| x * 3)          // Triple them
        .collect();
    println!("{:?}", result); // Outputs: [6, 12]
}

fn do_while_let() {
    let mut op = Some(9);

    // This reads: "while `let` destructures `op`
    // into `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = op {
        if i > 9 {
            println!("Greater than 9, quit!");
            op = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            op = Some(i + 1);
        }
    }
}

fn do_let_else() {
    fn process_value_with_double(opt: Option<i32>) -> i32 {
        let Some(v) = opt else { return 0};
        v * 2
    }
    let some_v = Some(9);
    let no_v: Option<i32> = None;
    println!(" Some {}", process_value_with_double(some_v));
    println!(" None: {}", process_value_with_double(no_v));
}

fn do_if_let() {
    let op: Option<i32> = None;
    if let Some(i) = op {
        println!("match some n: {}", i);
    } else {
        println!("match some n: {}", 0);
    }
}

fn get_str_option(value: Option<Option<i32>>) -> i32 {
    match value {
        Some(Some(num)) => num,
        Some(None) => 0,
        None => -1
    }
}

fn do_match_binding() {
    let number = 8;
    let result = match number {
        n if n % 2 == 0 => format!(" {} 是 偶数", n),
        n => format!(" {} 是奇数", n)
    };
    println!("{}", result); //
}

fn do_match_struct() {;
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => {
            println!("Should never happen.")
        }
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }
}

fn do_match_ref() {
    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 10;
    let mut mut_value = 100;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn do_match_enum() {
    let color = Color::HSV(122, 17, 40);

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => {

            println!("Hue: {}, saturation: {}, value: {}!", h, s, v)
        }

        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}

fn do_match_array() {
    // Try changing the values in the array, or make it a slice!
    let array = [10, 20, 60];
    match array {
        // Binds the second and the third elements to the respective variables
        [20, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [90, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [30, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
fn main() {
    let a = 5; // ①
    println!("initial a: {}", a);
    {
        let a = a + 1; // ②
        println!("Shadowing a: {}", a);
    }
    let a = a * 2; // ③
    println!("final a: {}", a);


    let n = 159143;
    if n % 2 != 0 { // ①
        println!("n 是基数");
    } else { // ②
        println!("n 是偶数");
    }

    let mut c = 0u8;
    loop {
        c += 1;
        if c == 3 {
            println!("c 是 3，跳过接下来的业务逻辑，执行跳到 loop 声明处继续执行。");
            // Skip the rest of this iteration
            continue;
        }
        if c == 10 {
            println!("c 是 10 ，停止执行");
            break;
        }
    }

    let mut cc = 0u8;
    while (cc != 10) {
        cc += 1;
        if cc == 3 {
            println!("cc 是 3，跳过接下来的业务逻辑，执行跳到 while 声明处继续执行。");
            // Skip the rest of this iteration
            continue;
        }
    }
    println!("cc 是 10 ，停止执行");

    for n in 1..10 {
        if n % 2 == 0 {
            println!("偶数: {} ", n);
        } else {
            println!("基数: {}", n);
        }
    }

    for n in 1..=10 {
        if n % 2 == 0 {
            println!(">偶数: {} ", n);
        } else {
            println!(">基数: {}", n);
        }
    }

    let car_names = vec!["BYD", "赛力斯", "特斯拉"];

    for car_name in car_names.iter() {
        match car_name {
            &"BYD" => println!(" 比亚迪车!"),
            _ => println!("车： {}", car_name),
        }
    }
    println!("车品牌: {:?}", car_names);

    let car_names2 = vec!["BYD", "赛力斯", "特斯拉"];
    for car_name in car_names2.into_iter() {
        match car_name {
            "BYD" => println!(" 比亚迪车!"),
            _ => println!("车： {}", car_name),
        }
    }
    // car_names2 不再可用
    // println!("车品牌: {:?}", car_names2);
    let mut car_names3 = vec!["BYD", "赛力斯", "特斯拉"];
    for car_name in car_names3.iter_mut() {
        *car_name = match car_name {
            &mut "BYD" => "BYD 品牌!",
            _ => "其他车品牌",
        }
    }
    println!("车品牌: {:?}", car_names3);

    do_match_value();
}

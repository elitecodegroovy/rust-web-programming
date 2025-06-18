
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
}

fn do_match_struct() {
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

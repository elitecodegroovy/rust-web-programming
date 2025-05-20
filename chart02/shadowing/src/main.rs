
const MAX_CONCURRENCY_NUM: u32 = 1_000;

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
}

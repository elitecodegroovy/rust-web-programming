
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
}

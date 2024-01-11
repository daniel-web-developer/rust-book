use std::io;

fn main() {

    let mut max = String::new();

    loop {

        println!("Type the desired nth number: ");

        io::stdin().read_line(&mut max)
            .expect("Error reading line.");

        let max: i32 = match max.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                max.clear();
                continue;
            },
        };

        println!("The nth number of the Fibonacci sequence is {}", fib(max));
        break;

    }

}

fn fib(max: i32) -> i64 {
    let mut a: i64;
    let mut b: i64 = 0;
    let mut num: i64 = 0;
    let mut i: i32 = 1;

    while i <= max {
        a = b;
        b = num;
        num = a + b;

        if b == 0 { // hardcoded so the sequence follows 0, 1, 1, 2...
            num = 1;
        }

        i += 1;
    }

    num
}

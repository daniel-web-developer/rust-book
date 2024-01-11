use std::io;

fn main() {

    let mut temp = String::new();
    let mut unit = String::new();

    loop {

        println!("Please input the temperature in numbers: ");

        io::stdin()
            .read_line(&mut temp).expect("Failed to read temperature");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        loop {

            println!("Type 1 if you wanna convert from Fahrenheit to Celsius and 2 otherwise");

            io::stdin()
                .read_line(&mut unit).expect("Failed to read unit.");

            let unit: char = match unit.trim().parse() {
                Ok(value) => value,
                Err(_) => continue,
            };

            match unit {
                '1' => {
                    println!("In Celsius, the temperature you typed is {}", (5.0/9.0) * (temp - 32.0));
                    break;
                },
                '2' => {
                    println!("In Fahrenheit, the temperature you typed is {}", (temp * (9.0/5.0)) + 32.0);
                    break;
                },
                _ => panic!("You didn't type 1 nor 2."),
            };
        }
        break;
    }
}


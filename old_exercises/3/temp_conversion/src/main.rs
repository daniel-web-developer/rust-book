use std::io;

fn main() {

    let mut temp = String::new();
    let mut option = String::new();

    loop {
        println!("Type '1' to convert from C to F, or '2' to do otherwise: ");

        io::stdin().read_line(&mut option).expect("Error while reading option.");

        let option = option.trim();

        if option != "1" && option != "2" {
            println!("Please input 1 or 2.");
            let _ = io::stdin().read_line(&mut String::new()).expect("Error clearing input"); // discarding input

            continue;
        } else {
            break;
        }
    }

    let option = option.trim(); // this is needed because this same line inside the first loop only
                                // works until the first loop ends

    loop {
        println!("Please input the temperature: ");

        io::stdin().read_line(&mut temp).expect("Error while inputing temperature.");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                let _ = io::stdin().read_line(&mut String::new()).expect("Error clearing input"); // discarding input

                continue;
            }
        };

        if (option == "1" && temp < -273.15) || (option == "2" && temp < -459.67) {
            println!("Please, input a temperature above absolute zero.");
            continue;
        }

        break;
    }

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("This is impossible"),
    };

    if option == "1" {
        let new_temp = (temp * (9.0/5.0)) + 32.0;

        println!("Initial temperature: {temp} °C. Converted temperature: {new_temp} °F.");
    } else {
        let new_temp = (temp - 32.0) * (5.0/9.0);

        println!("Initial temperature: {temp} °F. Converted temperature: {new_temp} °C.");
    }

}

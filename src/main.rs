use std::io;

fn fahren_to_cels(temp: f64) -> f64{
    ((temp-32.0) * 5.0/9.0)
}

fn cels_to_fahren(temp: f64) -> f64{
    ((temp*9.0/5.0) + 32.0)
}

fn main() {
    loop {
        println!("|Temparature Convert Options|");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Close the program");
        let mut conv_temp = String::new();
        io::stdin()
            .read_line(&mut conv_temp)
            .expect("Faild to readline");
        let conv_temp = conv_temp.trim();
        let conv_temp = match conv_temp.trim() {
            "1" => 1,
            "2" => 2,
            "3" => break,
            _ => {
                println!("Please inter number 1 or 2");
                continue;
            } 
        };

        print!("Please inter the temperature: ");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Faild to read Temparature");
        let temparature = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please inter the correct temparature");
                continue;
            }
        };
        if conv_temp == 1 {
            let mut converted_temperature = fahren_to_cels(temparature);
            println!("The converted temperature is {}", converted_temperature);
        } else {
            let mut converted_temperature = cels_to_fahren(temparature);
            println!("The converted temperature is {}", converted_temperature);
        }
    }
}



use std::error::Error;

pub fn temperature_calculator() -> Result<(), Box<dyn Error>> {
    let mut input = String::from("");

    println!("enter temperature:");
    std::io::stdin().read_line(&mut input)?;
    println!("enter unit:");
    std::io::stdin().read_line(&mut input)?;

    let mut arr = input.split("\n").collect::<Vec<&str>>();
    arr.pop();
    let unit = arr.pop();
    let mut temperature = arr.pop().unwrap_or("0").parse::<i32>()?;

    match unit {
        Some("c") => {
            temperature = temperature * 9 / 5 + 32;
            println!("new temperature is: {}f", temperature);
        }
        Some("f") => {
            temperature = (temperature - 32) * 5 / 9;
            println!("new temperature is: {}c", temperature);
        }
        _ => {
            println!("must be f or c");
        }
    }

    Ok(())
}

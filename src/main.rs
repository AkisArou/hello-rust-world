use std::io;

fn main() -> io::Result<()> {
    match get_data() {
        Ok (_) => println!("Thank you for using our advanced application! Have a great day!\n"),
        Err(_) => println!("YOU DID SOMETHING WRONG..."),
    }

    Ok(())
}

fn get_data() -> io::Result<()> {
    let name = input_str("Please enter your name: ")?;
    let last_name = input_str("Please enter your last name: ")?;
    let year_born = input_int("Please enter the year you were born: ")?;
    let age = (2020 - year_born).to_string();

    println!("\nYour full name is: {} {}, and you are {} years old\n",
        name.trim(),
        last_name.trim(),
        age
    );

    Ok(())
}

fn input_str(question: &str) -> io::Result<String> {
    let mut input = String::new();
    println!("{}", String::from("\n") + question);
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn input_int(question: &str) -> io::Result<i16> {
    match input_str(question).unwrap().trim().parse::<i16>() {
        Ok (y) => Ok(y),
        Err(_) => input_int(question),
    }
}
 
use crate::solutions::day1::run_day1;
pub mod solutions;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(day) => match day.as_str() {
            "day1" => run_day1()?,
            // Add more days here...
            _ => println!("Invalid day!"),
        },
        None => {
            // If no day is specified, run all days
            run_day1()?;
            // Add more days here...
        }
    }
    Ok(())  
}

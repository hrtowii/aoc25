use clap::Parser;
use std::fs::File;
use std::io::Read;
mod days;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input_path: Option<String>,
}

impl Args {
    fn input(&self) -> String {
        self.input_path
            .clone()
            .unwrap_or_else(|| format!("./inputs/{}.txt", self.day))
    }
}

fn solve(day: &u8, input: &mut String) -> String {
    match day {
        1 => days::day1::day1(input),
        2 => days::day2::day2(input),
        _ => "unimplemented".to_string(),
    }
}
fn main() {
    let args = Args::parse();
    let input = File::open(args.input());
    let thestring: &mut String = &mut String::new();
    match input {
        Ok(mut thefile) => {
            println!("got file:");
            _ = thefile.read_to_string(thestring);
            let x = solve(&args.day, thestring);
            println!("day {} sol {}", &args.day, x);
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

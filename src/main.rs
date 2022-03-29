use std::env::args;
use wordle_solver::Error;

fn main() {
    let command = match Command::get() {
        Some(c) => c,
        None => {
            println!("Invalid command. Expected one of the following:");
            println!("wordle help");
            help();
            return;
        }
    };

    if !command.correct_num_args() {
        println!("Incorrect number of arguments.");
        return;
    }

    match command {
        Command::Help => {
            help();
        }
        Command::Solve => {
            if let None = wordle_solver::run::solve() {
                print_error(Error::DataRead);
            }
        }
        Command::Test => {
            if let None = wordle_solver::run::test() {
                print_error(Error::DataRead);
            }
        }
        Command::Build => {
            let mut pool = String::from("input/pool.txt");
            let mut targets = String::from("input/targets.txt");
            let mut solver: u8 = 0;
            if args().count() == 5 {
                match args().nth(2).unwrap() {
                    p if &p == "d" => (),
                    path => pool = path,
                }
                match args().nth(3).unwrap() {
                    p if &p == "d" => (),
                    path => targets = path,
                }
                match args().nth(4).unwrap().parse() {
                    Ok(s) => solver = s,
                    _ => {
                        println!("Expected integer solver ID.");
                        return;
                    }
                }
            }
            if let Err(e) = wordle_solver::run::build(&pool, &targets, solver) {
                print_error(e);
            }
        }
    }
}

fn help() {
    println!("wordle build");
    println!("wordle build <words path> <targets path> <solver ID>");
    println!("wordle solve");
    println!("wordle test");
}

fn print_error(error: Error) {
    println!(
        "{}",
        match error {
            Error::PoolRead => "Failed to read words file.",
            Error::TargetsRead => "Failed to read targets file.",
            Error::PoolFormat => "Words file formatted incorrectly.",
            Error::TargetsFormat => "Targets file formatted incorrectly.",
            Error::PoolLength => "Words file too long.",
            Error::SolverID => "Invalid solver ID.",
            Error::DataWrite => "Failed to write data file.",
            Error::DataRead => "Data file missing or corrupted. Please build.",
        }
    );
}

enum Command {
    Help,
    Build,
    Solve,
    Test,
}

impl Command {
    fn get() -> Option<Command> {
        match args().nth(1)?.as_str() {
            "help" => Some(Command::Help),
            "build" => Some(Command::Build),
            "solve" => Some(Command::Solve),
            "test" => Some(Command::Test),
            _ => None,
        }
    }

    fn correct_num_args(&self) -> bool {
        let count = args().count();
        match self {
            Command::Build => count == 2 || count == 5,
            _ => count == 2,
        }
    }
}

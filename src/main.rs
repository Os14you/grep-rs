use std::{env, fs::File, io::{BufRead, BufReader}};

struct Config {
    pattern: String,
    files: Vec<String>,
    case_sensitivity: bool,
    line_numbers: bool,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} PATTERN FILE... [-i] [-n]", args[0]));
        }

        let mut case_sensitivity = false;
        let mut line_numbers = false;
        let mut non_flagged = Vec::new();

        for arg in &args[1..] {
            if arg == "-i" {
                case_sensitivity = true;
            } else if arg == "-n" {
                line_numbers = true;
            } else {
                non_flagged.push(arg.clone());
            }
        }

        let pattern = non_flagged[0].clone();
        let files = non_flagged[1..].to_vec();

        Ok(Config { pattern, files, case_sensitivity, line_numbers })

    }
}

fn grep(reader: &mut BufReader<File>, pattern: &str, case_sensitivity: bool, line_number: bool) {
    let mut line = String::new();
    let mut i = 0;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        let matched;
        if case_sensitivity {
            matched = line.to_lowercase().contains(&pattern.to_lowercase());
        } else {
            matched = line.contains(&pattern);
        }

        i += 1;
        if line_number {
            let new_prefix = format!("{}: ", i);
            line.insert_str(0, &new_prefix);
        }

        if matched {
            println!("{}", line);
        }
        line.clear();
    }
}

fn grep_file(file: &str, config: &Config) -> () {
    match File::open(file) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            grep(&mut reader, &config.pattern, config.case_sensitivity, config.line_numbers);
        }
        Err(e) => {
            eprintln!("Could not open a file: '{}', {}", file, e);
        }
    }
}

fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    for file in &config.files {
        grep_file(&file, &config);
    }
    
    Ok(())
}

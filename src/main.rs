use std::{env, fs::File, io::{BufRead, BufReader}};

struct Config {
    pattern: String,
    files: Vec<String>,
    case_sensitivity: bool,
    line_numbers: bool,
    stripe_ws: bool,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} PATTERN FILE... [-i] [-n] [-S]", args[0]));
        }

        let mut case_sensitivity = false;
        let mut line_numbers = false;
        let mut stripe_ws = true;
        let mut non_flagged = Vec::new();

        for arg in &args[1..] {
            match arg.as_str() {
                "-i" => case_sensitivity = true,
                "-n" => line_numbers = true,
                "-S" => stripe_ws = false,
                _ => non_flagged.push(arg.clone())
            }
        }

        if non_flagged.len() < 2 {
            return Err(String::from("Error: Missing PATTERN or FILE arguments."));
        }

        let pattern = non_flagged.remove(0);
        let files = non_flagged.to_vec();

        Ok(Config { pattern, files, case_sensitivity, line_numbers, stripe_ws })

    }
}

fn grep(reader: &mut BufReader<File>, pattern: &str, case_sensitivity: bool, line_number: bool, stripe_ws: bool) {
    let mut line = String::new();
    let mut i = 0;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 { break; }
        i += 1;

        let matched = if case_sensitivity {
            line.contains(pattern)
        } else {
            line.to_lowercase().contains(&pattern.to_lowercase())
        };

        if !matched {
            line.clear();
            continue;
        }

        line = if stripe_ws {
            line.trim().to_string()
        } else {
            line.trim_end().to_string()
        }; 

        if line_number {
            let new_prefix = format!("{}: ", i);
            line.insert_str(0, &new_prefix);
        }

        println!("{}", line);
        line.clear();
    }
}

fn grep_file(file: &str, config: &Config) -> () {
    match File::open(file) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            grep(&mut reader, &config.pattern, config.case_sensitivity, config.line_numbers, config.stripe_ws);
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

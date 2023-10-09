use std::error::Error;
use std::{fs, thread};

pub struct CliArgs {
    pub file_path: String,
    pub pattern: String,
}

fn search<'a>(pattern: &str, contents: Vec<&'a str>) -> Vec<&'a str> {
    let mut matches = Vec::<&str>::new();

    for line in contents {
        if line.contains(pattern) {
            matches.push(line);
        }
    }

    matches
}

pub fn run(cli_args: CliArgs) -> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(&cli_args.file_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let c2 = contents.clone();
    let pattern = cli_args.pattern.clone();

    let mid = lines.len() / 2;
    
    let handle1 = thread::spawn(move || {
        let cloned_contents = contents.clone();
        // Clone the contents into the thread
        let cloned_lines: Vec<&str> = cloned_contents.lines().collect();
        let lines1 = cloned_lines[..mid].to_vec();
        let matches = search(&cli_args.pattern, lines1);
        for line in matches {
            println!("{}", line);
        }
    });

    let handle2 = thread::spawn(move || {
        let cloned_contents = c2.clone();
        // Clone the contents into the thread
        let cloned_lines: Vec<&str> = cloned_contents.lines().collect();
        let lines2 = cloned_lines[mid..].to_vec();
        let matches = search(&pattern, lines2);
        for line in matches {
            println!("{}", line);
        }
    });


    handle1.join().unwrap();
    handle2.join().unwrap();
    
    Ok(())
}

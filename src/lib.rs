use std::error::Error;
use std::{fs, thread};
use std::sync::mpsc;

pub struct CliArgs {
    pub file_path: String,
    pub pattern: String,
}

fn search<'a>(pattern: &str,
              contents: &'a Vec<&str>) -> Vec<&'a &'a str> {
    
    let mut matches = Vec::<&&str>::new();
    
    for line in contents {
        if line.contains(pattern) {
            matches.push(line);
        }
    }

    matches
}

pub fn run(cli_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cli_args.file_path)?;
    let num_threads = thread::available_parallelism()?.get();
    let lines = contents.lines().collect::<Vec<&str>>();
    let vec_len = lines.len();

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let mut matches = Vec::<&str>::new();
        for line in lines {
            if line.contains((&cli_args.pattern)) {
                matches.push(line);
            } 
        }
        tx.send(matches).unwrap();
    });
 
    thread::spawn(move || {
        let mut matches = Vec::<&str>::new();
        for line in lines {
            if line.contains((&cli_args.pattern)) {
                matches.push(line);
            } 
        }
        tx1.send(matches).unwrap();
    });

    //let matches = search(&cli_args.pattern, &lines);

    //for line in matches {
    //    println!("{line}");
    //}
    
    for received in rx {
        for line in received {
            println!("{line}");
        }
    }

    Ok(())
}

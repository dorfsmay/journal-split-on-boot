use std::env;
use std::fs::{rename, File};
use std::io::{self, prelude::*, BufReader, BufWriter, Write};

struct OutputWriter {
    name: String,
    writer: BufWriter<File>,
}

fn new_writer(counter: u16) -> OutputWriter {
    let name: String = format!("{:0>5}", counter);
    let ofile = File::create(&name).unwrap();
    let writer = BufWriter::new(ofile);
    OutputWriter { name, writer }
}

fn split_file(ifile: &String) -> io::Result<()> {
    let file = File::open(ifile)?;
    let reader = BufReader::new(file);

    let mut counter: u16 = 1;
    let OutputWriter {
        mut name,
        mut writer,
    } = new_writer(counter);

    let mut previous_line: String = "".to_string();
    for line_read in reader.lines() {
        let line = &line_read?;

        if line.starts_with("-- Boot ") {
            let status = if previous_line.ends_with("Journal stopped") {
                "shutdown"
            } else {
                "crashed"
            };
            rename(&name, format!("{}-{}.log", name, status))?;

            counter += 1;
            OutputWriter { name, writer } = new_writer(counter);
        }
        previous_line = line.clone();
        writer.write_all(format!("{}\n", line).as_bytes())?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            eprintln!("\nFilename required!\n");
            std::process::exit(1)
        }
        2 => split_file(&args[1]),
        _ => {
            eprintln!("\nToo many arguments!!\n");
            std::process::exit(1)
        }
    }
}

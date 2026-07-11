use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, BufWriter};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn usage() {
    eprintln!("Usage: rev [options] [<file> ...]");
    eprintln!();
    eprintln!("Reverse lines characterwise.");
    eprintln!();
    eprintln!("Options:");
    eprintln!(" -0, --zero     use the NUL byte as line separator");
    eprintln!(" -h, --help     display this help");
    eprintln!(" -V, --version  display version");
}

fn rev_line(line: &[u8]) -> Vec<u8> {
    line.iter().rev().cloned().collect()
}

fn process(reader: impl BufRead, out: &mut BufWriter<io::StdoutLock>, separator: u8) -> io::Result<()> {
    for line_result in reader.split(separator) {
        let line = line_result?;
        out.write_all(&rev_line(&line))?;
        out.write_all(b"\n")?;
    }
    Ok(())
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut files: Vec<String> = Vec::new();
    let mut zero_sep = false;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => { usage(); return 0; }
            "-V" | "--version" => { eprintln!("rev-rs {}", VERSION); return 0; }
            "-0" | "--zero" => zero_sep = true,
            "--" => {
                i += 1;
                while i < args.len() {
                    files.push(args[i].clone());
                    i += 1;
                }
                break;
            }
            _ if arg.starts_with('-') && arg.len() > 1 => {
                eprintln!("rev: invalid option -- '{}'", &arg[1..2]);
                eprintln!("Try 'rev --help' for more information.");
                return 1;
            }
            _ => files.push(arg.clone()),
        }
        i += 1;
    }

    let separator = if zero_sep { b'\0' } else { b'\n' };
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut exit_code = 0;

    if files.is_empty() {
        if let Err(e) = process(io::stdin().lock(), &mut out, separator) {
            eprintln!("rev: {}", e);
            exit_code = 1;
        }
    } else {
        for path in &files {
            let result = if path == "-" {
                process(io::stdin().lock(), &mut out, separator)
            } else {
                match File::open(path) {
                    Ok(file) => process(BufReader::new(file), &mut out, separator),
                    Err(e) => {
                        eprintln!("rev: {}: {}", path, e);
                        exit_code = 1;
                        continue;
                    }
                }
            };
            if let Err(e) = result {
                eprintln!("rev: {}: {}", path, e);
                exit_code = 1;
            }
        }
    }

    let _ = out.flush();
    exit_code
}

fn main() {
    std::process::exit(run());
}

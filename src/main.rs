use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn count_lines<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut count = 0;
    for _ in reader.lines() {
        count += 1;
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        let stdin = io::stdin();
        let line_count = count_lines(stdin.lock())?;
        println!("{line_count}");
    } else {
        for file_path in &args {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let line_count = count_lines(reader)?;
            println!("{} {}", line_count, file_path);
        }
    }
    Ok(())
}

fn count_lines<R: std::io::BufRead>(reader: R) -> std::io::Result<usize> {
    let mut count = 0;
    for _ in reader.lines() {
        count += 1;
    }
    Ok(count)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let stdin = std::io::stdin();
        let line_count = count_lines(stdin.lock())?;
        println!("{line_count}");
    } else {
        for file_path in &args {
            let file = std::fs::File::open(file_path)?;
            let reader = std::io::BufReader::new(file);
            let line_count = count_lines(reader)?;
            println!("{} {}", line_count, file_path);
        }
    }
    Ok(())
}

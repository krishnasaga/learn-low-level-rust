use std::fs::metadata;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let size = metadata("numbers.txt")?.len();
    let mut numbers: Vec<i8> = Vec::with_capacity(size.try_into().unwrap());

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    loop {
        let line = lines.next();
        match line {
            Some(result) => match result {
                Ok(r) => {
                    numbers.push(r.clone().parse().unwrap());
                }
                Err(e) => print!("{}", e),
            },
            None => break,
        }
    }

    let total: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x as i32 * 100)
        .reduce(|acc: i32, c| acc + c as i32)
        .expect("R");

    print!("{}\n", total);

    Ok(())
}

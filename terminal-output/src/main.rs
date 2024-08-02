use std::io::{self, Write};

fn main() -> io::Result<()> {
    //Outputing UT8-encoded :) smile emoji
    let _ = io::stdout().write(&[0x01, 0xF6, 0x0A]);

    //Colors
    let _ = io::stdout().write(&[
        27, 91, 49, 59, 51, 49, 109, 84, 104, 105, 115, 32, 105, 115, 32, 98, 111, 108, 100, 32,
        114, 101, 100, 32, 116, 101, 120, 116, 27, 91, 48, 109,
    ]);

    print!("\n");

    Ok(())
}

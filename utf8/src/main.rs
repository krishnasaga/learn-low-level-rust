use std::fs::File;
use std::io::Read;
use std::io::{self, BufReader};

const UTF_SEQUENCE_BIT_MASK: u8 = 0b00111111;

fn main() -> io::Result<()> {
    let _ = to_code_points("chars.txt");
    Ok(())
}

fn to_code_points(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut bytes = [0; 1];

    let mut unicode_chars = Vec::<u32>::with_capacity(1000);
    let mut byte_sequence = Vec::<u8>::with_capacity(1000);

    loop {
        let _ = reader.read_exact(&mut bytes);
        let byte = bytes[0];
        let flag = byte & 0b11110000;
        byte_sequence.push(byte);
        if flag < 0x80 {
            unicode_chars.push((byte as u32).into());
        } else if flag >= 0x80 {
            if flag == 0b11000000 {
                let mut bb = [0; 1];
                let _ = reader.read_exact(&mut bb);
                let mut code_point: u32 = 0;
                code_point |= (byte & 0b00011111) as u32;
                code_point = (code_point << 6) | ((bb[0] & UTF_SEQUENCE_BIT_MASK) as u32);

                unicode_chars.push(code_point.into());
                for b in bb {
                    byte_sequence.push(b)
                }
            }
            if flag == 0b11100000 {
                let mut bb = [0; 2];
                let _ = reader.read_exact(&mut bb);

                let mut code_point: u32 = 0;
                code_point |= (byte & 0b00001111) as u32;
                code_point = (code_point << 6) | ((bb[0] & UTF_SEQUENCE_BIT_MASK) as u32);
                code_point = (code_point << 6) | ((bb[1] & UTF_SEQUENCE_BIT_MASK) as u32);

                unicode_chars.push(code_point.into());
                for b in bb {
                    byte_sequence.push(b)
                }
            }
            if flag == 0b11110000 {
                let mut bb = [0; 3];
                let _ = reader.read_exact(&mut bb);

                let mut code_point: u32 = 0;
                code_point |= (byte & 0b00000111) as u32;
                code_point = (code_point << 6) | ((bb[0] & UTF_SEQUENCE_BIT_MASK) as u32);
                code_point = (code_point << 6) | ((bb[1] & UTF_SEQUENCE_BIT_MASK) as u32);
                code_point = (code_point << 6) | ((bb[2] & UTF_SEQUENCE_BIT_MASK) as u32);
                unicode_chars.push(code_point.into());
                for b in bb {
                    byte_sequence.push(b)
                }
            }
        }
        if bytes[0] == 10 {
            break;
        }
    }
    for code_point in unicode_chars {
        print!("{:X} ", code_point);
    }

    print!("\n");
    for byte in byte_sequence {
        print!("{:X} ", byte);
    }
    print!("\n");

    Ok(())
}

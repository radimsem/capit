use std::{
    env,
    fs::{self, File},
    io::Write,
};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!("file path not defined in arguments list");
    }

    let path = args.get(1).unwrap();

    let content = fs::read_to_string(path)?;
    let mut file = File::create(path)?;

    let mut lines: Vec<&str> = content.lines().collect();
    let mut offset = 0;

    for (i, line) in lines.iter().rev().enumerate() {
        let idx = (lines.len() - 1) - i;

        if !line.is_empty() || idx == 1 {
            offset = idx;
            break;
        }

        if idx == 0 {
            break;
        }
        let prev = lines.get(idx - 1).unwrap();

        if !prev.is_empty() {
            offset = idx - 2;
            break;
        }
    }

    lines = lines[0..offset].to_vec();
    file.write_all(lines.join("\n").as_bytes())?;

    Ok(())
}

use anyhow::Result;
use aoc::io::read_stdin;
use reqwest::blocking::ClientBuilder;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let cookie_input = read_stdin()?;
    let cookie = format!("session={}", cookie_input.trim());
    let client = ClientBuilder::new()
        .user_agent("rust/reqwest/kaaveland@gmail.com")
        .build()?;

    for day in 1..=25 {
        let folder = format!("./input/day_{day:0>2}");
        let dest = format!("{folder}/input");
        fs::create_dir_all(folder)?;
        if !Path::new(dest.as_str()).exists() {
            let body = client
                .get(format!("https://adventofcode.com/2022/day/{day}/input"))
                .header("Cookie", cookie.as_str())
                .send()?
                .text()?;
            let mut fp = File::create(dest)?;
            fp.write_all(body.as_bytes())?;
        }
    }
    Ok(())
}

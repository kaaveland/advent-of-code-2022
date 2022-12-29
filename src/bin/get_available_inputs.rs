use anyhow::Result;
use aoc::io::read_stdin;
use reqwest::blocking::ClientBuilder;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

fn obtain_user_agent() -> Result<String> {
    println!("Enter a user agent string containing a way to contact you: ");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn obtain_session_cookie() -> Result<String> {
    println!("Enter your session cookie");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().replace("session=", "").replace('"', ""))
}

fn main() -> Result<()> {
    let cookie_input = obtain_session_cookie()?;
    let user_agent = obtain_user_agent()?;
    let cookie = format!("session={}", cookie_input.trim());
    let client = ClientBuilder::new().user_agent(user_agent).build()?;

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

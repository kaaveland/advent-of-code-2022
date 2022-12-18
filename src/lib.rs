pub mod io {
    use anyhow::Result;
    use std::io::{stdin, Read};

    pub fn read_stdin() -> Result<String> {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        Ok(buf)
    }
}

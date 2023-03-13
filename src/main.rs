use generators::generate::generate;
use tools::{consts::{setup_constants}, window::setup_window};
use std::io::{stdin, stdout, Read, Write};

mod tools;
mod generators;
mod point;


fn main_run() -> anyhow::Result<()> {
    setup_constants();

    let mut window = setup_window()?;
    generate(&mut window);

    Ok(())
}


pub fn main() {
    let res = main_run();
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
    } else {
        println!("Done.");
    }

    pause();
}


fn pause()
{
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}
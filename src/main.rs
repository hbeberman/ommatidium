use ommatidium::error::OmmaErr;
use ommatidium::term::OmmaTerm;

fn hello() -> Result<(), OmmaErr> {
    let mut term = OmmaTerm::new()?;
    println!("Terminal {}", term);
    term.move_cursor(1, 1)?;
    for y in 0..=24 {
        for x in 0..=79 {
            term.put_char_at(y, x, '#')?;
        }
    }
    Ok(())
}

fn main() {
    match hello() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}")
        }
    }
}

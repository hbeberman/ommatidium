use ommatidium::error::OmmaErr;
use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;
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
    let mut session = Session::new()?;
    let plane_id = session.new_plane()?;
    let window_id = session.new_window(plane_id, 0, 0, 10, 10)?;
    session
        .set_ommacell(
            window_id,
            3,
            3,
            OmmaCell {
                ch: '@',
                fg: 0,
                bg: 0,
                attrs: 0,
            },
        )
        .unwrap();
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

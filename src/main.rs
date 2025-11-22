use ommatidium::error::OmmaErr;
use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;

fn hello() -> Result<(), OmmaErr> {
    let wall = OmmaCell {
        ch: '#',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let mut session = Session::new()?;
    let plane_id = session.new_plane()?;
    let window_id = session.new_window(plane_id, 0, 0, 10, 10)?;
    session.fill_window(window_id, &wall)?;

    let window_id2 = session.new_window(plane_id, 15, 0, 5, 5)?;
    session.fill_window(window_id2, &wall)?;
    session.set_ommacell(
        window_id,
        3,
        3,
        OmmaCell {
            ch: '@',
            fg: 0,
            bg: 0,
            attrs: 0,
        },
    )?;
    session.render()?;
    loop {
        if let Some(key) = session.read_key()? {
            match key {
                'q' => return Ok(()),
                'Q' => return Ok(()),
                '\x03' => return Ok(()),
                _ => continue,
            }
        }
    }
}

fn main() {
    match hello() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}")
        }
    }
}

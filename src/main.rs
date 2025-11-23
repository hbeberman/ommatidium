use ommatidium::error::OmmaErr;
use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;

fn hello() -> Result<(), OmmaErr> {
    let floor = OmmaCell {
        ch: '.',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let wall = OmmaCell {
        ch: '#',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let horiz = OmmaCell {
        ch: '-',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let vert = OmmaCell {
        ch: '|',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let corner = OmmaCell {
        ch: 'O',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let special = OmmaCell {
        ch: '*',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let blank = OmmaCell {
        ch: ' ',
        fg: 0,
        bg: 0,
        attrs: 0,
    };
    let mut session = Session::new()?;
    let plane_id = session.new_plane()?;
    let window_id = session.new_window(plane_id, 0, 0, 10, 10)?;
    session.fill_window(window_id, &floor)?;
    session.set_window_border(window_id, vec![&wall])?;

    let window_id2 = session.new_window(plane_id, 15, 0, 5, 5)?;
    session.fill_window(window_id2, &wall)?;

    let window_id3 = session.new_window(plane_id, 5, 5, 10, 10)?;
    session.set_window_border(window_id3, vec![&horiz, &vert, &corner])?;

    let window_id4 = session.new_window(plane_id, 10, 3, 8, 4)?;
    session.fill_window(window_id4, &blank)?;
    session.set_window_border(window_id4, vec![&special])?;

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
                'S' => return Ok(()),
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

use ommatidium::error::OmmaErr;
use ommatidium::ommacell::OmmaCell;
use ommatidium::session::Session;
use ommatidium::window::WindowBuilder;

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
    let (id_world, window) = WindowBuilder::new(10, 10).plane(plane_id, 0, 0).build()?;
    session.add_window(window)?;
    session.fill_window(id_world, &floor)?;
    session.set_window_border(id_world, vec![&wall])?;

    let (id, window) = WindowBuilder::new(5, 5).plane(plane_id, 15, 0).build()?;
    session.add_window(window)?;
    session.fill_window(id, &wall)?;

    let (id, window) = WindowBuilder::new(10, 10).plane(plane_id, 5, 5).build()?;
    session.add_window(window)?;
    session.set_window_border(id, vec![&horiz, &vert, &corner])?;

    let (id, window) = WindowBuilder::new(20, 4).plane(plane_id, 11, 3).build()?;
    session.add_window(window)?;
    session.fill_window(id, &blank)?;
    session.set_window_border(id, vec![&special])?;
    session.window_string_raw(id, 1, 0, &wall, "Hello Dungeon!".to_string())?;
    session.window_string_raw(id, 1, 1, &wall, "Yes!".to_string())?;
    session.window_string_raw(id, 8, 1, &wall, "No!".to_string())?;

    session.set_ommacell(
        id_world,
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

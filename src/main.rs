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
    let player = OmmaCell {
        ch: '@',
        fg: 0,
        bg: 0,
        attrs: 0,
    };

    let mut session = Session::new()?;

    session
        .new_window(30, 30)
        .name("Backdrop".to_string())
        .fill(&blank)
        .submit(&mut session)?;

    let id_world = session
        .new_window(10, 10)
        .name("World".to_string())
        .fill(&floor)
        .submit(&mut session)?;
    let window = session.window(id_world)?;
    window.set_border(vec![&wall])?;

    session
        .new_window(5, 5)
        .offset(15, 0)
        .fill(&wall)
        .submit(&mut session)?;
    let id = session
        .new_window(10, 10)
        .offset(5, 5)
        .submit(&mut session)?;
    let window = session.window(id)?;
    window.set_border(vec![&horiz, &vert, &corner])?;

    let id = session
        .new_window(20, 4)
        .offset(11, 3)
        .fill(&blank)
        .submit(&mut session)?;

    let window = session.window(id)?;
    window.set_border(vec![&special])?;
    window.string_raw(1, 0, &wall, "Hello Dungeon!".to_string())?;
    window.string_raw(1, 1, &wall, "Yes!".to_string())?;
    window.string_raw(8, 1, &wall, "No!".to_string())?;

    let id_block = session
        .new_window(20, 20)
        .offset(0, 0)
        .fill(&wall)
        .hidden()
        .submit(&mut session)?;

    let id_temp = session
        .new_window(5, 5)
        .offset(10, 3)
        .parent(id_block)
        .fill(&floor)
        .submit(&mut session)?;

    let id_player = session
        .new_window(1, 1)
        .offset(3, 3)
        .parent(id_temp)
        .fill(&player)
        .submit(&mut session)?;

    session.window(id_world)?.set_ommacell(3, 3, &player)?;

    loop {
        session.render()?;
        if let Some(key) = session.read_key()? {
            match key {
                'S' => break,
                'p' => session.window(id_player)?.toggle_hidden(),
                'h' => session.window(id_block)?.toggle_hidden(),
                't' => session.window(id_temp)?.toggle_hidden(),
                '\x03' => break,
                _ => continue,
            }
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

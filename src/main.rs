use ommatidium::cell::*;
use ommatidium::error::OmmaErr;
use ommatidium::session::Session;

fn hello() -> Result<(), OmmaErr> {
    let mut session = Session::new()?;

    // TODO: Remove the need for this backdrop
    session
        .new_window(40, 30)
        .name("Backdrop".to_string())
        .fill(&BLANK_CELL)
        .submit(&mut session)?;

    let id_world = session
        .new_window(10, 10)
        .name("World".to_string())
        .fill(&FLOOR_CELL)
        .pad_mono(1)
        .border_mono(&WALL_CELL)
        .submit(&mut session)?;

    session
        .new_window(5, 5)
        .offset(15, 0)
        .fill(&WALL_CELL)
        .submit(&mut session)?;

    let id_transparent = session
        .new_window(10, 10)
        .offset(5, 5)
        .border_raw(&HORIZ_CELL, &VERT_CELL, &CORNER_CELL)
        .submit(&mut session)?;

    let id_dialog = session
        .new_window(25, 9)
        .offset(11, 3)
        .fill(&BLANK_CELL)
        .fill(&FLOOR_CELL)
        .border_mono(&SPECIAL_CELL)
        .pad_mono(2)
        .submit(&mut session)?;
    let window = session.window(id_dialog)?;
    window.string_raw(0, 0, &WALL_CELL, "Hello Dungeon!".to_string())?;
    window.string_raw(0, 1, &WALL_CELL, "Yes!".to_string())?;
    window.string_raw(7, 1, &WALL_CELL, "No!".to_string())?;

    let id_block = session
        .new_window(20, 20)
        .offset(0, 0)
        .fill(&WALL_CELL)
        .hidden()
        .submit(&mut session)?;

    let id_temp = session
        .new_window(5, 5)
        .offset(10, 3)
        .parent(id_block)
        .fill(&FLOOR_CELL)
        .submit(&mut session)?;

    let id_player = session
        .new_window(1, 1)
        .offset(3, 3)
        .parent(id_temp)
        .fill(&PLAYER_CELL)
        .submit(&mut session)?;

    session.window(id_world)?.set_ommacell(3, 3, &PLAYER_CELL)?;

    loop {
        session.render()?;
        if let Some(key) = session.read_key()? {
            match key {
                'S' => break,
                'p' => session.window(id_player)?.toggle_hidden(),
                'h' => session.window(id_block)?.toggle_hidden(),
                't' => session.window(id_temp)?.toggle_hidden(),
                'q' => session.window(id_world)?.toggle_border_hidden(),
                'w' => session.window(id_transparent)?.toggle_border_hidden(),
                'e' => session.window(id_dialog)?.toggle_border_hidden(),
                'z' => session.window(id_dialog)?.remove_border(),
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

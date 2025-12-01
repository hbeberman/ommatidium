use ommatidium::cell::*;
use ommatidium::color::{BLACK, BLUE, DARK_GREY, GREEN, RED};
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

    let floor = OmmaCell {
        fg: Some(DARK_GREY),
        ..FLOOR_CELL
    };

    let id_world = session
        .new_window(10, 10)
        .name("World".to_string())
        .fill(&floor)
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

    let positive = OmmaCell {
        fg: Some(GREEN),
        ..DEFAULT_CELL
    };
    let negative = OmmaCell {
        fg: Some(RED),
        ..DEFAULT_CELL
    };
    let blank_black = OmmaCell {
        bg: Some(BLACK),
        ..BLANK_CELL
    };
    let id_dialog = session
        .new_window(25, 9)
        .offset(11, 3)
        .fill(&blank_black)
        .border_mono(&SPECIAL_CELL)
        .pad_mono(2)
        .submit(&mut session)?;
    let window = session.window(id_dialog)?;
    window.string_raw(0, 0, &BLANK_CELL, "Hello Dungeon!".to_string())?;
    window.string_raw(0, 1, &positive, "Yes!".to_string())?;
    window.string_raw(7, 1, &negative, "No!".to_string())?;

    let id_zone = session
        .new_window(20, 20)
        .offset(0, 0)
        .fill(&floor)
        .border_mono(&WALL_CELL)
        .submit(&mut session)?;

    let player = OmmaCell {
        fg: Some(BLUE),
        ..PLAYER_CELL
    };
    let goblin = OmmaCell {
        ch: 'g',
        fg: Some(GREEN),
        ..DEFAULT_CELL
    };

    let id_player = session
        .new_object()
        .name("Player".to_string())
        .offset(3, 2)
        .parent(id_zone)
        .cell(&player)
        .submit(&mut session)?;

    let id_goblin = session
        .new_object()
        .name("Goblin 1".to_string())
        .offset(1, 1)
        .parent(id_zone)
        .cell(&goblin)
        .submit(&mut session)?;

    loop {
        session.render()?;
        if let Some(key) = session.read_key()? {
            match key {
                'S' => break,
                'p' => session.object(id_player)?.toggle_hidden(),
                'h' => session.window(id_dialog)?.toggle_hidden(),
                'q' => session.window(id_world)?.toggle_border_hidden(),
                'w' => session.window(id_transparent)?.toggle_border_hidden(),
                'e' => session.window(id_dialog)?.toggle_border_hidden(),
                'z' => session.window(id_zone)?.toggle_hidden(),
                'g' => session.object(id_goblin)?.toggle_hidden(),
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

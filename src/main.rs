use ommatidium::term;

fn main() {
    let terminfo = match term::terminfo() {
        Ok(terminfo) => terminfo,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    println!("Terminal is {}", terminfo);
}

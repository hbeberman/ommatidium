use ommatidium::ommacell::OmmaCell;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ommacell = OmmaCell::new(' ', 0, 0, 0);
        assert!(ommacell.ch == ' ');
    }
}

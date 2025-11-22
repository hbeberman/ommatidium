use ommatidium::session::Session;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let session = Session::new();
        let session2 = Session::new();
        let id = session.id();
        eprintln!("ID == {}", id);
        let id = session2.id();
        eprintln!("ID == {}", id);
        assert!(session.id() == 1);
        assert!(session2.id() == 2);
        assert!(session.planes_is_empty());
    }
}

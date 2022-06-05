crate::derive_wrapper!(
    pub struct Lobby(String);
);

impl Lobby {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        assert!(s.len() == 7, "lobby string must be 7 characters");
        Self(s)
    }

    pub fn new_random() -> Self {}
}

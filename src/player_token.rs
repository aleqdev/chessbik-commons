chessbik_derive_wrapper::derive_wrapper!(
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerToken(String);
);

impl PlayerToken {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        assert!(s.len() == 14, "player token string must be 14 characters");
        Self(s)
    }
}

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(
        Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
    )]
    pub struct OpponentName(pub String);
);

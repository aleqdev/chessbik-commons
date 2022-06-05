pub use derive_more;

#[macro_export]
macro_rules! derive_wrapper {
    ($i:item) => {
        #[derive(
            chessbik_commons::derive_more::Deref,
            chessbik_commons::derive_more::DerefMut,
            chessbik_commons::derive_more::AsRef,
            chessbik_commons::derive_more::From,
            chessbik_commons::derive_more::Into,
        )]
        $i
    };
}

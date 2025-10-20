pub trait Named {
    fn name() -> &'static str;
}

pub use tynamed_macros::Named;

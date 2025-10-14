pub trait Named {
    fn name() -> &'static str;
}

pub use named_derive::Named;

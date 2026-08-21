pub use definition::IdType;

#[cfg(feature = "serde")]
mod definition {
    pub trait IdType: Clone + serde::de::DeserializeOwned {}
    impl<T> IdType for T where T: Clone + serde::de::DeserializeOwned {}
}

#[cfg(not(feature = "serde"))]
mod definition {
    pub trait IdType: Clone {}
    impl<T> IdType for T where T: Clone {}
}

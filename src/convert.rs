pub trait Convert<T> : Sized {
    type Error;
    fn convert(value : T) -> Result<Self, Self::Error>;
}
pub trait Event<T> {
    fn generate<R>(&self) -> R;
}

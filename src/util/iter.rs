pub trait Stream {
    type Item;

    fn next(&mut self) -> impl std::future::Future<Output = Option<Self::Item>> + Send;
}

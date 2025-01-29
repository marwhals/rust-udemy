pub trait Container<T> {
    //leave abstract. The user of the trait will define the implementation. Same signature, different implementation
    fn get(&mut self) -> Option<T>;
    fn put(&mut self, item: T);
    fn is_empty(&self) -> bool;
}

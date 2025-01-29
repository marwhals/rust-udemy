use super::container::Container;

pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }
}
impl<T> Container<T> for Basket<T> {
    // pub fn new(item: T) -> Self {
    //     Basket { item: Some(item) }
    // }
//Inherited trait methods are public by default
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}


//-----Non generic
// pub struct Basket {
//     item: Option<String>,
// }
//
// impl Basket {
//     pub fn new(item: String) -> Self {
//         Basket { item: Some(item) }
//     }
//
//     pub fn get(&mut self) -> Option<String> {
//         self.item.take()
//     }
//
//     pub fn put(&mut self, item: String) {
//         self.item = Some(item);
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.item.is_none()
//     }
// }

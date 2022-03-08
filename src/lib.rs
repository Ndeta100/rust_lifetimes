struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        //get the first element
        let element = self.slice.get(0);
        //setn  self.slice= the other elements
        self.slice = &self.slice[1..];
        //return first element
        element
    }
}
struct MyMutableIterator<'a, T> {
    slice: &'a mut [T],
}
impl<'a, T> Iterator for MyMutableIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collection = vec![1, 2, 3, 4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };
        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        //for mutable iterator
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }
        assert_eq!(collection.get(0), Some(&2));
    }
}

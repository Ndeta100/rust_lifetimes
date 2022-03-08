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
struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}
impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)

        // //get the first element
        // let element = self.slice.get_mut(0);

        // //self.slice to the rest of the list
        // self.slice = &mut self.slice[1..];
        // //return the first element
        // element
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

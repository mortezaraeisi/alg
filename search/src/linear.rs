pub fn find<T>(vec: &Vec<T>, item: &T) -> Option<usize>
    where T: Ord {
    for (i, x) in vec.iter().enumerate() {
        let found = x == item;
        if found {
            return Some(i);
        }
    }
    None
}
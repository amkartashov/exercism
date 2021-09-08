pub fn find(array: &[i32], key: i32) -> Option<usize> {
    return find_rec(array, key, 0);

    fn find_rec(array: &[i32], key: i32, pos: usize) -> Option<usize> {
        let len = array.len();
        match len {
            0 => None,
            1 => {
                if array[0] == key {
                    Some(pos)
                } else {
                    None
                }
            }
            x => {
                let split_idx = x / 2;
                let new_pos = pos + split_idx;
                let mid_elem = array[split_idx];
                let (small, big) = array.split_at(split_idx);
                match mid_elem.cmp(&key) {
                    std::cmp::Ordering::Equal => Some(new_pos),
                    std::cmp::Ordering::Greater => find_rec(small, key, pos),
                    std::cmp::Ordering::Less => find_rec(big, key, new_pos),
                }
            }
        }
    }
}

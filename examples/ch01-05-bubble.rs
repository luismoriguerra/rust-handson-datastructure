pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![4, 2, 3, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}

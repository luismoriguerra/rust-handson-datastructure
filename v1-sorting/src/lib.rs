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

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
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

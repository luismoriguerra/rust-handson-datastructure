use std::fmt::Debug;

// Function to find the pivot index in the array
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    // Initialize the pivot index to 0
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    // Loop through the array starting from the second element
    for i in 1..v.len() {
        // If the current element is less than the pivot element
        if v[i] < v[p] {
            // Swap the element after the pivot with the current element
            v.swap(p + 1, i);
            // Swap the pivot element with the element after it
            v.swap(p, p + 1);
            // Move the pivot index to the right
            p += 1;
        }
    }

    // Return the pivot index
    p
}

pub fn threaded_quick_sort<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    rayon::join(
        || threaded_quick_sort(a),
        || threaded_quick_sort(&mut b[1..]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        let _p = threaded_quick_sort(&mut v);

        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }
}

fn main() {}

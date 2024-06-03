// Import necessary traits for comparison and debugging
use std::fmt::Debug;

// Define a public function `merge_sort` that sorts a vector of elements that implement `PartialOrd` and `Debug`
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // Base case: if the vector has 0 or 1 element, it is already sorted, so return it
    println!("v: {:?}", v);
    if v.len() <= 1 {
        return v;
    }

    // Create a new vector `res` with the same capacity as the input vector `v`
    let mut merged_sorted_collection = Vec::with_capacity(v.len());

    // Split the vector `v` into two halves and assign the second half to `b`
    let unsorted_b_part = v.split_off(v.len() / 2);

    // Recursively sort the first half `v` (now reduced after split) and the second half `b`
    let sorted_a = merge_sort(v);
    let sorted_b = merge_sort(unsorted_b_part);

    // Create iterators for the sorted halves `a` and `b`
    let mut sorted_a_collection = sorted_a.into_iter();
    let mut sorted_b_collection = sorted_b.into_iter();

    // Initialize peekable elements from both iterators
    let mut a_first_item = sorted_a_collection.next();
    let mut b_first_item = sorted_b_collection.next();

    // Merge the two sorted halves into `res`
    loop {
        match a_first_item {
            Some(ref a_val) => {
                match b_first_item {
                    Some(ref b_val) => {
                        // Compare the current elements of both iterators and push the smaller one into `res`
                        if b_val < a_val {
                            // take will remove the value from the Option, leaving a None in its place
                            merged_sorted_collection.push(b_first_item.take().unwrap()); // Move `b_val` into `res`
                            b_first_item = sorted_b_collection.next(); // Advance the iterator `b_it`
                        } else {
                            merged_sorted_collection.push(a_first_item.take().unwrap()); // Move `a_val` into `res`
                            a_first_item = sorted_a_collection.next(); // Advance the iterator `a_it`
                        }
                    }
                    None => {
                        // If `b_it` is exhausted, push the remaining elements of `a_it` into `res`
                        merged_sorted_collection.push(a_first_item.take().unwrap());
                        merged_sorted_collection.extend(sorted_a_collection);
                        println!("b res: {:?}", merged_sorted_collection);
                        return merged_sorted_collection;
                    }
                }
            }
            None => {
                // If `a_it` is exhausted, push the remaining elements of `b_it` into `res`
                if let Some(b_val) = b_first_item {
                    merged_sorted_collection.push(b_val);
                }

                merged_sorted_collection.extend(sorted_b_collection);
                println!("a res: {:?}", merged_sorted_collection);
                return merged_sorted_collection;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![
            99, 15, 30, 45, 200, 1, 5, 10, 14, 78, 68, 25, 77, 11, 46, 49,
        ];
        let v = merge_sort(v);
        assert_eq!(
            v,
            vec![1, 5, 10, 11, 14, 15, 25, 30, 45, 46, 49, 68, 77, 78, 99, 200]
        );
    }
}

fn main() {}

use super::Sorter;

// does not allocate memory
// smallest of remainder will be largest in sorted.
pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        },
        _ => {},
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on correct side
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                // done
                break;
            }
            // avoid unnecessary swaps back and forth
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                // done
                break;
            }
            right -= 1;
        }
    }

    let left = left + 1;
    // put pivot in proper place in left sorted items
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord
    {
        quicksort(slice)
    }
}

#[test]
fn quick_works() {
    let mut things = vec![4,2,3,1,5];
    QuickSort.sort(&mut things);
    assert_eq!(things, [1,2,3,4,5]);
}

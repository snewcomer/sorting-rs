use super::Sorter;

// O(n) or O(n^2) worst case
pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubble_works() {
    let mut things = vec![4,2,3,1,5];
    // super::sort::<_, Bubblesort>(&mut things);
    Bubblesort.sort(&mut things);
    assert_eq!(things, [1,2,3,4,5]);
}

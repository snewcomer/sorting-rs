use super::Sorter;

// faster if small lists O(n^2) worst case
pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted - 1;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);

                i -= 1;
            }
        }
    }
}

#[test]
fn insertion_works() {
    let mut things = vec![4,2,3,1,5];
    // super::sort::<_, Insertionsort>(&mut things);
    InsertionSort.sort(&mut things);
    assert_eq!(things, [1,2,3,4,5]);
}

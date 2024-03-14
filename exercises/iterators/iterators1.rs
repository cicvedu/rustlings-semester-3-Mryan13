// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: Create an immutable iterator over the vector.
    let mut my_iterable_fav_fruits = my_fav_fruits.iter(); 

    // First call to next() returns the first element
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    // Step 2: The next call should return the second element
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    // Continues to return the next element
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: The next call should return the fourth element
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    // The last fruit in the vector
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: Once all elements are consumed, it should return None
    assert_eq!(my_iterable_fav_fruits.next(), None);
}


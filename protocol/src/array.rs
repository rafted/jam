/// A struct to store a length and an array of elements.
///
/// # Examples
///
/// ```rs
/// let ca = CountedArray {
///     len: 5,
///     arr: vec![1, 2, 3, 4, 5],
/// };
///
/// assert_eq!(ca.len, 5);
/// assert_eq!(ca.arr, [1, 2, 3, 4, 5]);
/// ```
pub struct CountedArray<A, B> {
    pub len: A,
    pub arr: Vec<B>,
}
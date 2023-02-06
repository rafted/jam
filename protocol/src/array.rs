// Array which we know the size of, so we read x amount of times
// A: The type of the length/count/size of the array
// B: The type of the array
pub struct CountedArray<A, B> {
    pub len: A,
    pub arr: Vec<B>,
}

// Array we don't know the count of, so we just read as much as we can from the packet
// this occurs in places where the only field in the packet is the array
pub struct DynamicArray<A> {
    pub arr: Vec<A>,
}

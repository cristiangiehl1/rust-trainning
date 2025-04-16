// String is mutable.
// String size can change at runtime.
// String stored on the stack with a pointer to the heap.
// Value of String is stored on the heap.

// let s1 = String::from("Hello");

// Stack Memory - s1
// name     | value
// ptr      | address to the heap
// len      |   5
// capacity |   5

// ptr: Pointer to data stored on the heap.
// len: Number of bytes used to store the string.
// capacity: Total amount of memory received from the allocator.
// Size of s1 will be 24 bytes. 3 * 8 bytes for the pointer, length and capacity.

// Heap Memory
// index    |  value
// 0        |  H
// 1        |  e
// 2        |  l
// 3        |  l
// 4        |  o

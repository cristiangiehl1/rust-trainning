// Component in a computer to store data and instructions for the processor to execute.
// Random Access Memory (RAM) is volatile, when power turned off, data is lost.
// Two types of regions in RAM used by a program at runtime: Stack and Heap.

// Stack Memory (stack of plates):
// - Stack memory is a region of memory that stores data in a last-in, first-out (LIFO) order.
// - Stack data must have a known size, fixed size (like integers, floats, char, bool, etc...).
// - Stack memory is faster than heap memory because it is managed by the CPU.
// - Pushing and popping data from the stack is faster than allocating and deallocating memory on the heap, because the location for
// new data is always at the top of the stack.
// - Stack memory is used for static memory allocation.

// Heao Memory:
// - Types of unknown size (like vectors, strings, etc...) will allocated to the heap.
// - Allocating data on the heap will return a pointer (an address to location where data has been alllocated)
// because a pointer is fixed size (usize).
// - Allocating on the heap is slower than pushing to stack.
// - Accessing data on the heap is also slower, as it has to be accessed usign a pointer which points to an address.
// - Heap memory is a region of memory that stores data in a more flexible way.
//

Bubble Sort Algorithm

This repository contains a Rust implementation of the Bubble Sort algorithm. Bubble Sort is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares adjacent elements and swaps them if they are in the wrong order.

Code Description

The main function in this code is bubble_sort, which takes a mutable reference to a slice of i32 integers as input. The function then iterates over the slice using two nested loops, comparing adjacent elements and swapping them if they are in the wrong order.

The first loop iterates over the slice from index 0 to the second-to-last element. The second loop iterates over the slice from index 0 to n-i-1, where n is the length of the slice and i is the index of the outer loop. This ensures that the inner loop only needs to compare elements that have not already been sorted in previous iterations of the outer loop.

If the element at index j is greater than the element at index j+1, the swap method of the slice is used to swap the two elements.

Interesting Feature

One interesting feature of this implementation is the use of the swap method to swap two elements in the slice. The swap method is a convenient and efficient way to swap elements in a slice without needing to use temporary variables.

Here is an example of how the swap method can be used to swap two elements in a slice:

let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
arr.swap(1, 3);
println!("{:?}", arr); // Output: [3, 1, 1, 4, 5, 9, 2, 6, 5, 3, 5]

In this example, the swap method is used to swap the second and fourth elements of the array arr, resulting in the output [3, 1, 1, 4, 5, 9, 2, 6, 5, 3, 5]. This technique can be very useful in many different algorithms and data structures.

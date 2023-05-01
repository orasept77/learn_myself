Bubble Sort Algorithm
This repository contains a Rust implementation of the [Bubble Sort](./src/bubble_sort.rs) algorithm. Bubble Sort is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares adjacent elements and swaps them if they are in the wrong order.


Code Description   
The main function in this code is bubble_sort, which takes a mutable reference to a slice of i32 integers as input. The function then iterates over the slice using two nested loops, comparing adjacent elements and swapping them if they are in the wrong order.


The first loop iterates over the slice from index 0 to the second-to-last element. The second loop iterates over the slice from index 0 to n-i-1, where n is the length of the slice and i is the index of the outer loop. This ensures that the inner loop only needs to compare elements that have not already been sorted in previous iterations of the outer loop.

If the element at index j is greater than the element at index j+1, the swap method of the slice is used to swap the two elements.

Interesting Feature
One interesting feature of this implementation is the use of the swap method to swap two elements in the slice. The swap method is a convenient and efficient way to swap elements in a slice without needing to use temporary variables.


Here is an example of how the swap method can be used to swap two elements in a slice: 
```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
arr.swap(1, 3);
println!("{:?}", arr); // Output: [3, 1, 1, 4, 5, 9, 2, 6, 5, 3, 5]
```

In this example, the swap method is used to swap the second and fourth elements of the array arr, resulting in the output [3, 1, 1, 4, 5, 9, 2, 6, 5, 3, 5]. This technique can be very useful in many different algorithms and data structures.

________________________________________________________________________________________________________________________________________________

Insertion Sort Algorithm
This repository contains a Rust implementation of [Insertion Sort.rs](./src/insertion_sort.rs), which is a comparison-based sorting algorithm that builds the final sorted array one item at a time.


Code Description
The main function in this code is insertion_sort, which takes a mutable reference to a slice of i32 integers as input. The function then iterates over the slice using a loop that starts at index 1 and ends at the second-to-last element.


For each iteration of the loop, the value at the current index is stored in a variable called key, and a second variable called j is set to the index of the previous element. The loop then continues as long as j is greater than or equal to 0 and the value at index j is greater than key.


During each iteration of the loop, the value at index j is moved one position to the right in the array to make room for the key value. This is done by setting the value at index j+1 to the value at index j, and then decrementing j.


After the loop has completed, the key value is inserted into its correct position in the sorted array by setting the value at index j+1 to key.


Interesting Feature
One interesting feature of this implementation is the use of the j variable to iterate backwards through the sorted portion of the array. This allows the algorithm to efficiently find the correct position for the key value in the sorted array.

Here is an example of how the j variable is used in this implementation:
```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
let key = 5;
let mut j = 4;

while j >= 0 && arr[j] > key {
    arr[j + 1] = arr[j];
    j -= 1;
}

arr[j + 1] = key;
println!("{:?}", arr); // Output: [3, 1, 4, 1, 5, 5, 9, 2, 6, 5, 3]
```
In this example, the j variable is initialized to the index of the last element in the sorted portion of the array that is less than the key value. The loop then iterates backwards through the sorted portion of the array, moving each element one position to the right until it finds the correct position for the key value. Finally, the key value is inserted into the correct position in the array using the expression arr[j+1] = key. This technique can be very useful in many different algorithms and data structures.

______________________________________________________________________________________________________________________________________________

Quick Sort Algorithm
This repository contains a Rust implementation of the [Quick Sort](./src/quick_sort.rs) algorithm. Quick Sort is a divide-and-conquer algorithm that uses a pivot element to partition an array into two sub-arrays, and then recursively sorts each sub-array.


Code Description
The main function in this code is quick_sort, which takes a mutable reference to a slice of i32 integers as input, along with the indices low and high that represent the lower and upper bounds of the sub-array being sorted.


The function first checks if low is less than high, and if so, it calls the partition function to partition the array around a pivot element. The partition function selects the last element in the sub-array as the pivot, and then rearranges the elements so that all elements less than the pivot are to the left of it, and all elements greater than the pivot are to the right of it.


The partition function returns the index of the pivot element after partitioning, and the quick_sort function then recursively sorts the sub-array to the left of the pivot (using quick_sort(arr, low, pi - 1)) and the sub-array to the right of the pivot (using quick_sort(arr, pi + 1, high)).


Interesting Feature
One interesting feature of the Quick Sort algorithm is its use of a pivot element to partition the array. The choice of pivot can have a significant impact on the performance of the algorithm, and there are many different strategies for selecting a good pivot.


In this implementation, the pivot is always chosen to be the last element in the sub-array being sorted. This is a simple and effective strategy for small arrays, but can lead to poor performance for large arrays that are already sorted or nearly sorted.


To overcome this limitation, more advanced partitioning strategies such as the "median-of-three" pivot selection can be used. This strategy involves selecting the median value from the first, middle, and last elements in the sub-array as the pivot, which can help to ensure a more balanced partitioning of the array.


Here is an example of how the partition function works:
```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
let low = 0;
let high = arr.len() - 1;
let pi = partition(&mut arr, low, high);
println!("{:?}", arr); // Output: [1, 1, 2, 3, 3, 4, 5, 6, 5, 9, 5]
```
In this example, the partition function is called with the arr slice, low index of 0, and high index of 10. The function selects the last element in the sub-array (5) as the pivot, and then iterates over the elements in the sub-array from left to right.

For each element that is less than the pivot (1, 1, 4, 2, 3, 3, 5, and 5), the element is swapped with the element at the current index i (which is initially equal to low, or 0). This ensures that all elements less than the pivot are to the left of index i.

Finally, the pivot element is swapped with the element at index i (which is the first element that is greater than or equal to the pivot). This ensures that the pivot is in its correct sorted position in the sub-array. The function then returns the index of the pivot (i).

_______________________________________________________________________________________________________________________________________________

Rust Implementation of Selection Sort
This repository contains a Rust implementation of [Selection Sort](./src/selection_sort.rs) algorithm, which is a comparison-based sorting algorithm that divides an input array into a sorted and an unsorted part. The sorted part is initially empty and the unsorted part is the entire input array. The algorithm proceeds by finding the smallest element in the unsorted part, swapping it with the leftmost unsorted element, and then moving the boundary between the sorted and unsorted parts one element to the right. This process is repeated until the entire array is sorted.


Code Description
The implementation of selection sort in Rust is done using two nested loops. In the outer loop, the variable i iterates over the range of indices 0..n, where n is the length of the input array arr. In the inner loop, the variable j iterates over the range of indices (i + 1)..n. The purpose of the inner loop is to find the index min_idx of the smallest element in the subarray arr[i..n]. Once the smallest element has been found, it is swapped with the element at index i, which puts the smallest element in its correct position in the sorted subarray arr[0..i+1].


Interesting Feature
An interesting feature of selection sort is that it has a runtime complexity of O(n^2), which makes it less efficient than other sorting algorithms such as quicksort and mergesort. However, it has the advantage of requiring only a small amount of additional memory space, which makes it useful for sorting arrays that are too large to fit into memory. Additionally, selection sort has the property of being stable, which means that it preserves the relative order of equal elements in the input array.

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_idx = i;

        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        arr.swap(i, min_idx);
    }
}

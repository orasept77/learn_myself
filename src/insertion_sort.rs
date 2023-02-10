fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 1..n {
        let key = arr[i];
        let mut j = i - 1;

        while j >= 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        arr[j + 1] = key;
    }
}

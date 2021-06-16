fn insert_sort<T: PartialOrd>(arr: &mut [T]) {
    for ni in 1..arr.len() {
        let mut nj = ni;
        while 0 < nj && arr[nj-1] > arr[nj] {
            // res[nj]とres[nj-1]の入れ替え
            arr.swap(nj-1, nj);
            nj -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::insert_sort;

    #[test]
    fn it_works() {
        let mut arr: Vec<isize> = vec![5, 2, 4, 6, 1, 3];
        insert_sort(&mut arr);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], arr)
    }
}

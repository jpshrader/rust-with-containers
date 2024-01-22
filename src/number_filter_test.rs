#[cfg(test)]
mod filter_test {
    use crate::number_filter;

    #[test]
    fn it_filters_even_nums() {
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let even_nums = number_filter::filter(&numbers, |n| n % 2 == 0);

        assert_eq!(even_nums, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn it_filters_odd_nums() {
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let odd_nums = number_filter::filter(&numbers, |n| n % 2 != 0);

        assert_eq!(odd_nums, vec![1, 3, 5, 7, 9]);
    }
}
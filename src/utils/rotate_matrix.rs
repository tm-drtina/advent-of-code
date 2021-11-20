pub fn rotate_matrix_cc<T>(mat: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut res: Vec<Vec<T>> = (0..mat[0].len())
        .map(|i| mat.iter().map(|inner| inner[i].clone()).collect())
        .collect();
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::rotate_matrix_cc;

    #[test]
    fn test_rotate_cc_3_3() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        let actual = rotate_matrix_cc(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rotate_cc_4_3() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        let expected = vec![
            vec![4, 8, 12],
            vec![3, 7, 11],
            vec![2, 6, 10],
            vec![1, 5, 9],
        ];
        let actual = rotate_matrix_cc(&data);

        assert_eq!(expected, actual);
    }
}

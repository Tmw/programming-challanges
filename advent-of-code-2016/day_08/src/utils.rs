pub fn rotate_vec<T: Copy>(by: usize, source: &Vec<T>) -> Vec<T> {
    let mut source = source.to_owned();

    for _ in 0..by {
        let last = source.pop().unwrap();
        source.insert(0, last);
    }

    source
}

#[cfg(test)]
mod rotate_vec_test {
    use super::rotate_vec;

    #[test]
    fn test_rotate_vec() {
        let input = vec![true, false, false, false];
        let output = rotate_vec(2, &input);

        assert_eq!(output, vec![false, false, true, false]);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_equals() {
        let b1 = "String";
        let b2 = "String".to_owned();
        assert_eq!(b1, b2);
    }
}

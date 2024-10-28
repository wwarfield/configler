// sum 2 values and return string
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_as_string(3, 2);
        assert_eq!(result, "5");
    }
}
#[must_use]
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

#[cfg(test)]
mod tests {
    use super::sum_as_string;

    #[test]
    fn formats_sum() {
        assert_eq!(sum_as_string(2, 3), "5");
    }
}

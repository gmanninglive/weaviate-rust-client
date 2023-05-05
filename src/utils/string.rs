pub fn trim_trailing_slash(string: String) -> String {
    if string.ends_with('/') {
        return string[0..string.len() - 1].to_string();
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trimemove_trailing_slash() {
        let test_case_1 = trim_trailing_slash("test/".to_owned());
        let test_case_2 = trim_trailing_slash("testing".to_owned());

        assert_eq!(test_case_1, "test".to_owned());
        assert_eq!(test_case_2, "testing".to_owned());
    }
}

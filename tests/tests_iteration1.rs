use password_validation::iteration1_password_is_valid as password_is_valid;

#[test]
fn test_when_meets_all_criteria_is_valid() {
    assert!(password_is_valid("Aa3_00000"))
}

#[test]
fn test_when_has_zero_characters_password_invalid() {
    assert!(!password_is_valid(""))
}

#[test]
fn test_when_no_uppercase_is_invalid() {
    assert!(!password_is_valid("aa3_00000"))
}

#[test]
fn test_when_no_lowercase_is_invalid() {
    assert!(!password_is_valid("AA3_00000"))
}

#[test]
fn test_when_no_numeric_is_invalid() {
    assert!(!password_is_valid("Aaa_@@@@@"))
}

#[test]
fn test_when_no_underscore_is_invalid() {
    assert!(!password_is_valid("Aa3-00000"))
}

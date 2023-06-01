use password_validation::iteration2_password_is_valid as password_is_valid;
use password_validation::PasswordSpec;

const VALIDATION_TWO: PasswordSpec = PasswordSpec {
    minimum_length: 6 + 1,
    check_uppercase: true,
    check_lowercase: true,
    check_numeric: true,
    check_underscore: false,
};

const VALIDATION_THREE: PasswordSpec = PasswordSpec {
    minimum_length: 16 + 1,
    check_uppercase: true,
    check_lowercase: true,
    check_numeric: false,
    check_underscore: true,
};

#[test]
fn test_when_meets_all_criteria_is_valid() {
    assert!(password_is_valid(
        "Aa3_00000",
        PasswordSpec {
            minimum_length: 9,
            ..Default::default()
        }
    ));
    assert!(password_is_valid(
        "Aa3_00",
        PasswordSpec {
            minimum_length: 6,
            ..Default::default()
        }
    ))
}

#[test]
fn test_when_password_is_short() {
    assert!(!password_is_valid(
        "",
        PasswordSpec {
            minimum_length: 9,
            ..Default::default()
        }
    ));
    assert!(!password_is_valid(
        "Aa3_00",
        PasswordSpec {
            minimum_length: 7,
            ..Default::default()
        }
    ))
}
#[test]
fn test_validation_two_is_valid() {
    assert!(password_is_valid("Aa0####", VALIDATION_TWO))
}

#[test]
fn test_validation_two_is_invalid() {
    assert!(!password_is_valid("Aa0###", VALIDATION_TWO));
    assert!(!password_is_valid("aa0####", VALIDATION_TWO));
    assert!(!password_is_valid("AA0####", VALIDATION_TWO));
    assert!(!password_is_valid("Aaa####", VALIDATION_TWO))
}

#[test]
fn test_validation_three_is_valid() {
    assert!(password_is_valid("Aa_##############", VALIDATION_THREE))
}

#[test]
fn test_validation_three_is_invalid() {
    assert!(!password_is_valid("Aa_#############", VALIDATION_THREE));
    assert!(!password_is_valid("aa_##############", VALIDATION_THREE));
    assert!(!password_is_valid("AA_##############", VALIDATION_THREE));
    assert!(!password_is_valid("Aaa##############", VALIDATION_THREE))
}

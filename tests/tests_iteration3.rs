use password_validation::iteration3_password_is_valid as password_is_valid;
use password_validation::PasswordError;
use password_validation::PasswordSpec;

#[test]
fn test_when_all_criteria_valid() {
    assert_eq!(
        Ok(()),
        password_is_valid(
            "Aa3_00000",
            PasswordSpec {
                minimum_length: 9,
                ..Default::default()
            }
        )
    )
}

#[test]
fn test_when_check_one_reports_correct_error() {
    assert_eq!(
        Err(PasswordError {
            length: true,
            ..Default::default()
        }),
        password_is_valid(
            "Aa3_0000",
            PasswordSpec {
                minimum_length: 9,
                ..Default::default()
            }
        )
    );
    assert_eq!(
        Err(PasswordError {
            uppercase: true,
            ..Default::default()
        }),
        password_is_valid(
            "aa3_0000",
            PasswordSpec {
                check_uppercase: true,
                ..Default::default()
            }
        )
    );
    assert_eq!(
        Ok(()),
        password_is_valid(
            "Aa30000",
            PasswordSpec {
                check_uppercase: true,
                ..Default::default()
            }
        )
    )
}

#[test]
fn test_when_check_two_reports_correct_error() {
    assert_eq!(
        Err(PasswordError {
            length: true,
            ..Default::default()
        }),
        password_is_valid(
            "Aa3_0000",
            PasswordSpec {
                minimum_length: 9,
                check_uppercase: true,
                ..Default::default()
            }
        )
    );
    assert_eq!(
        Err(PasswordError {
            uppercase: true,
            ..Default::default()
        }),
        password_is_valid(
            "aa3_00000",
            PasswordSpec {
                minimum_length: 9,
                check_uppercase: true,
                ..Default::default()
            }
        )
    );
    assert_eq!(
        Err(PasswordError {
            length: true,
            uppercase: true,
            ..Default::default()
        }),
        password_is_valid(
            "aa3_0000",
            PasswordSpec {
                minimum_length: 9,
                check_uppercase: true,
                ..Default::default()
            }
        )
    );
}

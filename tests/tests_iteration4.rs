use password_validation::iteration4_password_is_valid as password_is_valid;
use password_validation::PasswordSpec;

#[test]
fn test_when_check_one_returns_correct_value() {
    assert_eq!(
        Ok(()),
        password_is_valid(
            "Aa3_0000",
            PasswordSpec {
                minimum_length: 9,
                ..Default::default()
            },
            1
        )
    );
    assert_eq!(
        Ok(()),
        password_is_valid(
            "aa3_0000",
            PasswordSpec {
                check_uppercase: true,
                ..Default::default()
            },
            1
        )
    );
    assert_eq!(
        Ok(()),
        password_is_valid(
            "Aa30000",
            PasswordSpec {
                check_uppercase: true,
                ..Default::default()
            },
            1
        )
    )
}

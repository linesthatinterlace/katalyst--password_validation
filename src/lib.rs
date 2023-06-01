#[derive(Default)]
pub struct PasswordSpec {
    pub minimum_length: usize,
    pub check_uppercase: bool,
    pub check_lowercase: bool,
    pub check_numeric: bool,
    pub check_underscore: bool,
}

#[derive(Default, PartialEq, Debug)]
pub struct PasswordError {
    pub length: bool,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numeric: bool,
    pub underscore: bool,
}

pub fn iteration1_password_is_valid(a: &str) -> bool {
    iteration2_password_is_valid(
        a,
        PasswordSpec {
            minimum_length: 8 + 1,
            check_uppercase: true,
            check_lowercase: true,
            check_numeric: true,
            check_underscore: true,
        },
    )
}

pub fn iteration2_password_is_valid(a: &str, spec: PasswordSpec) -> bool {
    iteration3_password_is_valid(a, spec).is_ok()
}

pub fn iteration3_password_is_valid(a: &str, spec: PasswordSpec) -> Result<(), PasswordError> {
    iteration4_password_is_valid(a, spec, 0)
}

pub fn iteration4_password_is_valid(
    a: &str,
    spec: PasswordSpec,
    threshold: usize,
) -> Result<(), PasswordError> {
    let length = a.len() < spec.minimum_length;
    let uppercase = spec.check_uppercase && a.chars().all(|c| !c.is_uppercase());
    let lowercase = spec.check_lowercase && a.chars().all(|c| !c.is_lowercase());
    let numeric = spec.check_numeric && a.chars().all(|c| !c.is_numeric());
    let underscore = spec.check_underscore && a.chars().all(|c| c != '_');

    if [length, uppercase, lowercase, numeric, underscore]
        .into_iter()
        .filter(|&x| x)
        .count()
        > threshold
    {
        Err(PasswordError {
            length,
            uppercase,
            lowercase,
            numeric,
            underscore,
        })
    } else {
        Ok(())
    }
}

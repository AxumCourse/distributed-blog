pub fn hash(pwd: &str) -> Result<String, String> {
    bcrypt::hash(pwd, bcrypt::DEFAULT_COST).map_err(|err| err.to_string())
}
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool, String> {
    bcrypt::verify(pwd, hashed_pwd).map_err(|err| err.to_string())
}

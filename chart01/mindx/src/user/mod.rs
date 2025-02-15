#[allow(unused)]
pub mod api;
pub mod model;
pub mod permission;

#[allow(dead_code)]
pub(crate) fn build_password_hash(password: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(password, 10u32)?)
}

pub(crate) fn verify_password_hash(password: &str, password_hash: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(password, password_hash)?)
}

#[allow(unused)]
pub(crate) fn verify_password_hash_option(
    password: &str,
    password_hash: &Option<String>,
) -> anyhow::Result<bool> {
    if let Some(password_hash) = password_hash {
        verify_password_hash(password, password_hash)
    } else {
        Err(anyhow::anyhow!("password_hash is empty"))
    }
}

use deno_core::error::AnyError;

#[deno_core::op2(async)]
#[string]
pub async fn op_language_server_latest_npm_package_version() -> Result<String, AnyError> {
    Ok("the-version".into())
}

use lib_core::ctx::Ctx;

use crate::params::ParamsForCreate;

pub async fn create_user(
    ctx: Ctx,
    _mm: ModelManager,
    _params: ParamsForCreate<UserForCreate>
) -> Result<User> {
    
}
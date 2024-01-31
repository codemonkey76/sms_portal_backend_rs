use crate::router::RpcRouter;

pub mod customer;
pub mod list;
pub mod contact;
pub mod message;
pub mod user;
mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(customer::rpc_router())
		.extend(contact::rpc_router())
}
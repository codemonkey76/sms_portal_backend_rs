use crate::router::RpcRouter;

pub mod agent;
pub mod conversation;
pub mod customer;
mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(agent::rpc_router())
		.extend(conversation::rpc_router())
		.extend(customer::rpc_router())
}
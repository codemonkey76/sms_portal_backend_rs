use crate::router::RpcRouter;

pub mod customer;
mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(customer::rpc_router())
}
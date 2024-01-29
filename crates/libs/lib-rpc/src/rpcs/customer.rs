use crate::rpcs::prelude::*;
use lib_core::model::customer::{
	Customer, CustomerBmc, CustomerFilter, CustomerForCreate, CustomerForUpdate
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		create_customer,
		get_customer,
		list_customers,
		update_customer,
		delete_customer,
	)
}

generate_common_rpc_fns!(
	Bmc: CustomerBmc,
	Entity: Customer,
	ForCreate: CustomerForCreate,
	ForUpdate: CustomerForUpdate,
	Filter: CustomerFilter,
	Suffix: customer
);
use crate::rpcs::prelude::*;
use lib_core::model::contact::{
	Contact, ContactBmc, ContactFilter, ContactForCreate, ContactForUpdate
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		create_contact,
		get_contact,
		list_contacts,
		update_contact,
		delete_contact,
	)
}

generate_common_rpc_fns!(
	Bmc: ContactBmc,
	Entity: Contact,
	ForCreate: ContactForCreate,
	ForUpdate: ContactForUpdate,
	Filter: ContactFilter,
	Suffix: contact
);
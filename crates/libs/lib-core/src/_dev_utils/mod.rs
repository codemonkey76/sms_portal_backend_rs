// region:    --- Modules

mod dev_db;

use fake::Dummy;
use crate::ctx::Ctx;
use crate::model::customer::{CustomerBmc, CustomerFilter, CustomerForCreate};
use crate::model::contact::{ContactBmc, ContactFilter, ContactForCreate};
use crate::model::{self, ModelManager};
use modql::filter::OpValString;
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT.get_or_init(|| async {
		info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			// NOTE: Rare occasion where unwrap is kind of ok.
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}

// region:    --- User seed/clean

pub async fn seed_users(
	ctx: &Ctx,
	mm: &ModelManager,
	usernames: &[&str],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for name in usernames {
		let id = seed_user(ctx, mm, name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_user(
	ctx: &Ctx,
	mm: &ModelManager,
	username: &str,
) -> model::Result<i64> {
	let pwd_clear = "seed-user-pwd";

	let id = model::user::UserBmc::create(
		ctx,
		mm,
		model::user::UserForCreate {
			username: username.to_string(),
			pwd_clear: pwd_clear.to_string(),
		},
	)
	.await?;

	Ok(id)
}

pub async fn clean_users(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_username: &str,
) -> model::Result<usize> {
	let users = model::user::UserBmc::list(
		ctx,
		mm,
		Some(vec![model::user::UserFilter {
			username: Some(
				OpValString::Contains(contains_username.to_string()).into(),
			),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = users.len();

	for user in users {
		model::user::UserBmc::delete(ctx, mm, user.id).await?;
	}

	Ok(count)
}

// endregion: --- User seed/clean

// region:    --- Customer seed/clean

pub async fn seed_customers(
	ctx: &Ctx,
	mm: &ModelManager,
	params: &[(&str, &str)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (name, sender_id) in params {
		let id = seed_customer(ctx, mm, name, sender_id).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_customer(
	ctx: &Ctx,
	mm: &ModelManager,
	name: &str,
	sender_id: &str,
) -> model::Result<i64> {
	CustomerBmc::create(
		ctx,
		mm,
		CustomerForCreate {
			name: name.to_string(),
			sender_id: sender_id.to_string()
		},
	)
	.await
}

/// Delete all customers that have their title contains contains_name
pub async fn clean_customers(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_name: &str,
) -> model::Result<usize> {
	let customers = CustomerBmc::list(
		ctx,
		mm,
		Some(vec![CustomerFilter {
			name: Some(OpValString::Contains(contains_name.to_string()).into()),
			sender_id: None,
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = customers.len();

	for customer in customers {
		CustomerBmc::delete(ctx, mm, customer.id).await?;
	}

	Ok(count)
}

// endregion: --- Customer seed/clean

// region:    --- Contact seed/clean

pub async fn seed_contacts(
	ctx: &Ctx,
	mm: &ModelManager,
	params: &[(&str, &i64, &str)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (phone, customer_id, first_name) in params {
		let id = seed_contact(ctx, mm, phone, customer_id, first_name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_contact(
	ctx: &Ctx,
	mm: &ModelManager,
	phone: &str,
	customer_id: &i64,
	first_name: &str
) -> model::Result<i64> {
	let faker = fake::Faker;
	ContactBmc::create(
		ctx,
		mm,
		ContactForCreate {
			phone: phone.to_string(),
            customer_id: *customer_id,
			first_name: first_name.to_string(),
            ..ContactForCreate::dummy(&faker)
		},
	)
	.await
}

/// Delete all contacts that have their title contains contains_name
pub async fn clean_contacts(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_first_name: &str,
) -> model::Result<usize> {
	let contacts = ContactBmc::list(
		ctx,
		mm,
		Some(vec![ContactFilter {
			first_name:Some(OpValString::Contains(contains_first_name.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	
	let count = contacts.len();
	

	for contact in contacts {
		ContactBmc::delete(ctx, mm, contact.id).await?;
	}

	Ok(count)
}

// endregion: --- contact seed/clean
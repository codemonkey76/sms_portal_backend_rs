use fake::Dummy;
use lib_utils::time::Rfc3339;
use modql::{field::Fields, filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue}};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use crate::model::Result;
use time::OffsetDateTime;
use crate::{ctx::Ctx, generate_common_bmc_fns, model::modql_utils::time_to_sea_value};
use crate::model::base;
use super::{base::DbBmc, ModelManager};
use fake::faker::{company::en::CompanyName, phone_number::en::PhoneNumber, name::en::{FirstName, LastName}};

#[serde_as]
#[derive(Dummy, Debug, Clone, Fields, FromRow, Serialize)]
pub struct Contact {
    #[dummy(default)]
    pub id: i64,
    
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,

    #[dummy(faker = "FirstName()")]
    pub first_name: String,

    #[dummy(faker = "LastName()")]
    pub last_name: String,

    #[dummy(faker = "CompanyName()")]
    pub company_name: String,

    #[dummy(default)]
    pub customer_id: i64,

    #[dummy(default)]
    pub cid: i64,

    #[serde_as(as = "Rfc3339")]
    #[dummy(expr = "OffsetDateTime::now_utc()")]
    pub ctime: OffsetDateTime,
    
    #[dummy(default)]
    pub mid: i64,
    
    #[serde_as(as = "Rfc3339")]
    #[dummy(expr = "OffsetDateTime::now_utc()")]
    pub mtime: OffsetDateTime
}


#[derive(Dummy, Fields, Deserialize)]
pub struct ContactForCreate {
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
    #[dummy(faker = "FirstName()")]
    pub first_name: String,
    #[dummy(faker = "LastName()")]
    pub last_name: String,
    #[dummy(faker = "CompanyName()")]
    pub company_name: String,
    #[dummy(faker = "100..10000")]
    pub customer_id: i64
}

#[derive(Fields, Deserialize, Default)]
pub struct ContactForUpdate {
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
}

#[derive(Debug, FilterNodes, Default, Deserialize)]
pub struct ContactFilter {
    pub id: Option<OpValsInt64>,

    pub phone: Option<OpValsString>,
    pub first_name: Option<OpValsString>,
    pub last_name: Option<OpValsString>,
    pub company_name: Option<OpValsString>,

    pub customer_id: Option<OpValsInt64>,

    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

pub struct ContactBmc {}

impl DbBmc for ContactBmc {
    const TABLE: &'static str = "contacts";
}

generate_common_bmc_fns!(
    Bmc: ContactBmc,
    Entity: Contact,
    ForCreate: ContactForCreate,
    ForUpdate: ContactForUpdate,
    Filter: ContactFilter
);

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>;

    use super::*;
    use crate::{_dev_utils::{self, clean_contacts, seed_contact, seed_contacts}, model};
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let faker = fake::Faker;
        let fx_phone = "12345678";
        let fx_customer_id = 100;
        let fx_first_name = "test_create_ok";

        // -- Exec
        let fx_contact_c = ContactForCreate {
            phone:fx_phone.to_string(),
            customer_id: fx_customer_id,
            first_name: fx_first_name.to_string(),
            ..ContactForCreate::dummy(&faker)
        };

        let contact_id = ContactBmc::create(&ctx, &mm, fx_contact_c).await?;

        // -- Check
        let contact = ContactBmc::get(&ctx, &mm, contact_id).await?;
        assert_eq!(contact.phone, fx_phone);

        // -- Clean
        let count = clean_contacts(&ctx, &mm, fx_first_name).await?;
		assert_eq!(count, 1, "Should have cleaned only 1 contact");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        // -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_first_name = "test_update_ok contact 01";
        let fx_phone = "12345678";
        let fx_customer_id = 100;
		let fx_contact_id = seed_contact(&ctx, &mm, fx_phone, &fx_customer_id, fx_first_name).await?;
		let fx_phone_updated = "87654321";

		// -- Exec
		let fx_contact_u = ContactForUpdate {
            phone: Some(fx_phone_updated.to_string()),    
            ..ContactForUpdate::default()
		};
		ContactBmc::update(&ctx, &mm, fx_contact_id, fx_contact_u).await?;

		// -- Check
		let contact = ContactBmc::get(&ctx, &mm, fx_contact_id).await?;
		assert_eq!(contact.phone, fx_phone_updated);
        assert_eq!(contact.first_name, fx_first_name);

		// -- Clean
		let count = clean_contacts(&ctx, &mm, fx_first_name).await?;
		assert_eq!(count, 1, "Should have cleaned only 1 contact");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_ok() -> Result<()> {
        // -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_first_name = "test_delete_ok contact 01";
        let fx_phone = "12345678";
        let fx_customer_id = 100;
		let fx_contact_id = seed_contact(&ctx, &mm, fx_phone,  &fx_customer_id, fx_first_name).await?;

		// -- Exec
		// check it's there
		ContactBmc::get(&ctx, &mm, fx_contact_id).await?;
		// do the delete
		ContactBmc::delete(&ctx, &mm, fx_contact_id).await?;

		// -- Check
		let res = ContactBmc::get(&ctx, &mm, fx_contact_id).await;
		assert!(
			matches!(&res, Err(model::Error::EntityNotFound { .. })),
			"should return a EntityNotFound"
		);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_customer1_contacts = &[("test_list_ok contact 01", &100, "test_list_ok contact"), ("test_list_ok contact 02", &100, "test_list_ok contact")];
		seed_contacts(&ctx, &mm, fx_customer1_contacts).await?;
	
        let fx_customer2_contacts = &[
			("test_list_ok cust 2, contact 1", &101, "test_list_ok cust 2"),
            ("test_list_ok cust 2, contact 2", &101, "test_list_ok cust 2"),
            ("test_list_ok cust 2, contact 3", &101, "test_list_ok cust 2"),
            ("test_list_ok cust 2, contact 4", &101, "test_list_ok cust 2"),
		];
		seed_contacts(&ctx, &mm, fx_customer2_contacts).await?;

		// -- Exec
		let contact_filter: ContactFilter = serde_json::from_value(json!(
			{
				"phone": {"$contains": "list_ok contact"}
			}
		))?;
		let contacts =
			ContactBmc::list(&ctx, &mm, Some(vec![contact_filter]), None).await?;

		// -- Check
		assert_eq!(contacts.len(), 2);
		let phones: Vec<String> = contacts.iter().cloned().map(|a| a.phone).collect::<Vec<_>>();
        let fx_phones: Vec<String> = fx_customer1_contacts.iter().map(|(phone,_, _)| phone.to_string()).collect::<Vec<_>>();
		assert_eq!(phones, fx_phones);

		// -- Clean
		let count = clean_contacts(&ctx, &mm, "test_list_ok contact").await?;
		assert_eq!(count, 2, "Should have cleaned 2 contacts");
		let count = clean_contacts(&ctx, &mm, "test_list_ok cust 2").await?;
		assert_eq!(count, 4, "Should have cleaned 3 contacts");

        Ok(())
    }



}

// endregion: --- Tests
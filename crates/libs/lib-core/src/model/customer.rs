use modql::{field::Fields, filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue}};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::model::{Result, base, ModelManager};
use crate::ctx::Ctx;
use modql::filter::ListOptions;
use sqlx::FromRow;
use time::OffsetDateTime;
use lib_utils::time::Rfc3339;
use crate::{generate_common_bmc_fns, model::modql_utils::time_to_sea_value};

use super::base::DbBmc;


// region:    --- Customer Types


/// Trait to implement on entities that have a conv_id
/// This will allow Ctx to be upgraded with the corresponding conv_id for
/// future access control.
pub trait CustomerScoped {
	fn customer_id(&self) -> i64;
}

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Customer {
    pub id: i64,

    pub name: String,
    pub sender_id: String,

    pub is_active: bool,

    // -- Timestamps
    //    (creator and last modified user_id and time)
    pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct CustomerForCreate {
    pub name: String,
    pub sender_id: String
}

#[derive(Fields, Deserialize)]
pub struct CustomerForUpdate {
    pub name: Option<String>,
    pub sender_id: Option<String>
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct CustomerFilter {
    pub id: Option<OpValsInt64>,
    pub name: Option<OpValsString>,
    pub sender_id: Option<OpValsString>,

    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

// endregion: --- Customer Types

pub struct CustomerBmc {}

impl DbBmc for CustomerBmc {
    const TABLE: &'static str = "customers";
}

// This will generate the `impl CustomerBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
    Bmc: CustomerBmc,
    Entity: Customer,
    ForCreate: CustomerForCreate,
    ForUpdate: CustomerForUpdate,
    Filter: CustomerFilter
);

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>;

    use super::*;
    use crate::_dev_utils::{self, clean_customers};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok customer 01";
        let fx_sender_id = "test_create_ok";

        // -- Exec
        let fx_customer_c = CustomerForCreate {
            name: fx_name.to_string(),
            sender_id: fx_sender_id.to_string()
        };

        let customer_id = CustomerBmc::create(&ctx, &mm, fx_customer_c).await?;

        // -- Check
        let customer = CustomerBmc::get(&ctx, &mm, customer_id).await?;
        assert_eq!(customer.name, fx_name);

        // -- Clean
        let count = clean_customers(&ctx, &mm, "test_create_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 agent");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn update_create_ok() -> Result<()> {
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn delete_create_ok() -> Result<()> {
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn list_create_ok() -> Result<()> {
        Ok(())
    }



}

// endregion: --- Tests
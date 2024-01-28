use modql::{field::Fields, filter::{FilterNodes, OpValsInt64, OpValsValue}};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use lib_utils::time::Rfc3339;
use crate::model::modql_utils::time_to_sea_value;
use super::{base::DbBmc, customer::CustomerScoped};


#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct CustomerUser {
    pub id: i64,

    // -- FK
    pub customer_id: i64,
    pub user_id: i64,

    // -- Timestamps
	// creator user_id and time
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	// last modifier user_id and time
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

impl CustomerScoped for CustomerUser {
    fn customer_id(&self) -> i64 {
        self.customer_id
    }
}


#[derive(Fields, Deserialize)]
pub struct CustomerUserForCreate {
	pub customer_id: i64,
	pub content: String,
}

impl CustomerScoped for CustomerUserForCreate {
    fn customer_id(&self) -> i64 {
        self.customer_id
    }
}

/// CustomerUser for Insert, which is derived from the public `CustomerUserForCreate`.
///
/// Notes:
///   - When `...ForCreate` requires additional information for insertion into the DB, the pattern
///     is to create a `...ForInsert` type, visible only in the model layer.
///   - This approach maintains a simple and ergonomic public API while ensuring
///     strong typing for database insertion.
///   - Exceptions apply to lower-level attributes like cid, ctime, mid, mtime, and owner_id,
///     which can be set directly through the base:: functions or some utilities. There's not
///     significant value in introducing `...ForInsert` types for all entities just for these
///     common, low-level database properties.
#[derive(Fields, Deserialize)]
pub(in crate::model) struct CustomerUserForInsert {
	pub customer_id: i64,
	pub user_id: i64,
}

#[derive(Fields, Deserialize, Default)]
pub struct CustomerUserForUpdate {
	pub customer_id: i64,
	pub content: Option<String>,
}

impl CustomerScoped for CustomerUserForUpdate {
	fn customer_id(&self) -> i64 {
		self.customer_id
	}
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CustomerUserFilter {
	id: Option<OpValsInt64>,

	customer_id: Option<OpValsInt64>,
    user_id: Option<OpValsInt64>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

// endregion: --- Types

// region:    --- CustomerUserBmc

pub struct CustomerUserBmc;

impl DbBmc for CustomerUserBmc {
	const TABLE: &'static str = "customer_users";
}

// Note: The strategy here is to not implement `CustomerUserBmc` CRUD functions,
//       as they will be managed directly from the `CustomerBmc` construct
//       This is because `CustomerUser` is an leaf entity better managed by its container `ConvBmc`.

// endregion: --- CustomerUserBmc
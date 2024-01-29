#![allow(unused)] // For example code.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.



use httpc_test::{Client, Response};
use serde_json::{json, Value, Map};


#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	
	// -- Login
	login(&hc).await?;


	// region:    --- Create Customer
	let params = json!({
		"data": {
			"name": "Test Customer",
			"sender_id": "abc123"
		}
	});


	let result = rpc_method(&hc, 1, "create_customer", &params).await?;
	let id = result.json_value::<i64>("/result/data/id")?;
	// endregion: --- Create Customer

	// region:    --- Update Customer
	
	let params = json!({
		"id": id,
		"data": {
			"name": "Edited"
		}
	});
	
	rpc_method(&hc, 2, "update_customer", &params).await?;
	
	// endregion: --- Update Customer


	// region:    --- Get Customer
	// endregion: --- Get Customer

	// region:    --- List Customers
	// endregion: --- List Customers
	
	

	// -- Logout
	logout(&hc).await?;


	Ok(())
}



async fn rpc_method(hc: &Client, id: i64, method: &str, params: &Value) -> Result<Response> {
	let result = hc.do_post("/api/rpc", json!({
		"id": id,
		"method": method.to_string(),
		"params": params
	})).await?;
	
	result.print().await?;


	Ok(result)
}

async fn login(hc: &Client) -> Result<()> {
	// -- Login
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);

	req_login.await?;

	Ok(())

}

async fn logout(hc: &Client) -> Result<()> {
	// -- Logoff
	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	
	req_logoff.await?;
	
	Ok(())

}
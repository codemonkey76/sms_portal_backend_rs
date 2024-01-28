#![allow(unused)] // For example code.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.



use httpc_test::{Client, Response};
use serde_json::{json, Value, Map};


macro_rules! map {
	() => {
		serde_json::Map::new()
	};

	($($key:expr => $value:expr),+ $(,)?) => {{
		let mut temp_map = serde_json::Map::new();
		$(
			temp_map.insert($key.to_string(), serde_json::Value::String($value.to_string()));
		)+
		temp_map
	}};
}

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	login(&hc).await?;


	let params = map! {
		"name" => "Test Customer",
		"sender_id" => "abc123"
	};


	rpc_method(&hc, 1, "create_customer", &params).await?;

	logout(&hc).await?;



	Ok(())
}



async fn rpc_method(hc: &Client, id: i64, method: &str, params: &Map<String, Value>) -> Result<()> {
	hc.do_post("/api/rpc", json!({
		"id": id,
		"method": method.to_string(),
		"params": {
			"data": params
		}
	})).await?.print().await?;

	Ok(())
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
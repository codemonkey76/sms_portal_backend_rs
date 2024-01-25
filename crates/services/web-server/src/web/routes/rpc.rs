use axum::{response::Response, Json, Router};
use lib_rpc::RpcRequest;
use tracing::debug;

pub fn routes() -> Router {Router::new().route("/rpc", post(rpc_handler))}
async fn rpc_handler(
    Json(rpc_request): Json<RpcRequest>
) -> Response {

}

async fn _rpc_handler(
    rpc_request: RpcRequest
) -> Result<Json<Value>> {
    
    let rpc_method = rpc_request.method.clone();
    let rpc_id = rpc_request.id.clone();

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");
    
    let result = exec_rpc(rpc_request).await?;

    let body_response = json!({
        "id": rpc_id,
        "result": result
    });

}
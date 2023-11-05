use std::error::Error;

use lsp_server::Connection;
use lsp_types::{request::Initialize, ClientCapabilities, InitializeParams, ServerCapabilities};
fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    //Create transport
    let (connection, io_threads) = Connection::stdio();
    let (id, params) = connection.initialize_start()?;

    let init_params: InitializeParams = serde_json::from_value(params).expect("Params failed");
    let client_capabilities: ClientCapabilities = init_params.capabilities;
    let server_capabilities: ServerCapabilities = ServerCapabilities::default();

    let initialize_data = serde_json::json!({
       "capabilities": server_capabilities,
       "serverInfo": {
           "name": "lsp-server-test",
           "version": "0.1"
       }
    });

    connection.initialize_finish(id, initialize_data)?;

    // --- start of main loop ---

    //TODO: Implement

    // --- end of main loop ---

    Ok(())
}

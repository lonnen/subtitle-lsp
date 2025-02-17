use tower_lsp::lsp_types::*;

pub fn server_capabilities(client_capabilities: &ClientCapabilities) -> ServerCapabilities {
    ServerCapabilities {
        ..Default::default()
    }
}
use tower_lsp::lsp_types::*;

pub fn server_capabilities(_client_capabilities: &ClientCapabilities) -> ServerCapabilities {
    ServerCapabilities {
        // text_document_sync: Some(TextDocumentSyncCapability::Options(
        //     TextDocumentSyncOptions {
        //         open_close: Some(true),
        //         change: Some(TextDocumentSyncKind::INCREMENTAL),
        //         will_save: None,
        //         will_save_wait_until: None,
        //         save: Some(SaveOptions::default().into()),
        //     },
        // )),
        ..Default::default()
    }
}
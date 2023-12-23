use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::request::{GotoTypeDefinitionParams, GotoTypeDefinitionResponse};
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
}
impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: None,
                text_document_sync: None,
                selection_range_provider: None,
                hover_provider: None,
                completion_provider: None,
                signature_help_provider: None,
                definition_provider: Some(OneOf::Left(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                implementation_provider: None,
                references_provider: None,
                document_highlight_provider: None,
                document_symbol_provider: None,
                workspace_symbol_provider: None,
                code_action_provider: None,
                code_lens_provider: None,
                document_formatting_provider: None,
                document_range_formatting_provider: None,
                document_on_type_formatting_provider: None,
                rename_provider: None,
                document_link_provider: None,
                color_provider: None,
                folding_range_provider: None,
                declaration_provider: None,
                execute_command_provider: None,
                workspace: None,
                call_hierarchy_provider: None,
                semantic_tokens_provider: None,
                moniker_provider: None,
                linked_editing_range_provider: None,
                inline_value_provider: None,
                inlay_hint_provider: None,
                diagnostic_provider: None,
                experimental: None,
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        eprintln!("Started");
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        eprintln!("open");
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        eprintln!("change");
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
            language_id: "".to_owned(),
        })
        .await
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        eprintln!("definition");
        let range = Range {
            start: Position {
                line: 1,
                character: 1,
            },
            end: Position {
                line: 2,
                character: 2,
            },
        };
        let uri = params.text_document_position_params.text_document.uri;
        self.client
            .publish_diagnostics(
                uri.clone(),
                vec![Diagnostic::new_simple(range, "Help".to_owned())],
                None,
            )
            .await;
        let _document = match self.document_map.get(uri.as_str()) {
            Some(rope) => rope,
            None => return Ok(None),
        };

        //error!("Got a textDocument/definition request, but it is not implemented");
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri,
            range,
        })))
        //Err(Error::method_not_found())
    }

    async fn goto_type_definition(
        &self,
        params: GotoTypeDefinitionParams,
    ) -> Result<Option<GotoTypeDefinitionResponse>> {
        eprintln!("type definition");
        let range = Range {
            start: Position {
                line: 1,
                character: 1,
            },
            end: Position {
                line: 2,
                character: 2,
            },
        };
        let uri = params.text_document_position_params.text_document.uri;
        self.client
            .publish_diagnostics(
                uri.clone(),
                vec![Diagnostic::new_simple(range, "Help type".to_owned())],
                None,
            )
            .await;
        let _document = match self.document_map.get(uri.as_str()) {
            Some(rope) => rope,
            None => return Ok(None),
        };

        //error!("Got a textDocument/definition request, but it is not implemented");
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri,
            range,
        })))
    }
}

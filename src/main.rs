mod parser;

use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::parser::include::QuteInclude;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        _client: client,
        document_map: DashMap::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
struct Backend {
    _client: Client,
    document_map: DashMap<String, Rope>,
}
impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());
    }

    fn _get_opened_document(
        &self,
        uri: &Url,
    ) -> Option<dashmap::mapref::one::Ref<'_, std::string::String, Rope>> {
        // when file is open
        if let Some(document) = self.document_map.get(uri.as_str()) {
            return Some(document);
        };
        None
    }

    async fn get_document(
        &self,
        uri: &Url,
    ) -> Option<dashmap::mapref::one::Ref<'_, std::string::String, Rope>> {
        // when file is open
        if let Some(document) = self._get_opened_document(uri) {
            return Some(document);
        };

        let Ok(text) = std::fs::read_to_string(uri.path()) else {
            eprintln!("Unable to open file and it is also not available on the client");
            return None;
        };

        // The file was no opened yet on the client so we have to open it.
        self.on_change(TextDocumentItem {
            uri: uri.clone(),
            text,
            version: 1,
            language_id: "".to_owned(),
        })
        .await;

        // The file should now be loaded
        if let Some(document) = self._get_opened_document(uri) {
            return Some(document);
        };
        None
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
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
        let params = params.text_document_position_params;
        let uri = params.text_document.uri;
        let position = params.position;
        let Some(document) = self.get_document(&uri).await else {
            eprintln!("Document is not opened.");
            return Ok(None);
        };
        let Some(line) = document.get_line(position.line.try_into().unwrap_or_default()) else {
            eprintln!("Unable to read the line referecned");
            return Ok(None);
        };

        let Some(template_folder) = get_templates_folder_from_template_uri(uri) else {
            eprintln!("Unable to retrieve template folder");
            return Ok(None);
        };

        if let Some(include) = parser::include::parse_include(line.to_string()) {
            match include {
                QuteInclude::Basic(reference) => {
                    return Ok(reverence_to_gotodefiniton(&reference, &template_folder));
                }
                QuteInclude::Fragment(fragment) => {
                    let reference = fragment.template;
                    return Ok(reverence_to_gotodefiniton(&reference, &template_folder));
                }
            }
        }

        Ok(None)
    }
}

fn reverence_to_gotodefiniton(
    reference: &str,
    templates_folder: &str,
) -> Option<GotoDefinitionResponse> {
    let path = template_reverence_to_path(reference, templates_folder);
    let Ok(uri) = Url::from_file_path(path) else {
        return None;
    };
    Some(GotoDefinitionResponse::Scalar(Location::new(
        uri,
        Range::default(),
    )))
}

fn get_templates_folder_from_template_uri(uri: Url) -> Option<String> {
    let path = uri.path();
    let pattern = "/src/main/resources/templates/";
    let Some((root, _)) = path.split_once(pattern) else {
        return None;
    };
    Some(format!("{}{}", root, pattern))
}

fn template_reverence_to_path(reverence: &str, templates_folder: &str) -> String {
    format!("{}{}.html", templates_folder, reverence)
}

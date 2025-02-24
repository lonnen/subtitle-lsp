mod capabilities;

use core::fmt;

use chumsky::{prelude::*, text::newline};
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::*, 
    Client, LanguageServer};

pub type Span = std::ops::Range<usize>;


#[derive(Debug)]
pub struct Backend {
    client: Client,
}

#[derive(Debug, PartialEq)]
pub struct Timespan {
    start: Timecode,
    end: Timecode,
}

#[derive(Debug, PartialEq)]
pub struct Timecode {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Index(String),
    Timespan(Timespan),
    Timecode(Timecode),
    Text(String),
    Delimeter,
    Card(String),
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{},{}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::Timespan(t) => write!(f, "{} --> {}", t.start, t.end),
            Token::Timecode(t) => write!(f, "{}", t),
            Token::Text(s) => write!(f, "{}", s),
            Token::Delimeter => write!(f, "\n"),
            Token::Card(s) => write!(f, "{}", s),
        }
    }
}

pub fn parser() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // let delimeter = text::newline()
    //     .then(text::newline())
    //     .map(|_| Token::Delimeter);

    // // A parser for indexes
    // let index = text::int(10)
    //     .chain::<char, _, _>(just(',').chain(text::digits(10)).or_not().flatten())
    //     .collect::<String>()
    //     .map(Token::Index);

    // // A parser for timespans
    // // let timespan = ;

    // // parser for timecodes
    // // let timecode = ;

    // // parser for text
    // let text_ = just('"')
    //     .ignore_then(filter(|c| *c != '"').repeated())
    //     .then_ignore(just('"'))
    //     .collect::<String>()
    //     .map(Token::Text);

    // // parser for delimeter
    // let delimeter = newline().repeated()

    // let token = index
    //     .or(timespan)
    //     .or(text_)
    //     .or(delimeter)
    //     .recover_with(skip_then_retry_until([]));

    // let token = delimeter
    //     .or(index)
    //     .recover_with(skip_then_retry_until([]));

    let token = any()
        .repeated()
        .then_ignore(newline())
        .collect::<String>()
        .map(|characters: String| match characters.as_str() {
            _ => Token::Text(characters),
        });

    token
        .map_with_span(|tok, span| (tok, span))
        //.padded()
        .repeated()
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let capabilities = capabilities::server_capabilities(&params.capabilities);

        Ok(InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: option_env!("CARGO_PKG_NAME")
                    .unwrap_or("subtitle-lsp")
                    .to_string(),
                version: Some(option_env!("CARGO_PKG_VERSION").unwrap_or("").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
            language_id: params.text_document.language_id,
        })
        .await
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            let item = TextDocumentItem {
                uri: params.text_document.uri,
                text: &text,
                version: None,
                language_id: None,
            };
            self.on_change(item).await;
            _ = self.client.semantic_tokens_refresh().await;
        }
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

struct TextDocumentItem {
    uri: Url,
    text: String,
    version: Option<i32>,
}

impl Backend {

    pub fn new(client: Client) -> Self {
        Self {
            client
        }
    }

    async fn on_change(&self, _params: TextDocumentItem) {
        self.client
            .log_message(MessageType::INFO, "backend: on change!")
            .await;
        
        // make a rope from &params.text
        // stick it into the document_map
        // attempt to parse &params.text
        //   package up any parse errors
        //   send them back to the client
        // update self.ast_map with whatever ast came back
        // update semantic_token_map with whatever we got there
    }
}
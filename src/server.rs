use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::TcpListener;
use tower_lsp::{
    Client, LanguageServer, LspService, Server, async_trait,
    jsonrpc::Error,
    lsp_types::{
        CompletionOptions, CompletionParams, CompletionResponse, ExecuteCommandOptions,
        ExecuteCommandParams, InitializeParams, InitializeResult, InitializedParams,
        ServerCapabilities, notification::Notification,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct NotificationParams {
    title: String,
    message: String,
    description: String,
}

enum CustomNotification {}

impl Notification for CustomNotification {
    type Params = NotificationParams;

    const METHOD: &'static str = "custom/notification";
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult, Error> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..CompletionOptions::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![String::from("custom.notification")],
                    work_done_progress_options: Default::default(),
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        println!("Initialization successful!");
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>, Error> {
        println!("Asked for completions!! Params were {:?}", params);
        Ok(Some(CompletionResponse::Array(vec![])))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>, Error> {
        if params.command == "banaan" {
            self.client
                .send_notification::<CustomNotification>(NotificationParams {
                    title: String::from("Hello, world!"),
                    message: String::from("This is a test message"),
                    description: String::from(""),
                })
                .await;

            Ok(None)
        } else {
            Err(Error::invalid_request())
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (input, output) = tokio::io::split(stream);

    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(input, output, socket).serve(service).await;
}

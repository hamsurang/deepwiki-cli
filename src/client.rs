use anyhow::{Context, Result};
use rmcp::{
    model::{CallToolRequestParams, RawContent},
    serve_client,
    service::RunningService,
    RoleClient,
};
use serde_json::json;

const MCP_ENDPOINT: &str = "https://mcp.deepwiki.com/mcp";

pub struct DeepWikiClient {
    service: RunningService<RoleClient, ()>,
}

impl DeepWikiClient {
    pub async fn connect() -> Result<Self> {
        let transport =
            rmcp::transport::streamable_http_client::StreamableHttpClientTransport::from_uri(
                MCP_ENDPOINT,
            );
        let service = serve_client((), transport)
            .await
            .context("Failed to connect to mcp.deepwiki.com")?;
        Ok(Self { service })
    }

    pub async fn ask_question(&self, repo: &str, question: &str) -> Result<String> {
        let result = self
            .service
            .peer()
            .call_tool(CallToolRequestParams {
                meta: None,
                name: "ask_question".into(),
                arguments: Some(
                    json!({ "repoName": repo, "question": question })
                        .as_object()
                        .cloned()
                        .unwrap(),
                ),
                task: None,
            })
            .await
            .context("Failed to call ask_question")?;
        Ok(extract_text(result))
    }

    pub async fn read_wiki_structure(&self, repo: &str) -> Result<String> {
        let result = self
            .service
            .peer()
            .call_tool(CallToolRequestParams {
                meta: None,
                name: "read_wiki_structure".into(),
                arguments: Some(
                    json!({ "repoName": repo })
                        .as_object()
                        .cloned()
                        .unwrap(),
                ),
                task: None,
            })
            .await
            .context("Failed to call read_wiki_structure")?;
        Ok(extract_text(result))
    }

    pub async fn read_wiki_contents(&self, repo: &str) -> Result<String> {
        let result = self
            .service
            .peer()
            .call_tool(CallToolRequestParams {
                meta: None,
                name: "read_wiki_contents".into(),
                arguments: Some(
                    json!({ "repoName": repo })
                        .as_object()
                        .cloned()
                        .unwrap(),
                ),
                task: None,
            })
            .await
            .context("Failed to call read_wiki_contents")?;
        Ok(extract_text(result))
    }

    pub async fn cancel(self) -> Result<()> {
        self.service.cancel().await?;
        Ok(())
    }
}

fn extract_text(result: rmcp::model::CallToolResult) -> String {
    result
        .content
        .into_iter()
        .filter_map(|c| {
            if let RawContent::Text(t) = c.raw {
                Some(t.text)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

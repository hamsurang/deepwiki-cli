use anyhow::{Context, Result};
use rmcp::{
    model::{CallToolRequestParams, CallToolResult, RawContent},
    serve_client,
    service::RunningService,
    RoleClient,
};
use serde_json::json;
use std::future::Future;

const MCP_ENDPOINT: &str = "https://mcp.deepwiki.com/mcp";

struct ToolCallSpec {
    name: &'static str,
    arguments: serde_json::Value,
    error_context: &'static str,
}

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
        let spec = ToolCallSpec {
            name: "ask_question",
            arguments: json!({ "repoName": repo, "question": question }),
            error_context: "Failed to call ask_question",
        };
        self.call_tool_text(spec).await
    }

    pub async fn read_wiki_structure(&self, repo: &str) -> Result<String> {
        let spec = ToolCallSpec {
            name: "read_wiki_structure",
            arguments: json!({ "repoName": repo }),
            error_context: "Failed to call read_wiki_structure",
        };
        self.call_tool_text(spec).await
    }

    pub async fn read_wiki_contents(&self, repo: &str) -> Result<String> {
        let spec = ToolCallSpec {
            name: "read_wiki_contents",
            arguments: json!({ "repoName": repo }),
            error_context: "Failed to call read_wiki_contents",
        };
        self.call_tool_text(spec).await
    }

    pub async fn cancel(self) -> Result<()> {
        self.service.cancel().await?;
        Ok(())
    }

    async fn call_tool_text(&self, spec: ToolCallSpec) -> Result<String> {
        call_tool_text_with(spec, |params| async {
            let result = self.service.peer().call_tool(params).await?;
            Ok(extract_text_segments(result))
        })
        .await
    }
}

async fn call_tool_text_with<TCall, TFut>(spec: ToolCallSpec, caller: TCall) -> Result<String>
where
    TCall: FnOnce(CallToolRequestParams) -> TFut,
    TFut: Future<Output = Result<Vec<String>>>,
{
    let params = build_call_tool_request_params(&spec)?;
    let text_segments = caller(params)
        .await
        .with_context(|| spec.error_context.to_string())?;
    Ok(join_text_segments(text_segments))
}

fn build_call_tool_request_params(spec: &ToolCallSpec) -> Result<CallToolRequestParams> {
    let arguments = spec
        .arguments
        .as_object()
        .cloned()
        .context("Tool arguments must be a JSON object")?;
    Ok(CallToolRequestParams {
        meta: None,
        name: spec.name.into(),
        arguments: Some(arguments),
        task: None,
    })
}

fn extract_text_segments(result: CallToolResult) -> Vec<String> {
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
        .collect()
}

fn join_text_segments(text_segments: Vec<String>) -> String {
    text_segments.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn build_params_for_ask_question() {
        let spec = ToolCallSpec {
            name: "ask_question",
            arguments: json!({ "repoName": "facebook/react", "question": "How?" }),
            error_context: "Failed",
        };
        let params = build_call_tool_request_params(&spec).expect("params should be built");

        assert_eq!(params.name, "ask_question");
        let arguments = params.arguments.expect("arguments should exist");
        assert_eq!(
            arguments.get("repoName"),
            Some(&serde_json::Value::String("facebook/react".to_string()))
        );
        assert_eq!(
            arguments.get("question"),
            Some(&serde_json::Value::String("How?".to_string()))
        );
    }

    #[test]
    fn build_params_for_repo_only_tools() {
        for tool_name in ["read_wiki_structure", "read_wiki_contents"] {
            let spec = ToolCallSpec {
                name: tool_name,
                arguments: json!({ "repoName": "owner/repo" }),
                error_context: "Failed",
            };
            let params = build_call_tool_request_params(&spec).expect("params should be built");
            assert_eq!(params.name, tool_name);
            let arguments = params.arguments.expect("arguments should exist");
            assert_eq!(
                arguments.get("repoName"),
                Some(&serde_json::Value::String("owner/repo".to_string()))
            );
        }
    }

    #[tokio::test]
    async fn call_tool_text_with_mocked_caller_joins_lines() {
        let spec = ToolCallSpec {
            name: "ask_question",
            arguments: json!({ "repoName": "owner/repo", "question": "Q" }),
            error_context: "Failed to call ask_question",
        };
        let text = call_tool_text_with(spec, |_params| async {
            Ok(vec!["line1".to_string(), "line2".to_string()])
        })
        .await
        .expect("call should succeed");
        assert_eq!(text, "line1\nline2");
    }

    #[tokio::test]
    async fn call_tool_text_with_mocked_caller_wraps_error_context() {
        let spec = ToolCallSpec {
            name: "read_wiki_structure",
            arguments: json!({ "repoName": "owner/repo" }),
            error_context: "Failed to call read_wiki_structure",
        };
        let err = call_tool_text_with(spec, |_params| async { Err(anyhow!("boom")) })
            .await
            .expect_err("call should fail");
        let err_text = format!("{:#}", err);
        assert!(err_text.contains("Failed to call read_wiki_structure"));
        assert!(err_text.contains("boom"));
    }
}

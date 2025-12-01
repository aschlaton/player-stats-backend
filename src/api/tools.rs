use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;
use tokio_postgres::Client;

use super::boxscores::models::QueryParams;
use super::db::query_boxscores;

#[derive(Debug, Error)]
#[error("Box scores query error: {0}")]
pub struct BoxScoresError(String);

#[derive(Deserialize)]
pub struct GetBoxScoresArgs {
    #[serde(flatten)]
    pub params: QueryParams,
}

pub struct GetBoxScores {
    pub client: Arc<Client>,
}

impl Tool for GetBoxScores {
    const NAME: &'static str = "get_boxscores";

    type Error = BoxScoresError;
    type Args = GetBoxScoresArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Query NBA player box scores with filters, sorting, and pagination".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "pts": {"type": "integer", "description": "Minimum points"},
                    "reb": {"type": "integer", "description": "Minimum rebounds"},
                    "ast": {"type": "integer", "description": "Minimum assists"},
                    "stl": {"type": "integer", "description": "Minimum steals"},
                    "blk": {"type": "integer", "description": "Minimum blocks"},
                    "player": {"type": "string", "description": "Player name (partial match)"},
                    "season": {"type": "string", "description": "Season (e.g., '2023-24')"},
                    "team": {"type": "string", "description": "Team abbreviation"},
                    "limit": {"type": "integer", "description": "Number of results to return"},
                    "offset": {"type": "integer", "description": "Offset for pagination"},
                    "sort_by": {
                        "description": "Field or expression to sort by",
                        "oneOf": [
                            {"type": "string"},
                            {
                                "type": "object",
                                "properties": {
                                    "field": {"type": "string"},
                                    "weight": {"type": "number"}
                                }
                            }
                        ]
                    },
                    "asc": {"type": "boolean", "description": "Sort ascending (default: false)"}
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let response = query_boxscores(&self.client, args.params)
            .await
            .map_err(BoxScoresError)?;
        serde_json::to_string(&response).map_err(|e| BoxScoresError(e.to_string()))
    }
}

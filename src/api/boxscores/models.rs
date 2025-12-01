use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    GameDate,
    Pts,
    Reb,
    Ast,
    Stl,
    Blk,
    Fgm,
    Fga,
    FgPercent,
    ThreePm,
    ThreePa,
    ThreePPercent,
    Ftm,
    Fta,
    FtPercent,
    Oreb,
    Dreb,
    Tov,
    Pf,
    PlusMinus,
    Fp,
    Min,
}

impl SortField {
    pub fn as_sql(&self) -> &str {
        match self {
            SortField::GameDate => "game_date",
            SortField::Pts => "pts",
            SortField::Reb => "reb",
            SortField::Ast => "ast",
            SortField::Stl => "stl",
            SortField::Blk => "blk",
            SortField::Fgm => "fgm",
            SortField::Fga => "fga",
            SortField::FgPercent => "fg_percent",
            SortField::ThreePm => "three_pm",
            SortField::ThreePa => "three_pa",
            SortField::ThreePPercent => "three_p_percent",
            SortField::Ftm => "ftm",
            SortField::Fta => "fta",
            SortField::FtPercent => "ft_percent",
            SortField::Oreb => "oreb",
            SortField::Dreb => "dreb",
            SortField::Tov => "tov",
            SortField::Pf => "pf",
            SortField::PlusMinus => "plus_minus",
            SortField::Fp => "fp",
            SortField::Min => "min",
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
pub enum SortExpression {
    Field(SortField),
    Weighted { field: SortField, weight: f64 },
    Sum { terms: Vec<SortExpression> },
}

impl SortExpression {
    pub fn as_sql(&self) -> String {
        match self {
            SortExpression::Field(f) => f.as_sql().to_string(),
            SortExpression::Weighted { field, weight } => {
                format!("({} * {})", weight, field.as_sql())
            }
            SortExpression::Sum { terms } => {
                let parts: Vec<String> = terms.iter().map(|e| e.as_sql()).collect();
                format!("({})", parts.join(" + "))
            }
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, IntoParams)]
pub struct SortParams {
    pub sort_by: Option<SortExpression>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub asc: Option<bool>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolLike {
        Bool(bool),
        String(String),
    }

    match Option::<BoolLike>::deserialize(deserializer)? {
        Some(BoolLike::Bool(b)) => Ok(Some(b)),
        Some(BoolLike::String(s)) => {
            match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => Ok(Some(true)),
                "false" | "0" | "no" | "" => Ok(Some(false)),
                _ => Ok(None),
            }
        }
        None => Ok(None),
    }
}

#[derive(Serialize, ToSchema)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct QueryParams {
    // Main stats
    pub pts: Option<i32>,
    pub reb: Option<i32>,
    pub ast: Option<i32>,
    pub stl: Option<i32>,
    pub blk: Option<i32>,

    // Additional stats
    pub fgm: Option<i32>,
    pub fga: Option<i32>,
    pub fg_percent: Option<f64>,
    pub three_pm: Option<i32>,
    pub three_pa: Option<i32>,
    pub three_p_percent: Option<f64>,
    pub ftm: Option<i32>,
    pub fta: Option<i32>,
    pub ft_percent: Option<f64>,
    pub oreb: Option<i32>,
    pub dreb: Option<i32>,
    pub tov: Option<i32>,
    pub pf: Option<i32>,
    pub plus_minus: Option<i32>,
    pub fp: Option<f64>,
    pub min: Option<i32>,

    // Meta filters
    pub season: Option<String>,
    pub player: Option<String>,
    pub team: Option<String>,
    pub player_id: Option<String>,
    pub game_id: Option<String>,

    // Pagination
    pub limit: Option<i64>,
    pub offset: Option<i64>,

    // Sorting
    #[serde(flatten)]
    pub sort: SortParams,
}

#[derive(Serialize, ToSchema)]
pub struct BoxScore {
    pub player_id: String,
    pub game_id: String,
    pub team_id: String,
    pub season: String,
    pub player: String,
    pub team: String,
    pub match_up: String,
    pub game_date: String,
    pub w_l: String,
    pub min: Option<i32>,
    pub pts: Option<i32>,
    pub fgm: Option<i32>,
    pub fga: Option<i32>,
    pub fg_percent: Option<f64>,
    pub three_pm: Option<i32>,
    pub three_pa: Option<i32>,
    pub three_p_percent: Option<f64>,
    pub ftm: Option<i32>,
    pub fta: Option<i32>,
    pub ft_percent: Option<f64>,
    pub oreb: Option<i32>,
    pub dreb: Option<i32>,
    pub reb: Option<i32>,
    pub ast: Option<i32>,
    pub stl: Option<i32>,
    pub blk: Option<i32>,
    pub tov: Option<i32>,
    pub pf: Option<i32>,
    pub plus_minus: Option<i32>,
    pub fp: Option<f64>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse {
    pub data: Vec<BoxScore>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

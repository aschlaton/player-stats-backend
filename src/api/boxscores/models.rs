use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct FilterParams {
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

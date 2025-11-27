use axum::{extract::Query, Json};
use tokio_postgres::NoTls;

use super::models::{BoxScore, CountResponse, FilterParams};

#[utoipa::path(
    get,
    path = "/api/boxscores/count",
    responses(
        (status = 200, description = "Get total box score count", body = CountResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_count() -> Result<Json<CountResponse>, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let row = client
        .query_one("SELECT COUNT(*) FROM player_box_scores", &[])
        .await
        .map_err(|e| format!("Query error: {}", e))?;

    let count: i64 = row.get(0);

    Ok(Json(CountResponse { count }))
}

#[utoipa::path(
    get,
    path = "/api/boxscores/filter",
    params(FilterParams),
    responses(
        (status = 200, description = "Filter box scores by stats and metadata", body = Vec<BoxScore>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn filter_boxscores(
    Query(params): Query<FilterParams>,
) -> Result<Json<Vec<BoxScore>>, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not set".to_string())?;

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // build query
    let mut query = String::from("SELECT player_id, game_id, team_id, season, player, team, match_up, game_date, w_l, min, pts, fgm, fga, fg_percent, three_pm, three_pa, three_p_percent, ftm, fta, ft_percent, oreb, dreb, reb, ast, stl, blk, tov, pf, plus_minus, fp FROM player_box_scores WHERE 1=1");
    let mut param_values: Vec<String> = Vec::new();

    // numeric filters
    if let Some(pts) = params.pts {
        query.push_str(&format!(" AND pts >= ${}", param_values.len() + 1));
        param_values.push(pts.to_string());
    }
    if let Some(reb) = params.reb {
        query.push_str(&format!(" AND reb >= ${}", param_values.len() + 1));
        param_values.push(reb.to_string());
    }
    if let Some(ast) = params.ast {
        query.push_str(&format!(" AND ast >= ${}", param_values.len() + 1));
        param_values.push(ast.to_string());
    }
    if let Some(stl) = params.stl {
        query.push_str(&format!(" AND stl >= ${}", param_values.len() + 1));
        param_values.push(stl.to_string());
    }
    if let Some(blk) = params.blk {
        query.push_str(&format!(" AND blk >= ${}", param_values.len() + 1));
        param_values.push(blk.to_string());
    }
    if let Some(fgm) = params.fgm {
        query.push_str(&format!(" AND fgm >= ${}", param_values.len() + 1));
        param_values.push(fgm.to_string());
    }
    if let Some(fga) = params.fga {
        query.push_str(&format!(" AND fga >= ${}", param_values.len() + 1));
        param_values.push(fga.to_string());
    }
    if let Some(fg_percent) = params.fg_percent {
        query.push_str(&format!(" AND fg_percent >= ${}", param_values.len() + 1));
        param_values.push(fg_percent.to_string());
    }
    if let Some(three_pm) = params.three_pm {
        query.push_str(&format!(" AND three_pm >= ${}", param_values.len() + 1));
        param_values.push(three_pm.to_string());
    }
    if let Some(three_pa) = params.three_pa {
        query.push_str(&format!(" AND three_pa >= ${}", param_values.len() + 1));
        param_values.push(three_pa.to_string());
    }
    if let Some(three_p_percent) = params.three_p_percent {
        query.push_str(&format!(" AND three_p_percent >= ${}", param_values.len() + 1));
        param_values.push(three_p_percent.to_string());
    }
    if let Some(ftm) = params.ftm {
        query.push_str(&format!(" AND ftm >= ${}", param_values.len() + 1));
        param_values.push(ftm.to_string());
    }
    if let Some(fta) = params.fta {
        query.push_str(&format!(" AND fta >= ${}", param_values.len() + 1));
        param_values.push(fta.to_string());
    }
    if let Some(ft_percent) = params.ft_percent {
        query.push_str(&format!(" AND ft_percent >= ${}", param_values.len() + 1));
        param_values.push(ft_percent.to_string());
    }
    if let Some(oreb) = params.oreb {
        query.push_str(&format!(" AND oreb >= ${}", param_values.len() + 1));
        param_values.push(oreb.to_string());
    }
    if let Some(dreb) = params.dreb {
        query.push_str(&format!(" AND dreb >= ${}", param_values.len() + 1));
        param_values.push(dreb.to_string());
    }
    if let Some(tov) = params.tov {
        query.push_str(&format!(" AND tov >= ${}", param_values.len() + 1));
        param_values.push(tov.to_string());
    }
    if let Some(pf) = params.pf {
        query.push_str(&format!(" AND pf >= ${}", param_values.len() + 1));
        param_values.push(pf.to_string());
    }
    if let Some(plus_minus) = params.plus_minus {
        query.push_str(&format!(" AND plus_minus >= ${}", param_values.len() + 1));
        param_values.push(plus_minus.to_string());
    }
    if let Some(fp) = params.fp {
        query.push_str(&format!(" AND fp >= ${}", param_values.len() + 1));
        param_values.push(fp.to_string());
    }
    if let Some(min) = params.min {
        query.push_str(&format!(" AND min >= ${}", param_values.len() + 1));
        param_values.push(min.to_string());
    }

    // meta filters
    if let Some(season) = params.season {
        query.push_str(&format!(" AND season = ${}", param_values.len() + 1));
        param_values.push(season);
    }
    if let Some(player) = params.player {
        query.push_str(&format!(" AND player ILIKE ${}", param_values.len() + 1));
        param_values.push(format!("%{}%", player));
    }
    if let Some(team) = params.team {
        query.push_str(&format!(" AND team = ${}", param_values.len() + 1));
        param_values.push(team);
    }
    if let Some(player_id) = params.player_id {
        query.push_str(&format!(" AND player_id = ${}", param_values.len() + 1));
        param_values.push(player_id);
    }
    if let Some(game_id) = params.game_id {
        query.push_str(&format!(" AND game_id = ${}", param_values.len() + 1));
        param_values.push(game_id);
    }

    query.push_str(" ORDER BY game_date DESC LIMIT 1000");

    let params_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = param_values
        .iter()
        .map(|s| s as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();

    let rows = client
        .query(&query, &params_refs)
        .await
        .map_err(|e| format!("Query error: {}", e))?;

    let box_scores: Vec<BoxScore> = rows
        .iter()
        .map(|row| BoxScore {
            player_id: row.get(0),
            game_id: row.get(1),
            team_id: row.get(2),
            season: row.get(3),
            player: row.get(4),
            team: row.get(5),
            match_up: row.get(6),
            game_date: row.get(7),
            w_l: row.get(8),
            min: row.get(9),
            pts: row.get(10),
            fgm: row.get(11),
            fga: row.get(12),
            fg_percent: row.get(13),
            three_pm: row.get(14),
            three_pa: row.get(15),
            three_p_percent: row.get(16),
            ftm: row.get(17),
            fta: row.get(18),
            ft_percent: row.get(19),
            oreb: row.get(20),
            dreb: row.get(21),
            reb: row.get(22),
            ast: row.get(23),
            stl: row.get(24),
            blk: row.get(25),
            tov: row.get(26),
            pf: row.get(27),
            plus_minus: row.get(28),
            fp: row.get(29),
        })
        .collect();

    Ok(Json(box_scores))
}

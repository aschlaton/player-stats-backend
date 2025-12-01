use tokio_postgres::Client;

use super::boxscores::models::{BoxScore, QueryParams, PaginatedResponse};

pub async fn query_boxscores(
    client: &Client,
    params: QueryParams,
) -> Result<PaginatedResponse, String> {
    let mut query = String::from("SELECT player_id, game_id, team_id, season, player, team, match_up, game_date, w_l, min, pts, fgm, fga, fg_percent, three_pm, three_pa, three_p_percent, ftm, fta, ft_percent, oreb, dreb, reb, ast, stl, blk, tov, pf, plus_minus, fp FROM player_box_scores WHERE 1=1");

    if let Some(pts) = params.pts {
        query.push_str(&format!(" AND pts >= {}", pts));
    }
    if let Some(reb) = params.reb {
        query.push_str(&format!(" AND reb >= {}", reb));
    }
    if let Some(ast) = params.ast {
        query.push_str(&format!(" AND ast >= {}", ast));
    }
    if let Some(stl) = params.stl {
        query.push_str(&format!(" AND stl >= {}", stl));
    }
    if let Some(blk) = params.blk {
        query.push_str(&format!(" AND blk >= {}", blk));
    }
    if let Some(fgm) = params.fgm {
        query.push_str(&format!(" AND fgm >= {}", fgm));
    }
    if let Some(fga) = params.fga {
        query.push_str(&format!(" AND fga >= {}", fga));
    }
    if let Some(fg_percent) = params.fg_percent {
        query.push_str(&format!(" AND fg_percent >= {}", fg_percent));
    }
    if let Some(three_pm) = params.three_pm {
        query.push_str(&format!(" AND three_pm >= {}", three_pm));
    }
    if let Some(three_pa) = params.three_pa {
        query.push_str(&format!(" AND three_pa >= {}", three_pa));
    }
    if let Some(three_p_percent) = params.three_p_percent {
        query.push_str(&format!(" AND three_p_percent >= {}", three_p_percent));
    }
    if let Some(ftm) = params.ftm {
        query.push_str(&format!(" AND ftm >= {}", ftm));
    }
    if let Some(fta) = params.fta {
        query.push_str(&format!(" AND fta >= {}", fta));
    }
    if let Some(ft_percent) = params.ft_percent {
        query.push_str(&format!(" AND ft_percent >= {}", ft_percent));
    }
    if let Some(oreb) = params.oreb {
        query.push_str(&format!(" AND oreb >= {}", oreb));
    }
    if let Some(dreb) = params.dreb {
        query.push_str(&format!(" AND dreb >= {}", dreb));
    }
    if let Some(tov) = params.tov {
        query.push_str(&format!(" AND tov >= {}", tov));
    }
    if let Some(pf) = params.pf {
        query.push_str(&format!(" AND pf >= {}", pf));
    }
    if let Some(plus_minus) = params.plus_minus {
        query.push_str(&format!(" AND plus_minus >= {}", plus_minus));
    }
    if let Some(fp) = params.fp {
        query.push_str(&format!(" AND fp >= {}", fp));
    }
    if let Some(min) = params.min {
        query.push_str(&format!(" AND min >= {}", min));
    }

    if let Some(ref season) = params.season {
        query.push_str(&format!(" AND season = '{}'", season.replace("'", "''")));
    }
    if let Some(ref player) = params.player {
        query.push_str(&format!(" AND player ILIKE '%{}%'", player.replace("'", "''")));
    }
    if let Some(ref team) = params.team {
        query.push_str(&format!(" AND team = '{}'", team.replace("'", "''")));
    }
    if let Some(ref player_id) = params.player_id {
        query.push_str(&format!(" AND player_id = '{}'", player_id.replace("'", "''")));
    }
    if let Some(ref game_id) = params.game_id {
        query.push_str(&format!(" AND game_id = '{}'", game_id.replace("'", "''")));
    }

    let count_query = query.replace(
        "SELECT player_id, game_id, team_id, season, player, team, match_up, game_date, w_l, min, pts, fgm, fga, fg_percent, three_pm, three_pa, three_p_percent, ftm, fta, ft_percent, oreb, dreb, reb, ast, stl, blk, tov, pf, plus_minus, fp FROM player_box_scores",
        "SELECT COUNT(*) FROM player_box_scores"
    );

    let count_row = client
        .query_one(&count_query, &[])
        .await
        .map_err(|e| format!("Count query error: {}", e))?;

    let total: i64 = count_row.get(0);

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let sort_sql = params.sort.sort_by
        .as_ref()
        .map(|expr| expr.as_sql())
        .unwrap_or_else(|| "game_date".to_string());

    let order = if params.sort.asc.unwrap_or(false) { "ASC" } else { "DESC" };

    query.push_str(&format!(" ORDER BY {} {} LIMIT {} OFFSET {}", sort_sql, order, limit, offset));

    let rows = client
        .query(&query, &[])
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

    Ok(PaginatedResponse {
        data: box_scores,
        total,
        limit,
        offset,
    })
}

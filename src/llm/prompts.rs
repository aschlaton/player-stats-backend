pub const QUERY_PROMPT: &str =
    "You are an assistant that converts a user's natural language query into structured NBA box score filter parameters.

CRITICAL: Only include parameters that are EXPLICITLY mentioned in the user's query. Do NOT infer or add parameters that are not requested.

Your JSON output will be processed by this code:

```rust
if let Some(pts) = params.pts {
    query.push_str(&format!(\" AND pts >= {}\", pts));
}
if let Some(reb) = params.reb {
    query.push_str(&format!(\" AND reb >= {}\", reb));
}
if let Some(ast) = params.ast {
    query.push_str(&format!(\" AND ast >= {}\", ast));
}
if let Some(stl) = params.stl {
    query.push_str(&format!(\" AND stl >= {}\", stl));
}
if let Some(blk) = params.blk {
    query.push_str(&format!(\" AND blk >= {}\", blk));
}
if let Some(fgm) = params.fgm {
    query.push_str(&format!(\" AND fgm >= {}\", fgm));
}
if let Some(fga) = params.fga {
    query.push_str(&format!(\" AND fga >= {}\", fga));
}
if let Some(fg_percent) = params.fg_percent {
    query.push_str(&format!(\" AND fg_percent >= {}\", fg_percent));
}
if let Some(three_pm) = params.three_pm {
    query.push_str(&format!(\" AND three_pm >= {}\", three_pm));
}
if let Some(three_pa) = params.three_pa {
    query.push_str(&format!(\" AND three_pa >= {}\", three_pa));
}
if let Some(three_p_percent) = params.three_p_percent {
    query.push_str(&format!(\" AND three_p_percent >= {}\", three_p_percent));
}
if let Some(ftm) = params.ftm {
    query.push_str(&format!(\" AND ftm >= {}\", ftm));
}
if let Some(fta) = params.fta {
    query.push_str(&format!(\" AND fta >= {}\", fta));
}
if let Some(ft_percent) = params.ft_percent {
    query.push_str(&format!(\" AND ft_percent >= {}\", ft_percent));
}
if let Some(oreb) = params.oreb {
    query.push_str(&format!(\" AND oreb >= {}\", oreb));
}
if let Some(dreb) = params.dreb {
    query.push_str(&format!(\" AND dreb >= {}\", dreb));
}
if let Some(tov) = params.tov {
    query.push_str(&format!(\" AND tov >= {}\", tov));
}
if let Some(pf) = params.pf {
    query.push_str(&format!(\" AND pf >= {}\", pf));
}
if let Some(plus_minus) = params.plus_minus {
    query.push_str(&format!(\" AND plus_minus >= {}\", plus_minus));
}
if let Some(fp) = params.fp {
    query.push_str(&format!(\" AND fp >= {}\", fp));
}
if let Some(min) = params.min {
    query.push_str(&format!(\" AND min >= {}\", min));
}

if let Some(ref season) = params.season {
    query.push_str(&format!(\" AND season = '{}'\", season.replace(\"'\", \"''\")));
}
if let Some(ref player) = params.player {
    query.push_str(&format!(\" AND player ILIKE '%{}%'\", player.replace(\"'\", \"''\")));
}
if let Some(ref team) = params.team {
    query.push_str(&format!(\" AND team = '{}'\", team.replace(\"'\", \"''\")));
}
if let Some(ref player_id) = params.player_id {
    query.push_str(&format!(\" AND player_id = '{}'\", player_id.replace(\"'\", \"''\")));
}
if let Some(ref game_id) = params.game_id {
    query.push_str(&format!(\" AND game_id = '{}'\", game_id.replace(\"'\", \"''\")));
}

let limit = params.limit.unwrap_or(50);
let offset = params.offset.unwrap_or(0);

let sort_sql = params.sort.sort_by
    .as_ref()
    .map(|expr| expr.as_sql())
    .unwrap_or_else(|| \"game_date\".to_string());

let order = if params.sort.asc.unwrap_or(false) { \"ASC\" } else { \"DESC\" };

query.push_str(&format!(\" ORDER BY {} {} LIMIT {} OFFSET {}\", sort_sql, order, limit, offset));
```

Examples:
'LeBron James highest scoring game' → {\"reasoning\": \"User wants LeBron's highest scoring game, limit 1, sort by pts desc\", \"player\": \"LeBron James\", \"limit\": 1, \"sort_by\": \"pts\", \"asc\": false}
'top 5 games with 30+ points' → {\"reasoning\": \"Top 5 games with minimum 30 points\", \"pts\": 30, \"limit\": 5, \"sort_by\": \"pts\", \"asc\": false}
'show me 2 LeBron games' → {\"reasoning\": \"2 games by LeBron, no filters\", \"player\": \"LeBron\", \"limit\": 2}";

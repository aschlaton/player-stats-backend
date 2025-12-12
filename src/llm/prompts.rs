pub const SQL_PROMPT: &str = "You are a SQL expert. Convert the user's natural language query into a PostgreSQL SELECT query for the player_box_scores_view view.

View schema:
- player_id VARCHAR(50)
- game_id VARCHAR(50)
- team_id VARCHAR(50)
- season VARCHAR(20)
- player VARCHAR(255)
- team VARCHAR(100)
- match_up VARCHAR(100)
- game_date VARCHAR(50)
- w_l VARCHAR(10)
- min INTEGER (minutes played)
- pts INTEGER (points)
- fgm INTEGER (field goals made)
- fga INTEGER (field goals attempted)
- fg_percent DOUBLE PRECISION (field goal percentage, stored as percentage e.g. 45.5 not 0.455)
- three_pm INTEGER (3-pointers made)
- three_pa INTEGER (3-pointers attempted)
- three_p_percent DOUBLE PRECISION (three point percentage, stored as percentage e.g. 38.2 not 0.382)
- ftm INTEGER (free throws made)
- fta INTEGER (free throws attempted)
- ft_percent DOUBLE PRECISION (free throw percentage, stored as percentage e.g. 87.5 not 0.875)
- oreb INTEGER (offensive rebounds)
- dreb INTEGER (defensive rebounds)
- reb INTEGER (total rebounds)
- ast INTEGER (assists)
- stl INTEGER (steals)
- blk INTEGER (blocks)
- tov INTEGER (turnovers)
- pf INTEGER (personal fouls)
- plus_minus INTEGER
- fp DOUBLE PRECISION (fantasy points)

Examples:
1. \"LeBron's highest scoring games\" → SELECT * FROM player_box_scores_view WHERE player ILIKE '%LeBron%' ORDER BY pts DESC LIMIT 10

2. \"best offensive games of Stephen Curry\" → SELECT *, (pts + (ast * 1.5) + (oreb * 2) + (fg_percent * 0.5)) as offensive_score FROM player_box_scores_view WHERE player ILIKE '%Curry%' ORDER BY offensive_score DESC LIMIT 10

3. \"most efficient shooting performances with at least 20 points\" → SELECT player, game_date, pts, fg_percent, three_p_percent, (fg_percent + three_p_percent) / 2 as shooting_efficiency FROM player_box_scores_view WHERE pts >= 20 ORDER BY shooting_efficiency DESC LIMIT 15

4. \"best defensive games\" → SELECT *, (stl + blk + dreb) as defensive_score FROM player_box_scores_view ORDER BY defensive_score DESC LIMIT 10

5. \"triple doubles\" → SELECT * FROM player_box_scores_view WHERE pts >= 10 AND reb >= 10 AND ast >= 10 ORDER BY game_date DESC

Use your best judgment to create composite scores for subjective terms like 'best offensive game', 'most dominant performance', etc. by combining relevant stats with appropriate weights.

Return ONLY the SQL query, no explanation or markdown formatting.";

pub const QUERY_PROMPT: &str =
    "You are an assistant that converts a user's natural language query into structured NBA box score filter parameters.

CRITICAL: Only include parameters that are EXPLICITLY mentioned in the user's query. Do NOT infer or add parameters that are not requested.

DO infer a player's full name from the query if the user provides a partial name, nickname, or typo (i.e. AD -> Anthony Davis, LBJ -> LeBron James, KD -> Kevin Durant, etc.)

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

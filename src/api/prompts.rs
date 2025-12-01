pub const QUERY_PROMPT: &str =
    "Convert the user's natural language query into structured NBA box score filter parameters.

Instructions - only include parameters when specified:
- player: Include when asking for a specific player. Use full name (e.g., 'LeBron James').
- pts: Include when user specifies minimum points (e.g., '30+ points' → pts: 30, '40 points' → pts: 40).
- reb: Include when user specifies minimum rebounds (e.g., '10+ rebounds' → reb: 10).
- ast: Include when user specifies minimum assists (e.g., '5+ assists' → ast: 5, '10 assists' → ast: 10).
- stl: Include when user specifies minimum steals (e.g., '3+ steals' → stl: 3).
- blk: Include when user specifies minimum blocks (e.g., '2+ blocks' → blk: 2).
- team: Include when user mentions a team. Use abbreviation ('Lakers' → 'LAL').
- season: Include when user specifies a season (format: '2024-25').
- limit: Include when user wants specific number of results ('top 5' → 5, 'highest' → 1).
- sort_by: Include when query implies ranking. Use stat field: 'pts', 'reb', 'ast', 'stl', 'blk'.
- asc: Include when sort_by is set. false = descending (highest/top), true = ascending (lowest).

Examples:
- 'LeBron James highest scoring game' → {\"player\": \"LeBron James\", \"limit\": 1, \"sort_by\": \"pts\", \"asc\": false}
- 'top 5 games with 30+ points' → {\"pts\": 30, \"limit\": 5, \"sort_by\": \"pts\", \"asc\": false}
- 'show me 10 Steph Curry games' → {\"player\": \"Stephen Curry\", \"limit\": 10}";

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TeamState {
    pub description: String,
    pub team_a_name: String,
    pub team_b_name: String,
    pub team_a: i32,
    pub team_b: i32,
}

impl Default for TeamState {
    fn default() -> Self {
        Self {
            description: "Competition - Stage x".to_string(),
            team_a_name: "Team A".to_string(),
            team_b_name: "Team B".to_string(),
            team_a: 0,
            team_b: 0,
        }
    }
}

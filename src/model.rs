use serde::{Deserialize, Serialize};

/// Status of a map in a pick/ban line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapStatus {
    /// Not selected or unaffected
    None,
    /// This map is banned
    Banned,
    /// This map is picked
    Picked,
}

/// Winner assigned to a map/mode slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotWinner {
    None,
    TeamA,
    TeamB,
}

/// A single map entry with optional map name and its status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEntry {
    pub map: Option<String>,
    pub status: MapStatus,
}

/// A named mode (column) containing multiple `MapEntry` items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeLine {
    pub name: String,
    pub maps: Vec<MapEntry>,
}

/// The complete state used by the UI and bridge publishing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamState {
    pub description: String,
    pub commentator_a: String,
    pub commentator_b: String,
    pub team_a_name: String,
    pub team_b_name: String,
    pub team_a_full: String,
    pub team_a_trunc: String,
    pub team_a_dir: String,
    pub team_b_full: String,
    pub team_b_trunc: String,
    pub team_b_dir: String,
    pub team_a: i32,
    pub team_b: i32,
    pub map_mode_slots: Vec<(Option<String>, Option<String>)>,
    #[serde(default = "default_slot_winners")]
    pub slot_winners: Vec<SlotWinner>,
    pub mode_lines: Vec<ModeLine>,
    pub selected_slot: Option<usize>,
}

fn default_slot_winners() -> Vec<SlotWinner> {
    vec![SlotWinner::None; 9]
}

impl TeamState {
    /// Keep `slot_winners` aligned with `map_mode_slots` length.
    pub fn ensure_slot_winners_len(&mut self) {
        let expected_len = self.map_mode_slots.len();
        if self.slot_winners.len() < expected_len {
            self.slot_winners.resize(expected_len, SlotWinner::None);
        } else if self.slot_winners.len() > expected_len {
            self.slot_winners.truncate(expected_len);
        }
    }

    /// Recompute team scores from recorded map winners.
    pub fn sync_scores_from_slot_winners(&mut self) {
        self.ensure_slot_winners_len();

        let team_a_wins = self
            .slot_winners
            .iter()
            .filter(|winner| matches!(winner, SlotWinner::TeamA))
            .count() as i32;
        let team_b_wins = self
            .slot_winners
            .iter()
            .filter(|winner| matches!(winner, SlotWinner::TeamB))
            .count() as i32;

        self.team_a = team_a_wins;
        self.team_b = team_b_wins;
    }
}

impl Default for TeamState {
    fn default() -> Self {
        Self {
            description: "Competition - Stage X".to_string(),
            commentator_a: String::new(),
            commentator_b: String::new(),
            team_a_name: "Team A".to_string(),
            team_a_full: "Team A".to_string(),
            team_a_trunc: "Team A".to_string(),
            team_a_dir: String::new(),
            team_b_name: "Team B".to_string(),
            team_b_full: "Team B".to_string(),
            team_b_trunc: "Team B".to_string(),
            team_b_dir: String::new(),
            team_a: 0,
            team_b: 0,
            map_mode_slots: vec![(Some(String::new()), Some(String::new())); 9],
            slot_winners: default_slot_winners(),
            mode_lines: vec![
                ModeLine {
                    name: "Splat Zones".to_string(),
                    maps: vec![
                        MapEntry {
                            map: None,
                            status: MapStatus::None
                        };
                        8
                    ],
                },
                ModeLine {
                    name: "Tower Control".to_string(),
                    maps: vec![
                        MapEntry {
                            map: None,
                            status: MapStatus::None
                        };
                        8
                    ],
                },
                ModeLine {
                    name: "Clam Blitz".to_string(),
                    maps: vec![
                        MapEntry {
                            map: None,
                            status: MapStatus::None
                        };
                        8
                    ],
                },
                ModeLine {
                    name: "Rainmaker".to_string(),
                    maps: vec![
                        MapEntry {
                            map: None,
                            status: MapStatus::None
                        };
                        8
                    ],
                },
            ],
            selected_slot: None,
        }
    }
}

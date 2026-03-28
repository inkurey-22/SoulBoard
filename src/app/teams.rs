/// Helper functions to load team display names from the `assets/teams` directory.
///
/// These are used by the UI update logic to populate full/truncated team
/// names when a team directory is selected.
pub fn load_team_names(sel: &str) -> Option<(String, String)> {
    if sel.is_empty() {
        return None;
    }

    let dir = format!("assets/teams/{}", sel);

    let full = std::fs::read_to_string(format!("{}/nameFull.txt", dir))
        .unwrap_or_else(|_| sel.to_string())
        .trim()
        .to_string();

    let trunc = std::fs::read_to_string(format!("{}/nameTrunc.txt", dir))
        .unwrap_or_else(|_| sel.to_string())
        .trim()
        .to_string();

    Some((full, trunc))
}

/// Helper functions to load team display names from the `assets/teams` directory.
///
/// These are used by the UI update logic to populate full/truncated team
/// names when a team directory is selected.
use std::path::Path;

pub fn list_team_dirs() -> Vec<String> {
    let mut teams = vec![String::new()];

    if let Ok(entries) = std::fs::read_dir("assets/teams") {
        for entry in entries.flatten() {
            if entry.path().is_dir()
                && let Some(name) = entry.file_name().to_str()
            {
                teams.push(name.to_string());
            }
        }
    }

    teams
}

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

pub fn create_team(full_name: &str, trunc_name: &str, logo_src: Option<&Path>) -> Result<(), String> {
    let full_name = full_name.trim();
    let trunc_name = trunc_name.trim();

    if full_name.is_empty() {
        return Err("Full name is required".to_string());
    }
    if trunc_name.is_empty() {
        return Err("Short name is required".to_string());
    }
    if full_name.contains('/') || full_name.contains('\\') {
        return Err("Full name cannot contain path separators".to_string());
    }

    let team_dir = Path::new("assets/teams").join(full_name);
    if team_dir.exists() {
        return Err("A team with this full name already exists".to_string());
    }

    std::fs::create_dir_all(&team_dir)
        .map_err(|err| format!("Failed to create team directory: {}", err))?;

    if let Err(err) = std::fs::write(team_dir.join("nameFull.txt"), full_name) {
        let _ = std::fs::remove_dir_all(&team_dir);
        return Err(format!("Failed to write nameFull.txt: {}", err));
    }

    if let Err(err) = std::fs::write(team_dir.join("nameTrunc.txt"), trunc_name) {
        let _ = std::fs::remove_dir_all(&team_dir);
        return Err(format!("Failed to write nameTrunc.txt: {}", err));
    }

    let Some(logo_src) = logo_src else {
        let _ = std::fs::remove_dir_all(&team_dir);
        return Err("A logo file is required".to_string());
    };

    let image = image::open(logo_src)
        .map_err(|err| format!("Failed to read logo file: {}", err))?;
    image
        .save_with_format(team_dir.join("logo.png"), image::ImageFormat::Png)
        .map_err(|err| format!("Failed to save logo.png: {}", err))?;

    Ok(())
}

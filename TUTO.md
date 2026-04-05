# SoulBoard Tutorial

This document explains how to prepare your files, run SoulBoard, and consume its live data in your overlay pages.

## Table of Contents

- [1. Quick Start](#1-quick-start)
- [2. Required Folder Layout](#2-required-folder-layout)
- [3. Run and Persistence](#3-run-and-persistence)
- [4. Bridge Endpoints](#4-bridge-endpoints)
- [5. Payload Fields](#5-payload-fields)
- [6. Overlay Example (WebSocket)](#6-overlay-example-websocket)
- [7. Suggested Project Layout for Broadcast Packages](#7-suggested-project-layout-for-broadcast-packages)
- [8. Troubleshooting](#8-troubleshooting)
- [9. Practical Integration Notes](#9-practical-integration-notes)

## 1. Quick Start

1. Put `assets/` next to the SoulBoard executable.
2. Ensure `assets/maps`, `assets/modes`, and `assets/teams` exist.
3. Run the SoulBoard executable (`soulboard` on Linux/macOS or `soulboard.exe` on Windows).
4. Connect your overlay HTML to `ws://127.0.0.1:7878/ws`.

SoulBoard does not render overlays itself. It only provides state to external clients.

## 2. Required Folder Layout

SoulBoard scans `assets/maps`, `assets/modes`, and `assets/teams` at startup.

- Map list comes from file names in `assets/maps`.
- Mode list comes from file names in `assets/modes`.
- Team list comes from directory names in `assets/teams`.

Example:

```tree
assets/
├── maps/
│   ├── Casse Rascasse.png
│   ├── Galeries Guppy.png
│   └── ...
├── modes/
│   ├── Clam Blitz.png
│   ├── Rainmaker.png
│   └── ...
└── teams/
        ├── IKS Sina/
        ├── Team Alpha/
        └── ...
```

Team folder format:

```tree
assets/teams/IKS Sina/
├── logo.png
├── nameFull.txt
└── nameTrunc.txt
```

Important notes:

- `logo.png` must be exactly that file name.
- `nameFull.txt` and `nameTrunc.txt` are optional but recommended.
- If `nameFull.txt` or `nameTrunc.txt` is missing, SoulBoard falls back to the team directory name.

## 3. Run and Persistence

### Running SoulBoard

Simply run the SoulBoard executable from the command line:

```bash
# Linux/macOS
./soulboard

# Windows
soulboard.exe
```

Make sure `assets/` and `exports/` are in the same directory as the executable.

### State file

SoulBoard saves UI state to `soulboard_state.json` after every update. On next launch, it tries to load this file automatically.

## 4. Bridge Endpoints

SoulBoard opens a local bridge on `127.0.0.1:7878` with:

- WebSocket: `ws://127.0.0.1:7878/ws`
- HTTP snapshot: `http://127.0.0.1:7878/score`

Use WebSocket for live updates. Use HTTP when you only need a one-time fetch.

## 5. Payload Fields

The bridge sends JSON derived from app state.

Common top-level fields:

- `description`
- `commentator_a`, `commentator_b`
- `team_a`, `team_b` (scores)
- `team_a_name`, `team_b_name` (typically truncated names)
- `team_a_full`, `team_b_full`
- `team_a_trunc`, `team_b_trunc`
- `team_a_dir`, `team_b_dir` (team folder names)
- `mode_lines` (pick/ban grid data)
- `map`, `mode` (current map/mode resolved from selected slot logic)

Notes about `map` and `mode`:

- They are resolved from the slot marked as "Use" in the UI.
- If no slot is marked, SoulBoard falls back to the first non-empty slot.
- Internal fields `map_mode_slots` and `selected_slot` are not included in bridge payload.

## 6. Overlay Example (WebSocket)

```js
function connect() {
    const ws = new WebSocket("ws://127.0.0.1:7878/ws");

    ws.onmessage = (event) => {
        try {
            const payload = JSON.parse(event.data);
            teamANameEl.textContent = String(payload.team_a_full ?? "Team A");
            teamBNameEl.textContent = String(payload.team_b_full ?? "Team B");

            // Commentators (new fields)
            if (commentatorAEl) {
                commentatorAEl.textContent = String(payload.commentator_a ?? "");
            }
            if (commentatorBEl) {
                commentatorBEl.textContent = String(payload.commentator_b ?? "");
            }

            // Team logos: expect a folder per team with a `logo.png` inside
            const teamAName = payload.team_a_name ?? null;
            const teamBName = payload.team_b_name ?? null;

            const logoA = teamLogoPath(teamAName);
            if (logoA) {
                teamALogoEl.src = logoA;
                teamALogoEl.style.display = 'block';
            } else {
                teamALogoEl.removeAttribute('src');
                teamALogoEl.style.display = 'none';
            }

            const logoB = teamLogoPath(teamBName);
            if (logoB) {
                teamBLogoEl.src = logoB;
                teamBLogoEl.style.display = 'block';
            } else {
                teamBLogoEl.removeAttribute('src');
                teamBLogoEl.style.display = 'none';
            }
            // adjust fonts after updating text
            setTimeout(adjustAll, 0);
        } catch (_error) {
            // Ignore malformed packets
        }
    };

    ws.onclose = () => setTimeout(connect, 1000);
    ws.onerror = () => ws.close();
}

connect();
```

## 7. Suggested Project Layout for Broadcast Packages

```tree
.
├── assets/
│   ├── images/
│   ├── maps/
│   ├── modes/
│   └── teams/
├── exports/
│   ├── start.html
│   ├── ingame.html
│   └── ...
├── soulboard(.exe)
└── soulboard_state.json
```

This keeps asset paths and overlay paths stable.

## 8. Troubleshooting

- No maps/modes/teams in dropdowns:
    - Verify `assets/` is next to the executable.
    - Verify folder names are exactly `maps`, `modes`, `teams`.
- Team names do not look right:
    - Check `nameFull.txt` and `nameTrunc.txt` encoding/content.
    - Ensure there is no trailing whitespace (SoulBoard trims values, but keep files clean).
- No updates in overlay:
    - Confirm SoulBoard is running.
    - Confirm URL is `ws://127.0.0.1:7878/ws`.
    - Inspect browser devtools for WebSocket errors.
- Logo missing:
    - Confirm `assets/teams/<Team>/logo.png` exists.
    - Ensure your `teamLogoPath(...)` logic matches the incoming team field you use.

## 9. Practical Integration Notes

- Prefer `team_a_full` / `team_b_full` for title cards.
- Prefer `team_a_trunc` / `team_b_trunc` for cramped widgets.
- Keep your overlay JS tolerant to missing fields (`??` fallbacks).
- Reconnect automatically on socket close, as shown above.

That is enough to build stable, scene-specific HTML overlays while using SoulBoard as the live state source.

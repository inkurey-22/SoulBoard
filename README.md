# Soulboard

Soulboard is a simple desktop dashboard app built with Rust and Iced.
Its aim is to provide a simple interface for Splatoon Commentators and Streamers to manage their overlay informations (score, teams, description, map/modes...).

It works with a simple Rust app to input informations. The app then shares the state with external HTML/overlay clients through a WebSocket connection at `ws://localhost:7878`.

## What it does now

- Track Team A and Team B scores
- Edit team names and a match description
- Reset scores
- Manage map pool with pick and ban
- Share live state with external HTML/overlay clients

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
```

## Disclaimer
I am not exposing assets here (such as font and especially overlay images) since they are property of the Ink Souls Association and not open-source. Please use your own assets (and modify paths as needed).

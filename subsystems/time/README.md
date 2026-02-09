# Time Subsystem

Standalone backend + frontend subsystem for game time progression.

## Goals
- Represent time as `day + minute + lightzone`.
- Progress time only from frontend-issued commands.
- Support named action costs (initially `craft totem = 40`).
- Provide two clock visualizations side-by-side.

## Time Model
- 24-hour day.
- 1 game minute = 1 tick.
- `day: usize`.
- `minute: u16` in `0..=1439`.

## Lightzones
- `Morning` (light blue): 04:00-11:59 (`240..=719`)
- `Afternoon` (yellow/orange): 12:00-19:59 (`720..=1199`)
- `Night` (shadow purple): 20:00-03:59 (`1200..=1439` and `0..=239`)

## API
- `GET /api/state`
  - Returns current timestamp + angles + operation metadata.
- `POST /api/tick` with body `{ "minutes": <u32> }`
  - Advances by explicit minute delta.
- `POST /api/command` with body `{ "command": "<string>" }`
  - Supports `tick <n>` and named actions.
- `GET /api/actions`
  - Returns available action commands and their minute costs.

## Command Examples
- `tick 1`
- `tick 60`
- `craft totem`

## Clock Behavior

### 15 degrees per hour clock
- Static 24-hour dial.
- Moving hand rotates by `minute_of_day * 0.25` degrees.

### 45 degrees per hour clock
- Static bottom arrow points upward.
- Rotating dial spins clockwise by `minute_of_day * 0.75` degrees.
- Top wedge mask hides the seam between past (left) and future (right).
- Backend returns both total angle (`0..1080` over full day) and visual angle (`mod 360`).

## Running
```bash
cd subsystems/time
cargo run --bin time-subsystem
```
Open `http://127.0.0.1:3001`.

## Testing
```bash
cd subsystems/time
cargo test
```

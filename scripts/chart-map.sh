#!/usr/bin/env bash
# Charts the wayfinder map for intportal-app: one map issue, child tickets as
# native sub-issues, and blocked-by edges. Run once, inside the repo clone.
set -euo pipefail
repo=$(gh repo view --json nameWithOwner --jq .nameWithOwner)

for l in wayfinder:map wayfinder:grilling wayfinder:research wayfinder:prototype wayfinder:task; do
  gh label create "$l" --force --color 5a1a1a >/dev/null
done

dbid() { gh api "repos/$repo/issues/$1" --jq .id; }
issue() { # title label body -> number
  gh issue create --title "$1" --label "$2" --body "$3" | grep -oE '[0-9]+$'
}

map=$(issue "Map: IntPortal v1 on phones" wayfinder:map "$(cat <<'EOF'
Destination: v1 of IntPortal on a real phone. Sign-in works, the week shows real classes, the island counts down to the next class, grades show a correct GPA, a reminder fires.

## Notes
Tauri 2 + Rust core + Svelte 5 (ADR-0001). Data on the device only (ADR-0002). Glossary in CONTEXT.md. The old macOS app's docs are in docs/legacy/ and are the reference for every feature. The head builds alone and reviews once per part PR.

## Decisions so far
- Rewrite direction (grilling, 2026-09-26): see the closed grilling ticket.

## Fog (charted later, after v1)
Notes (PUPNotes port). Syllabus, quizzes and study. IntAssis on the device (llama.cpp). Widgets and Live Activity. Calendar export. Desktop parity and retiring the old Mac app. Cloud subscription tier and sync.
EOF
)")

g=$(issue "Rewrite direction" wayfinder:grilling "Part of #$map

Settled 2026-09-26:
1. Stack: Tauri 2, Rust core, Svelte 5.
2. Look: even 50/50 hybrid. Pixel frames and notched corners, clean legacy type inside. Pixelify for display only; the Legibility Rule carries over.
3. Port the landing and hub, the custom island, the IntAssis deck and AI, and all 15 rooms.
4. Moments: living campus landing, sync as a jeepney journey, grade reveal stamps, pixel empty states.
5. v1: sign-in, schedule (week and today), island glance, grades and GPA, reminders, landing, rooms, moments.
6. AI on the device first, cloud subscription later.
7. Data on the device only; sync with the cloud tier.
8. Island in the app for v1; widgets and Live Activity later.
9. No store accounts yet; emulator, simulator and sideloading until the beta.
10. Private repo until the beta; desktop builds in CI but doesn't ship.")
gh issue close "$g" >/dev/null

p0=$(issue "P0: repo, docs and CI" wayfinder:task "Part of #$map

Copy the legacy docs into docs/legacy/, write CONTEXT.md and ADR-0001/0002, set up CI (cargo test, svelte-check, vitest, Tauri desktop build).")
s0=$(issue "S0: SIS sign-in over plain HTTP from Rust" wayfinder:prototype "Part of #$map

spikes/s0-sis-http. The student runs it with their own credentials. Answer: does /schedule return rows after a plain HTTP form POST? If yes, the core owns SIS access. If no, a small Tauri plugin hosts a hidden native webview and the core still parses the HTML.")
s1=$(issue "S1: Tauri 2 + Svelte shell on Android and iOS" wayfinder:prototype "Part of #$map

Measure the landing canvas scene at 60 fps, store a secret in the Keychain or Keystore, and fire a local notification, on the Android emulator and the iOS simulator.")
api=$(issue "Core API design (architect)" wayfinder:research "Part of #$map

Run /pstack:architect on the boundary between the core and the UI: Tauri commands and events, domain types, errors.")
core=$(issue "Core crate: types, parsers, SIS, logic, SQLite" wayfinder:task "Part of #$map

ClassSession, Term, GradeReport, Campus, Mark. Parsers ported from Swift, with the old fixtures as literal expected values. SISHost failover. GPA, NextClass, DayAgenda, IslandGlance (5 cases). SQLite store.")
ds=$(issue "Design system: rooms, hybrid pixel components, gallery" wayfinder:task "Part of #$map

Tokens ported from Palette.Roles.derived with contrast clamps for all 15 rooms. NotchFrame, Stamp, PixelButton, Dither. Fonts, the Legibility Rule, a Reduce Motion store. A gallery route for review.")
shell=$(issue "Shell prototype: island, navigation, sign-in, campus landing" wayfinder:prototype "Part of #$map

The real Svelte shell with demo data. HITL: the student reviews screenshots before Schedule and Grades build on it.")
sched=$(issue "Schedule: week, today, marks, island glance" wayfinder:task "Part of #$map

Week and today views with WeekTurn, vacant and online marks, subject colours, the glance, the Schedule controls in the island, empty-state scenes.")
grades=$(issue "Grades: GPA, trend, stamps" wayfinder:task "Part of #$map

GPA, the trend across past terms, units progress, grade reveal stamps.")
rem=$(issue "Reminders, sync journey, settings" wayfinder:task "Part of #$map

Weekly local notifications, the jeepney sync journey, the Settings screen with a wiring test that fails when a stored preference has no reader.")

for c in $g $p0 $s0 $s1 $api $core $ds $shell $sched $grades $rem; do
  gh api --method POST "repos/$repo/issues/$map/sub_issues" -F sub_issue_id="$(dbid "$c")" >/dev/null
done

block() { # child blocker...
  local c=$1; shift
  for b in "$@"; do
    gh api --method POST "repos/$repo/issues/$c/dependencies/blocked_by" -F issue_id="$(dbid "$b")" >/dev/null
  done
}
block "$api" "$s0" "$s1"
block "$core" "$api"
block "$ds" "$s1"
block "$shell" "$ds"
block "$sched" "$core" "$shell"
block "$grades" "$core" "$shell"
block "$rem" "$sched"

echo "map #$map"

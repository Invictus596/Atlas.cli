# Project Atlas - Progress Ledger

## Vision
AI-powered CLI for Codebase Scouting and Stakeholder Shipping.

## Core Modules Checklist

### CLI (Rust — `atlas/src/`)
- [x] **CLI Parser** — clap v4 subcommand routing (`scout`, `ship`)
- [x] **TUI Engine** — ratatui + crossterm render loop with status bar
- [x] **Config System** — serde/toml-based `.atlas.toml` load/save
- [x] **Directory Scanner** — recursive filesystem traversal
- [x] **Gemini Integration** — AI-powered codebase analysis
- [x] **PDF Generator** — structured report export
- [x] **Twilio/WhatsApp Integration** — stakeholder notifications
- [ ] **`.env` Support** — dotenvy crate for secret management
- [ ] **Ship Mode (Git Diff)** — actual diff parsing + AI summaries
- [ ] **PDF Font Fix** — cross-platform font discovery
- [ ] **Scanner Improvements** — binary filtering, size caps
- [ ] **`atlas login` command** — validate key against Supabase

### Website (Next.js — `atlas/website/`)
- [x] **Landing Page** (`/`) — animated hero, terminal mockup, feature cards
- [x] **Setup Page** (`/setup`) — form UI (email/phone/WhatsApp)
- [x] **Dashboard Page** (`/dashboard`) — repo list, quick start
- [ ] **Supabase Backend** — real DB tables (api_keys, repos, users)
- [ ] **Real API Key Provisioning** — replace mock `setTimeout` generator
- [ ] **Auth on Dashboard** — protect route, check session
- [ ] **Live Repo Data** — fetch from Supabase, not hardcoded
- [ ] **CLI ↔ Web Bridge** — login flow connects web to CLI config

---

## 🔍 Full Audit (2026-04-09)

### CLI — What Works
| Module | Status | Notes |
|--------|--------|-------|
| CLI Parser | ✅ Complete | `scout` and `ship` subcommands |
| TUI Engine | ✅ Complete | Animated dashboard, status bar, key bindings |
| Config System | ✅ Complete | `.atlas.toml` load/save |
| Directory Scanner | ✅ Complete | Recursive traversal with ignore lists |
| Gemini AI | ✅ Complete | Real API calls to `gemini-1.5-flash` |
| PDF Generator | ⚠️ Partial | Works but **hardcoded font path** (`/usr/share/fonts/truetype/liberation`) — fails on systems without Liberation fonts |
| Twilio Notifier | ⚠️ Partial | SMS works, WhatsApp wrapper exists but **never called** from TUI |

### CLI — Issues
1. **No `.env` support** — All secrets live in `.atlas.toml` (not git-safe)
2. **Ship mode is a stub** — `render_ship_panel()` shows "awaiting git diff trigger" with zero logic
3. **No binary file filtering** — scanner reads ALL files, including binaries
4. **No API key validation** — empty `api_key` hits Gemini API and gets cryptic error
5. **`atlas login` doesn't exist** — dashboard references it but CLI has no such command

### Website — What Works
| Page | Status | Notes |
|------|--------|-------|
| Landing (`/`) | ✅ Complete | Polished — Framer Motion, typing terminal, glow effects |
| Setup (`/setup`) | ⚠️ Mock Only | Form generates **fake** `ATLAS_live_XXX` key via `setTimeout` + random string. No backend. |
| Dashboard (`/dashboard`) | ⚠️ Static | Hardcoded 3 repos, hardcoded "Active" status. No auth. |

### Website — Issues
1. **No API/backend** — Setup generates fake keys client-side. No Supabase, no database
2. **No CLI ↔ Web connection** — CLI uses `.atlas.toml` directly. Dashboard can't talk to CLI
3. **Dashboard is static** — No live data, no real integration
4. **No auth** — Anyone can visit `/dashboard` or `/setup`
5. **`atlas login [YOUR_KEY]`** referenced but doesn't exist in CLI

---

## 📋 Full Implementation Plan

### Phase 1: `.env` Setup (Both CLI + Website)
1. Create `.env.example` at `atlas/` root (for Rust CLI):
   ```
   GEMINI_API_KEY=
   TWILIO_ACCOUNT_SID=
   TWILIO_AUTH_TOKEN=
   TWILIO_FROM_PHONE=
   SUPABASE_URL=
   SUPABASE_ANON_KEY=
   ```
2. Create `.env.example` at `atlas/website/` (for Next.js):
   ```
   NEXT_PUBLIC_SUPABASE_URL=
   NEXT_PUBLIC_SUPABASE_ANON_KEY=
   GEMINI_API_KEY=
   TWILIO_ACCOUNT_SID=
   TWILIO_AUTH_TOKEN=
   TWILIO_FROM_PHONE=
   ```
3. Add `.env` / `.env.local` to both `.gitignore` files
4. Add `dotenvy` crate to Rust CLI `Cargo.toml`
5. Refactor `config.rs` to load secrets from env vars, keep `.atlas.toml` for non-secret prefs (phones, emails)
6. Add validation: fail early if `GEMINI_API_KEY` is missing

### Phase 2: Supabase Backend (for Web Dashboard)
7. Set up Supabase tables: `api_keys`, `repositories`, `users`
8. Add Supabase client to website (`@supabase/supabase-js`)
9. Wire `/setup` form to create real user + generate/store API key in Supabase
10. Wire `/dashboard` to fetch real repos from Supabase
11. Add basic auth check on `/dashboard`

### Phase 3: CLI ↔ Web Bridge
12. Add `atlas login <key>` CLI command — validates key against Supabase
13. On successful login, auto-generate `.atlas.toml` with fetched config
14. Add `SUPABASE_URL` and `SUPABASE_ANON_KEY` to CLI `.env`

### Phase 4: CLI Improvements
15. **PDF font fix** — use `font-kit` crate or bundle a font for cross-platform support
16. **Scanner filtering** — skip binaries, add file size cap, better status labels ("Skipped (binary)", "Scanned")
17. **Error handling** — validate API key before hitting Gemini, graceful fallbacks

### Phase 5: Ship Mode + AI Polish
18. Create `src/git_diff.rs` — run `git diff` via `tokio::process::Command`
19. Parse diff output into structured data (added/removed/modified lines per file)
20. Feed diff context into Gemini AI prompt for Tech/QA/Business summaries
21. Update TUI Ship panel to show diff summary table
22. Wire Ship mode's `[a]` Analyze key to diff-aware AI prompt
23. Actually call `send_whatsapp()` from TUI (currently defined but never used)
24. PDF: parse and render Markdown formatting (headings, lists, code blocks)

### Phase 6: Polish
25. Add `atlas init` command to generate `.env.example` + `.atlas.toml` template
26. Better error messages throughout CLI
27. Website: add loading states, error boundaries, proper 404 page

---

## Changelog

### 2026-04-07 — Initial Scaffold
- Established `Cargo.toml` with `clap`, `crossterm`, `ratatui`
- Implemented CLI subcommands: `atlas scout`, `atlas ship`
- Built terminal lifecycle (raw mode, alternate screen, teardown)
- Render loop with centered bordered blocks per subcommand
- Event listener for `q` / `Ctrl+C` clean exit

### 2026-04-07 — Config System & Status Bar
- Added `serde` + `toml` dependencies
- Created `src/config.rs` with `Config` struct (api_key, ceo_phone, client_whatsapp, dev_email)
- Implemented `load_config()` and `save_config()` targeting `.atlas.toml`
- Updated `main.rs` to load config on startup
- Added status bar at bottom of TUI showing config state

### 2026-04-07 — Animated btop Dashboard & AI Skeleton
- **Dependencies:** Added `tokio` (full), `reqwest` (json, rustls-tls), `serde_json` to `Cargo.toml`
- **Async runtime:** Converted `fn main()` → `#[tokio::main] async fn main()`
- **Animation state:** Added `frame_count: usize` to `App`; non-blocking 100 ms event polling with frame counter increment
- **Logo animation:** `LOGO_FRAMES` const with 3 ASCII frames (Atlas lifting the globe), rendered via `app.frame_count % 3`
- **Color palette:** Unified on `Color::Rgb(2, 6, 23)` background + `Color::Rgb(56, 189, 248)` sky-blue for borders, text, and ASCII art
- **Layout:** Top section (Left 35% "SYSTEM & CONTEXT" / Right 65% "ENGINE: SCOUT") + 3-line bottom Status Bar
- **Left pane:** Animated logo (centered) + Gauge widget "CONTEXT DENSITY" at mocked 45%
- **Right pane:** Table widget with 3 columns (File Path, Size, Status) and 3 mock rows
- **AI skeleton:** Created `src/ai.rs` with `pub async fn generate_report(api_key, context) -> Result<String, String>` returning mock HTML
- `BorderType::Rounded` preserved on all blocks; existing config loading + scout/ship routing intact
- Build verified: `cargo build` compiles cleanly (zero errors)

### 2026-04-08 — Core Logic Implementation (Gemini CLI)
- **Directory Scanner:** Implemented `src/scanner.rs` using `tokio::fs` for recursive traversal with ignore lists (`.git`, `target`, `node_modules`).
- **Gemini Integration:** Upgraded `src/ai.rs` from mock to real Gemini 1.5 Flash API calls via `reqwest`.
- **PDF Generator:** Added `genpdf` dependency and implemented `src/report.rs` for enterprise report generation.
- **Twilio/WhatsApp:** Created `src/notifier.rs` for SMS/WhatsApp stakeholder notifications.
- **Config Expansion:** Updated `Config` struct in `src/config.rs` to support Twilio credentials.
- **TUI Integration:**
    - Wired `[a]` key to trigger async AI analysis.
    - Wired `[p]` key to generate PDF report from AI results.
    - Wired `[n]` key to send notifications to CEO.
    - Added real-time "CONTEXT DENSITY" calculation based on scanned line counts.
    - Implemented status message feedback in the TUI status bar.
- Build verified: `cargo build` compiles with zero errors.

### 2026-04-09 — Full Audit & Website Discovery
- Discovered `atlas/website/` — Next.js 16 web app with Landing, Setup, and Dashboard pages
- Landing page is production-ready (Framer Motion, Lenis smooth scroll, Tailwind v4)
- Setup page generates **mock** API keys client-side (no backend)
- Dashboard is static with hardcoded repos
- No Supabase integration, no auth, no CLI ↔ Web bridge
- Documented complete implementation plan (6 phases)
- Migrated secrets strategy from `.atlas.toml` → `.env` files

---

## Session Sign-off — 2026-04-09
**Status:** 📋 Complete audit done. Full implementation plan documented. Ready for next session.
**Next session:** Start Phase 1 — `.env` setup for CLI and website.
 
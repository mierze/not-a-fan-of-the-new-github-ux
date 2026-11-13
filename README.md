# Bookish Ladies of Long Beach

Simple single-club, single-month starter. Rust 2024 edition + Axum; React/Vite builds into public/. No database, accounts, or voting backend.

## Run
Requires Rust 1.85+ (2024 edition) and Node 20.19+.

```sh
cd web
npm ci
npm run build
cd ..
cargo run --release
```

Open http://localhost:8080, or http://YOUR-SERVER-IP:8080 from another machine. Default binding is 0.0.0.0:8080. Set BIND_ADDR to override. Run from the project root so data/ and public/ resolve correctly. Deployment needs the release binary, data/ and public/ in the same working directory. Put TLS at your reverse proxy for internet deployment.

## Edit this month's selection
Edit data/club.json and restart Axum:
- name, month, roundId: club and voting month; change roundId for a new shortlist.
- maxPicks: shortlist limit, currently six (assumption: six favorites, not six winning books).
- typeformUrl: paste the actual HTTPS voting form URL. Until configured, the UI says voting is coming soon.
- readingGoal / booksRead: manually managed monthly club progress.
- books: stable unique id, title, author, genre, isbn, description, reviewUrl.
- reviewUrl: paste a specific StoryGraph book page or other review page. Empty links fall back to Open Library ISBN pages.

The eight books and descriptions are starter content, not the club's confirmed nominations. Book jackets are typographic placeholders. Replace the JSON with your organizer's picks. No automatic book API calls: the initial metadata is curated. Open Library Search/Covers APIs are a natural later addition.

## Voting scope
Shortlists persist only in localStorage in the same browser. Opening Typeform does NOT submit or automatically transfer these selections. Members enter their choices in your Typeform. Configure Typeform questions and limits there. This app never claims a vote was submitted and has no results/thank-you screen because it cannot verify submission. No live tally or duplicate-vote protection in this app.

## Development
Run cargo run at the root. In a second terminal run npm run dev inside web/. Vite proxies /api to port 8080. Production serves the built frontend and API on the same origin.

## Validation
Frontend production build checked in the authoring environment. Rust compiler was unavailable there, so cargo check and runtime verification must be run on your Rust machine. No hosted deployment was created: this project uses the requested native Axum server.

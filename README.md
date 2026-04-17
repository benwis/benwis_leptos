# benwis_leptos

A full-stack personal blog and portfolio site built with [Leptos](https://leptos.dev/) (Rust + WASM), using server-side rendering with client-side hydration via the islands architecture.

## Tech Stack

- **Framework:** Leptos 0.8 with islands architecture
- **Backend:** Axum
- **Database:** SQLite via SQLx (migrations run automatically on startup)
- **Auth:** axum_session_auth with Argon2 password hashing
- **Styling:** SCSS (compiled by cargo-leptos)
- **Markdown:** Rendered to HTML with femark, pre-rendered and cached in the DB on write
- **Deployment:** Fly.io via Docker

## Project Structure

This is a Cargo workspace with three crates:

```
app/          Shared library (components, routes, models, server functions)
frontend/     WASM entry point (cdylib for client-side hydration)
server/       Axum binary (SSR, middleware, static files)
migrations/   SQLx migration files
styles/       Tailwind input CSS
public/       Static assets
```

Feature flags gate compilation targets:
- `ssr` -- server-only code (DB access, session management, auth)
- `hydrate` -- client-only code (WASM hydration)

## Setup

```bash
# Install cargo-leptos
cargo install --locked cargo-leptos

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Create your .env from the template
cp .env_template .env
```

The default `DATABASE_URL` is `sqlite:db/App.db?mode=rwc`. The database and tables are created automatically on first run.

## Development

```bash
# Run with hot reload
cargo leptos watch
```

Dev server runs at http://localhost:3000/.

## Production Build

```bash
cargo leptos build --release
```

## Docker

```bash
docker build -t benwis_leptos .
docker run -p 3000:3000 benwis_leptos
```

## License

MIT

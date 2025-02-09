
## Original setup
export DATABASE_URL=postgres://postgres@localhost:5432/diesel-pg-timestamp-usage
diesel setup
diesel migration generate create-example
# Write code in `migrations/up.sql` & `down.sql`
diesel migration run
diesel migration redo
diesel print-schema > src/schema.rs
# Write code in `src/models.rs`
# Write code in `src/main.rs`
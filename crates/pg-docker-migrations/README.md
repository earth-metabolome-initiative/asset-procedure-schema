# PG Docker Migrations

Helper crate to setup a dockerized PostgreSQL instance with migrations applied, for testing purposes.

Starting a Docker and applying migrations takes some time, so this crate is intended for use in integration tests which you most likely will want to [`ignore`](https://doc.rust-lang.org/reference/attributes/testing.html#r-attributes.testing.ignore) in normal test runs.

While it remains vitally important to test with the PostgreSQL backend, prefer using the [`pg2sqlite`](https://github.com/LucaCappelletti94/pg2sqlite)-based SQLite tests for most of your testing needs, as they significantly much faster to run.

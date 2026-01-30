# Core Extensions

This directory contains `SQL` documents relative to including PostgreSQL extensions.
Since PostgreSQL 18, we do not need anymore the `UUID` extension to use the `UUID` type,
as it is now natively supported by PostgreSQL.

Any extension added here should be commonly used and available for both PostgreSQL and SQLite. If it is not available for the latter, it should be added solely if it is transparently ignorable by SQLite.

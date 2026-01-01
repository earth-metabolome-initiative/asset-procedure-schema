# Time Report for Directus Schema Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Clippy Fixes` which took 16 seconds, 214 ms, 262 µs and 134 ns (90.62% of all time).

| name                               | time                                  | percentage | comment |
|------------------------------------|---------------------------------------|------------|---------|
| Database Introspection             | 118 ms, 458 µs and 221 ns             | 0.66%      |         |
| Schema Validation                  | 297 ms, 317 µs and 97 ns              | 1.66%      |         |
| SQL Workspace Generation           | 752 ms, 907 µs and 39 ns              | 4.21%      |         |
| TOML Formatting                    | 152 ms, 291 µs and 137 ns             | 0.85%      |         |
| Code Formatting (1)                | 177 ms, 20 µs and 773 ns              | 0.99%      |         |
| Clippy Fixes                       | 16 seconds, 214 ms, 262 µs and 134 ns | 90.62%     |         |
| Code Formatting (2)                | 165 ms, 331 µs and 609 ns             | 0.92%      |         |
| Workspace Dependency Visualization | 7 ms, 559 µs and 474 ns               | 0.04%      |         |
| DAG Structure Visualization        | 7 ms, 835 µs and 982 ns               | 0.04%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 729 ms, 90 µs and 55 ns (96.84% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 11 ms, 841 µs and 596 ns | 1.57%      |         |
| writing_crate_lib       | 729 ms, 90 µs and 55 ns  | 96.84%     |         |
| writing_sink_crate_toml | 601 µs and 180 ns        | 0.08%      |         |
| writing_sink_crate_lib  | 8 ms, 877 µs and 499 ns  | 1.18%      |         |
| workspace_toml          | 2 ms, 422 µs and 547 ns  | 0.32%      |         |
| workspace_rustfmt       | 74 µs and 162 ns         | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

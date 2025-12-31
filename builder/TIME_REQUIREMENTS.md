# Time Report for Directus Schema Generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 486 ms, 124 µs and 826 ns (89.09% of all time).

| name                               | time                                  | percentage | comment |
|------------------------------------|---------------------------------------|------------|---------|
| Database Introspection             | 97 ms, 242 µs and 370 ns              | 0.64%      |         |
| Schema Validation                  | 300 ms, 976 µs and 234 ns             | 1.99%      |         |
| SQL Workspace Generation           | 776 ms, 267 µs and 137 ns             | 5.13%      |         |
| TOML Formatting                    | 127 ms, 254 µs and 393 ns             | 0.84%      |         |
| Code Formatting (1)                | 177 ms, 432 µs and 580 ns             | 1.17%      |         |
| Clippy Fixes                       | 13 seconds, 486 ms, 124 µs and 826 ns | 89.09%     |         |
| Code Formatting (2)                | 165 ms, 179 µs and 146 ns             | 1.09%      |         |
| Workspace Dependency Visualization | 7 ms, 396 µs and 28 ns                | 0.05%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 752 ms, 200 µs and 379 ns (96.90% of all time).

| name                    | time                      | percentage | comment |
|-------------------------|---------------------------|------------|---------|
| writing_crate_toml      | 12 ms, 116 µs and 23 ns   | 1.56%      |         |
| writing_crate_lib       | 752 ms, 200 µs and 379 ns | 96.90%     |         |
| writing_sink_crate_toml | 644 µs and 566 ns         | 0.08%      |         |
| writing_sink_crate_lib  | 8 ms, 874 µs and 878 ns   | 1.14%      |         |
| workspace_toml          | 2 ms, 356 µs and 999 ns   | 0.30%      |         |
| workspace_rustfmt       | 74 µs and 292 ns          | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

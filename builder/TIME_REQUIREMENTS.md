# Time Report for Directus Schema Generation

The total time spent on all tasks was 14 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 57 ms, 198 µs and 706 ns (89.07% of all time).

| name                     | time                                 | percentage | comment |
|--------------------------|--------------------------------------|------------|---------|
| Database Introspection   | 87 ms, 450 µs and 980 ns             | 0.60%      |         |
| Schema Validation        | 303 ms, 298 µs and 88 ns             | 2.07%      |         |
| SQL Workspace Generation | 756 ms, 882 µs and 538 ns            | 5.16%      |         |
| TOML Formatting          | 113 ms, 207 µs and 537 ns            | 0.77%      |         |
| Code Formatting (1)      | 177 ms, 32 µs and 986 ns             | 1.21%      |         |
| Clippy Fixes             | 13 seconds, 57 ms, 198 µs and 706 ns | 89.07%     |         |
| Code Formatting (2)      | 164 ms, 993 µs and 594 ns            | 1.13%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 733 ms, 390 µs and 768 ns (96.90% of all time).

| name                    | time                      | percentage | comment |
|-------------------------|---------------------------|------------|---------|
| writing_crate_toml      | 11 ms, 566 µs and 98 ns   | 1.53%      |         |
| writing_crate_lib       | 733 ms, 390 µs and 768 ns | 96.90%     |         |
| writing_sink_crate_toml | 636 µs and 243 ns         | 0.08%      |         |
| writing_sink_crate_lib  | 8 ms, 822 µs and 324 ns   | 1.17%      |         |
| workspace_toml          | 2 ms, 379 µs and 973 ns   | 0.31%      |         |
| workspace_rustfmt       | 87 µs and 132 ns          | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

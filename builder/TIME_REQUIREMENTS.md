# Time Report for Directus Schema Generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 363 ms, 531 µs and 687 ns (89.00% of all time).

| name                               | time                                  | percentage | comment |
|------------------------------------|---------------------------------------|------------|---------|
| Database Introspection             | 98 ms, 981 µs and 134 ns              | 0.66%      |         |
| Schema Validation                  | 304 ms, 148 µs and 781 ns             | 2.03%      |         |
| SQL Workspace Generation           | 770 ms, 226 µs and 227 ns             | 5.13%      |         |
| TOML Formatting                    | 123 ms, 820 µs and 23 ns              | 0.82%      |         |
| Code Formatting (1)                | 178 ms, 361 µs and 989 ns             | 1.19%      |         |
| Clippy Fixes                       | 13 seconds, 363 ms, 531 µs and 687 ns | 89.00%     |         |
| Code Formatting (2)                | 169 ms, 185 µs and 584 ns             | 1.13%      |         |
| Workspace Dependency Visualization | 7 ms, 328 µs and 647 ns               | 0.05%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 746 ms, 277 µs and 56 ns (96.89% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 11 ms, 872 µs and 485 ns | 1.54%      |         |
| writing_crate_lib       | 746 ms, 277 µs and 56 ns | 96.89%     |         |
| writing_sink_crate_toml | 632 µs and 588 ns        | 0.08%      |         |
| writing_sink_crate_lib  | 8 ms, 839 µs and 695 ns  | 1.15%      |         |
| workspace_toml          | 2 ms, 527 µs and 116 ns  | 0.33%      |         |
| workspace_rustfmt       | 77 µs and 287 ns         | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

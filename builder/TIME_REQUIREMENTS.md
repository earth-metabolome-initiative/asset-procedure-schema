# Time Report for APS Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Clippy Fixes` which took 16 seconds, 279 ms, 235 µs and 913 ns (91.11% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 112 ms, 558 µs and 659 ns             | 0.63%      |
| Schema Validation                  | 61 ms, 416 µs and 325 ns              | 0.34%      |
| SQL Workspace Generation           | 793 ms, 821 µs and 894 ns             | 4.44%      |
| TOML Formatting                    | 151 ms, 534 µs and 412 ns             | 0.85%      |
| Code Formatting (1)                | 267 ms, 515 µs and 247 ns             | 1.50%      |
| Clippy Fixes                       | 16 seconds, 279 ms, 235 µs and 913 ns | 91.11%     |
| Code Formatting (2)                | 179 ms, 855 µs and 624 ns             | 1.01%      |
| Workspace Dependency Visualization | 10 ms, 401 µs and 766 ns              | 0.06%      |
| DAG Structure Visualization        | 10 ms, 728 µs and 80 ns               | 0.06%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 768 ms, 114 µs and 462 ns (96.76% of all time).

| name                    | time                      | percentage |
|-------------------------|---------------------------|------------|
| writing_crate_toml      | 11 ms, 759 µs and 583 ns  | 1.48%      |
| writing_crate_lib       | 768 ms, 114 µs and 462 ns | 96.76%     |
| writing_sink_crate_toml | 757 µs and 448 ns         | 0.10%      |
| writing_sink_crate_lib  | 10 ms, 246 µs and 289 ns  | 1.29%      |
| workspace_toml          | 2 ms, 858 µs and 452 ns   | 0.36%      |
| workspace_rustfmt       | 85 µs and 660 ns          | 0.01%      |

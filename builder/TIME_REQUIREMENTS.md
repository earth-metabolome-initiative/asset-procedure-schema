# Time Report for APS Generation

The total time spent on all tasks was 33 seconds.
The slowest task was `Clippy Fixes` which took 24 seconds, 311 ms, 243 µs and 379 ns (72.77% of all time).

| name                               | time                                  | percentage | comment |
|------------------------------------|---------------------------------------|------------|---------|
| Database Introspection             | 138 ms, 312 µs and 482 ns             | 0.41%      |         |
| Schema Validation                  | 2 seconds, 850 ms, 69 µs and 508 ns   | 8.53%      |         |
| SQL Workspace Generation           | 5 seconds, 464 ms, 796 µs and 129 ns  | 16.36%     |         |
| TOML Formatting                    | 152 ms, 112 µs and 279 ns             | 0.46%      |         |
| Code Formatting (1)                | 182 ms, 246 µs and 257 ns             | 0.55%      |         |
| Clippy Fixes                       | 24 seconds, 311 ms, 243 µs and 379 ns | 72.77%     |         |
| Code Formatting (2)                | 174 ms, 631 µs and 816 ns             | 0.52%      |         |
| Workspace Dependency Visualization | 63 ms, 928 µs and 845 ns              | 0.19%      |         |
| DAG Structure Visualization        | 70 ms, 7 µs and 110 ns                | 0.21%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 5 seconds, 428 ms, 382 µs and 723 ns (99.33% of all time).

| name                    | time                                 | percentage | comment |
|-------------------------|--------------------------------------|------------|---------|
| writing_crate_toml      | 22 ms, 844 µs and 36 ns              | 0.42%      |         |
| writing_crate_lib       | 5 seconds, 428 ms, 382 µs and 723 ns | 99.33%     |         |
| writing_sink_crate_toml | 714 µs and 542 ns                    | 0.01%      |         |
| writing_sink_crate_lib  | 10 ms, 133 µs and 837 ns             | 0.19%      |         |
| workspace_toml          | 2 ms, 644 µs and 565 ns              | 0.05%      |         |
| workspace_rustfmt       | 76 µs and 426 ns                     | 0.00%      |         |

![Plot](TIME_REQUIREMENTS.png)

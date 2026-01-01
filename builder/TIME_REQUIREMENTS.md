# Time Report for APS Generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 402 ms, 168 µs and 75 ns (88.76% of all time).

| name                               | time                                 | percentage | comment |
|------------------------------------|--------------------------------------|------------|---------|
| Database Introspection             | 118 ms, 237 µs and 558 ns            | 0.78%      |         |
| Schema Validation                  | 297 ms, 673 µs and 487 ns            | 1.97%      |         |
| SQL Workspace Generation           | 770 ms, 685 µs and 195 ns            | 5.10%      |         |
| TOML Formatting                    | 148 ms, 760 µs and 362 ns            | 0.99%      |         |
| Code Formatting (1)                | 177 ms, 875 µs and 287 ns            | 1.18%      |         |
| Clippy Fixes                       | 13 seconds, 402 ms, 168 µs and 75 ns | 88.76%     |         |
| Code Formatting (2)                | 168 ms, 465 µs and 273 ns            | 1.12%      |         |
| Workspace Dependency Visualization | 7 ms, 323 µs and 806 ns              | 0.05%      |         |
| DAG Structure Visualization        | 7 ms, 820 µs and 839 ns              | 0.05%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 747 ms, 8 µs and 190 ns (96.93% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 11 ms, 693 µs and 176 ns | 1.52%      |         |
| writing_crate_lib       | 747 ms, 8 µs and 190 ns  | 96.93%     |         |
| writing_sink_crate_toml | 628 µs and 731 ns        | 0.08%      |         |
| writing_sink_crate_lib  | 8 ms, 946 µs and 102 ns  | 1.16%      |         |
| workspace_toml          | 2 ms, 339 µs and 942 ns  | 0.30%      |         |
| workspace_rustfmt       | 69 µs and 54 ns          | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

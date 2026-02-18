# Time Report for APS Generation

The total time spent on all tasks was a minute.
The slowest task was `Clippy Fixes` which took 48 seconds, 601 ms, 883 µs and 164 ns (91.75% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 298 ms, 642 µs and 196 ns             | 0.56%      |
| Schema Validation                  | 152 ms, 799 µs and 926 ns             | 0.29%      |
| SQL Workspace Generation           | 2 seconds, 728 ms and 299 ns          | 5.15%      |
| TOML Formatting                    | 370 ms, 714 µs and 696 ns             | 0.70%      |
| Code Formatting (1)                | 425 ms, 522 µs and 766 ns             | 0.80%      |
| Clippy Fixes                       | 48 seconds, 601 ms, 883 µs and 164 ns | 91.75%     |
| Code Formatting (2)                | 355 ms, 155 µs and 593 ns             | 0.67%      |
| Workspace Dependency Visualization | 21 ms and 681 µs                      | 0.04%      |
| DAG Structure Visualization        | 19 ms, 675 µs and 86 ns               | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 987 ms, 135 µs and 131 ns (72.84% of all time).

| name                                     | time                                | percentage |
|------------------------------------------|-------------------------------------|------------|
| writing_crate_toml                       | 706 ms, 883 µs and 779 ns           | 25.91%     |
| writing_crate_lib                        | 1 second, 987 ms, 135 µs and 131 ns | 72.84%     |
| writing_sink_crate_toml                  | 516 µs and 321 ns                   | 0.02%      |
| writing_sink_crate_lib                   | 6 ms, 936 µs and 658 ns             | 0.25%      |
| writing_sink_crate_toml_aps-dag-entities | 760 µs and 617 ns                   | 0.03%      |
| writing_sink_crate_lib_aps-dag-entities  | 6 ms, 769 µs and 353 ns             | 0.25%      |
| workspace_toml                           | 18 ms, 919 µs and 818 ns            | 0.69%      |
| workspace_rustfmt                        | 78 µs and 622 ns                    | 0.00%      |

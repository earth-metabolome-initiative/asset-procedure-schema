# Time Report for APS Generation

The total time spent on all tasks was a minute.
The slowest task was `Clippy Fixes` which took 57 seconds, 523 ms, 819 µs and 85 ns (93.51% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 432 ms, 446 µs and 34 ns             | 0.70%      |
| Schema Validation                  | 99 ms, 489 µs and 294 ns             | 0.16%      |
| SQL Workspace Generation           | 2 seconds, 272 ms, 392 µs and 322 ns | 3.69%      |
| TOML Formatting                    | 591 ms, 77 µs and 283 ns             | 0.96%      |
| Code Formatting (1)                | 308 ms, 761 µs and 97 ns             | 0.50%      |
| Clippy Fixes                       | 57 seconds, 523 ms, 819 µs and 85 ns | 93.51%     |
| Code Formatting (2)                | 258 ms, 455 µs and 360 ns            | 0.42%      |
| Workspace Dependency Visualization | 15 ms, 375 µs and 979 ns             | 0.02%      |
| DAG Structure Visualization        | 14 ms, 98 µs and 483 ns              | 0.02%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 743 ms, 180 µs and 86 ns (76.71% of all time).

| name                                     | time                               | percentage |
|------------------------------------------|------------------------------------|------------|
| writing_crate_toml                       | 492 ms, 20 µs and 209 ns           | 21.65%     |
| writing_crate_lib                        | 1 second, 743 ms, 180 µs and 86 ns | 76.71%     |
| writing_sink_crate_toml                  | 762 µs and 786 ns                  | 0.03%      |
| writing_sink_crate_lib                   | 10 ms, 614 µs and 454 ns           | 0.47%      |
| writing_sink_crate_toml_aps-dag-entities | 918 µs and 782 ns                  | 0.04%      |
| writing_sink_crate_lib_aps-dag-entities  | 10 ms, 421 µs and 692 ns           | 0.46%      |
| workspace_toml                           | 14 ms, 367 µs and 661 ns           | 0.63%      |
| workspace_rustfmt                        | 106 µs and 652 ns                  | 0.00%      |

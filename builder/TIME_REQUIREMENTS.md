# Time Report for APS Generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Clippy Fixes` which took 1 minute, 35 seconds, 430 ms, 842 µs and 880 ns (93.59% of all time).

| name                               | time                                            | percentage |
|------------------------------------|-------------------------------------------------|------------|
| Database Introspection             | 150 ms, 452 µs and 36 ns                        | 0.15%      |
| Schema Validation                  | 194 ms, 689 µs and 246 ns                       | 0.19%      |
| SQL Workspace Generation           | 5 seconds, 68 ms, 637 µs and 307 ns             | 4.97%      |
| TOML Formatting                    | 188 ms, 556 µs and 568 ns                       | 0.18%      |
| Code Formatting (1)                | 453 ms, 393 µs and 274 ns                       | 0.44%      |
| Clippy Fixes                       | 1 minute, 35 seconds, 430 ms, 842 µs and 880 ns | 93.59%     |
| Code Formatting (2)                | 385 ms, 861 µs and 824 ns                       | 0.38%      |
| Workspace Dependency Visualization | 61 ms, 33 µs and 806 ns                         | 0.06%      |
| DAG Structure Visualization        | 33 ms, 477 µs and 509 ns                        | 0.03%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 3 seconds, 336 ms, 823 µs and 616 ns (65.83% of all time).

| name                                        | time                                 | percentage |
|---------------------------------------------|--------------------------------------|------------|
| writing_crate_toml                          | 1 second, 674 ms, 26 µs and 769 ns   | 33.03%     |
| writing_crate_lib                           | 3 seconds, 336 ms, 823 µs and 616 ns | 65.83%     |
| writing_sink_crate_toml                     | 591 µs and 253 ns                    | 0.01%      |
| writing_sink_crate_lib                      | 8 ms, 568 µs and 222 ns              | 0.17%      |
| writing_sink_crate_toml_aps-dag-entities    | 936 µs and 441 ns                    | 0.02%      |
| writing_sink_crate_lib_aps-dag-entities     | 8 ms, 524 µs and 770 ns              | 0.17%      |
| writing_sink_crate_toml_aps-dag-table_names | 1 ms, 2 µs and 492 ns                | 0.02%      |
| writing_sink_crate_lib_aps-dag-table_names  | 8 ms, 790 µs and 267 ns              | 0.17%      |
| workspace_toml                              | 29 ms, 292 µs and 155 ns             | 0.58%      |
| workspace_rustfmt                           | 81 µs and 322 ns                     | 0.00%      |

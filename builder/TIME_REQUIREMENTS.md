# Time Report for APS Generation

The total time spent on all tasks was 5 minutes.
The slowest task was `Schema Validation` which took 2 minutes, 53 seconds, 67 ms, 492 µs and 188 ns (54.47% of all time).

| name                               | time                                            | percentage |
|------------------------------------|-------------------------------------------------|------------|
| Database Introspection             | 1 second, 627 ms, 866 µs and 843 ns             | 0.51%      |
| Schema Validation                  | 2 minutes, 53 seconds, 67 ms, 492 µs and 188 ns | 54.47%     |
| SQL Workspace Generation           | 52 seconds, 963 ms, 273 µs and 783 ns           | 16.67%     |
| TOML Formatting                    | 483 ms, 472 µs and 310 ns                       | 0.15%      |
| Code Formatting (1)                | 316 ms, 761 µs and 625 ns                       | 0.10%      |
| Clippy Fixes                       | 1 minute, 27 seconds, 610 ms, 126 µs and 506 ns | 27.58%     |
| Code Formatting (2)                | 246 ms, 813 µs and 282 ns                       | 0.08%      |
| Workspace Dependency Visualization | 691 ms, 915 µs and 27 ns                        | 0.22%      |
| DAG Structure Visualization        | 704 ms, 525 µs and 279 ns                       | 0.22%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was a minute.
The slowest task was `writing_crate_lib` which took 26 seconds, 45 ms, 674 µs and 198 ns (49.18% of all time).

| name                                      | time                                 | percentage |
|-------------------------------------------|--------------------------------------|------------|
| writing_enum_crate                        | 5 ms, 47 µs and 243 ns               | 0.01%      |
| writing_crate_toml                        | 12 seconds, 295 ms, 5 µs and 888 ns  | 23.21%     |
| writing_crate_lib                         | 26 seconds, 45 ms, 674 µs and 198 ns | 49.18%     |
| writing_sink_crate_toml                   | 926 µs and 893 ns                    | 0.00%      |
| writing_sink_crate_lib                    | 3 seconds, 563 ms, 900 µs and 200 ns | 6.73%      |
| writing_sink_crate_toml_aps-dag-grantees  | 527 µs and 417 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-grantees   | 2 ms, 15 µs and 240 ns               | 0.00%      |
| writing_sink_crate_toml_aps-dag-resources | 306 µs and 649 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-resources  | 1 ms, 657 µs and 904 ns              | 0.00%      |
| workspace_toml                            | 11 seconds, 47 ms, 909 µs and 208 ns | 20.86%     |
| workspace_rustfmt                         | 302 µs and 943 ns                    | 0.00%      |

# Time Report for APS Generation

The total time spent on all tasks was now.
The slowest task was `SQL Workspace Generation` which took 3 seconds, 826 ms, 453 µs and 940 ns (65.64% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 344 ms, 998 µs and 412 ns            | 5.92%      |
| Schema Validation                  | 159 ms, 970 µs and 982 ns            | 2.74%      |
| SQL Workspace Generation           | 3 seconds, 826 ms, 453 µs and 940 ns | 65.64%     |
| TOML Formatting                    | 447 ms, 309 µs and 660 ns            | 7.67%      |
| Code Formatting (1)                | 421 ms, 12 µs and 302 ns             | 7.22%      |
| Clippy Fixes                       | 218 ms, 591 µs and 708 ns            | 3.75%      |
| Code Formatting (2)                | 355 ms, 386 µs and 403 ns            | 6.10%      |
| Workspace Dependency Visualization | 29 ms, 23 µs and 561 ns              | 0.50%      |
| DAG Structure Visualization        | 26 ms, 991 µs and 225 ns             | 0.46%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 2 seconds, 622 ms, 259 µs and 202 ns (68.53% of all time).

| name                                     | time                                 | percentage |
|------------------------------------------|--------------------------------------|------------|
| writing_crate_toml                       | 1 second, 162 ms, 541 µs and 35 ns   | 30.38%     |
| writing_crate_lib                        | 2 seconds, 622 ms, 259 µs and 202 ns | 68.53%     |
| writing_sink_crate_toml                  | 582 µs and 463 ns                    | 0.02%      |
| writing_sink_crate_lib                   | 7 ms, 861 µs and 872 ns              | 0.21%      |
| writing_sink_crate_toml_aps-dag-entities | 855 µs and 780 ns                    | 0.02%      |
| writing_sink_crate_lib_aps-dag-entities  | 7 ms, 807 µs and 570 ns              | 0.20%      |
| workspace_toml                           | 24 ms, 472 µs and 636 ns             | 0.64%      |
| workspace_rustfmt                        | 73 µs and 382 ns                     | 0.00%      |

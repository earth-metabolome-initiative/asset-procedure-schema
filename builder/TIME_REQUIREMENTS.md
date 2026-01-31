# Time Report for APS Generation

The total time spent on all tasks was now.
The slowest task was `SQL Workspace Generation` which took 2 seconds, 210 ms, 258 µs and 984 ns (81.13% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 88 ms, 186 µs and 544 ns             | 3.24%      |
| Schema Validation                  | 14 µs and 341 ns                     | 0.00%      |
| SQL Workspace Generation           | 2 seconds, 210 ms, 258 µs and 984 ns | 81.13%     |
| TOML Formatting                    | 111 ms, 338 µs and 752 ns            | 4.09%      |
| Code Formatting (1)                | 93 ms, 402 µs and 136 ns             | 3.43%      |
| Clippy Fixes                       | 89 ms, 261 µs and 112 ns             | 3.28%      |
| Code Formatting (2)                | 94 ms, 997 µs and 493 ns             | 3.49%      |
| Workspace Dependency Visualization | 18 ms, 765 µs and 582 ns             | 0.69%      |
| DAG Structure Visualization        | 18 ms, 137 µs and 120 ns             | 0.67%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 829 ms, 48 µs and 158 ns (82.75% of all time).

| name                                     | time                               | percentage |
|------------------------------------------|------------------------------------|------------|
| writing_crate_toml                       | 330 ms, 757 µs and 252 ns          | 14.96%     |
| writing_crate_lib                        | 1 second, 829 ms, 48 µs and 158 ns | 82.75%     |
| writing_sink_crate_toml                  | 730 µs and 287 ns                  | 0.03%      |
| writing_sink_crate_lib                   | 10 ms, 396 µs and 262 ns           | 0.47%      |
| writing_sink_crate_toml_aps-dag-ownables | 1 ms, 28 µs and 277 ns             | 0.05%      |
| writing_sink_crate_lib_aps-dag-ownables  | 9 ms, 729 µs and 351 ns            | 0.44%      |
| writing_sink_crate_toml_aps-dag-owners   | 831 µs and 510 ns                  | 0.04%      |
| writing_sink_crate_lib_aps-dag-owners    | 10 ms, 132 µs and 683 ns           | 0.46%      |
| workspace_toml                           | 17 ms, 518 µs and 673 ns           | 0.79%      |
| workspace_rustfmt                        | 86 µs and 531 ns                   | 0.00%      |

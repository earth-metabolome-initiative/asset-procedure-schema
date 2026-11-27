# Time Report for APS Generation

The total time spent on all tasks was 4 minutes.
The slowest task was `Schema Validation` which took 2 minutes, 41 seconds, 316 ms, 267 µs and 138 ns (55.74% of all time).

| name                               | time                                             | percentage |
|------------------------------------|--------------------------------------------------|------------|
| Database Introspection             | 1 second, 695 ms, 366 µs and 507 ns              | 0.59%      |
| Schema Validation                  | 2 minutes, 41 seconds, 316 ms, 267 µs and 138 ns | 55.74%     |
| SQL Workspace Generation           | 13 seconds, 382 ms, 665 µs and 70 ns             | 4.62%      |
| TOML Formatting                    | 494 ms, 669 µs and 924 ns                        | 0.17%      |
| Code Formatting (1)                | 319 ms, 444 µs and 947 ns                        | 0.11%      |
| Clippy Fixes                       | 1 minute, 51 seconds, 595 ms, 517 µs and 442 ns  | 38.56%     |
| Code Formatting (2)                | 238 ms, 937 µs and 499 ns                        | 0.08%      |
| Workspace Dependency Visualization | 168 ms, 276 µs and 126 ns                        | 0.06%      |
| DAG Structure Visualization        | 176 ms, 395 µs and 40 ns                         | 0.06%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was 13 seconds.
The slowest task was `writing_crate_lib` which took 7 seconds, 335 ms, 436 µs and 269 ns (54.81% of all time).

| name                                      | time                                 | percentage |
|-------------------------------------------|--------------------------------------|------------|
| writing_enum_crate                        | 7 ms, 126 µs and 768 ns              | 0.05%      |
| writing_crate_toml                        | 2 seconds, 896 ms, 325 µs and 296 ns | 21.64%     |
| writing_crate_lib                         | 7 seconds, 335 ms, 436 µs and 269 ns | 54.81%     |
| writing_sink_crate_toml                   | 429 µs and 872 ns                    | 0.00%      |
| writing_sink_crate_lib                    | 676 ms, 360 µs and 590 ns            | 5.05%      |
| writing_sink_crate_toml_aps-dag-resources | 334 µs and 653 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-resources  | 1 ms, 602 µs and 28 ns               | 0.01%      |
| writing_sink_crate_toml_aps-dag-users     | 285 µs and 309 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-users      | 1 ms, 443 µs and 318 ns              | 0.01%      |
| workspace_toml                            | 2 seconds, 463 ms, 128 µs and 954 ns | 18.41%     |
| workspace_rustfmt                         | 192 µs and 13 ns                     | 0.00%      |

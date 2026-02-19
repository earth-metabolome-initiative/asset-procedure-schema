# Time Report for APS Generation

The total time spent on all tasks was a minute.
The slowest task was `Clippy Fixes` which took 56 seconds, 43 ms, 239 µs and 32 ns (89.53% of all time).

| name                               | time                                | percentage |
|------------------------------------|-------------------------------------|------------|
| Database Introspection             | 143 ms, 344 µs and 772 ns           | 0.23%      |
| Schema Validation                  | 190 ms, 381 µs and 43 ns            | 0.30%      |
| SQL Workspace Generation           | 5 seconds, 98 ms, 192 µs and 688 ns | 8.14%      |
| TOML Formatting                    | 182 ms, 845 µs and 695 ns           | 0.29%      |
| Code Formatting (1)                | 451 ms, 766 µs and 472 ns           | 0.72%      |
| Clippy Fixes                       | 56 seconds, 43 ms, 239 µs and 32 ns | 89.53%     |
| Code Formatting (2)                | 386 ms, 56 µs and 48 ns             | 0.62%      |
| Workspace Dependency Visualization | 68 ms, 202 µs and 474 ns            | 0.11%      |
| DAG Structure Visualization        | 34 ms, 226 µs and 695 ns            | 0.05%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 3 seconds, 349 ms, 244 µs and 316 ns (65.69% of all time).

| name                                        | time                                 | percentage |
|---------------------------------------------|--------------------------------------|------------|
| writing_crate_toml                          | 1 second, 691 ms, 35 µs and 589 ns   | 33.17%     |
| writing_crate_lib                           | 3 seconds, 349 ms, 244 µs and 316 ns | 65.69%     |
| writing_sink_crate_toml                     | 639 µs and 105 ns                    | 0.01%      |
| writing_sink_crate_lib                      | 8 ms, 661 µs and 283 ns              | 0.17%      |
| writing_sink_crate_toml_aps-dag-entities    | 981 µs and 791 ns                    | 0.02%      |
| writing_sink_crate_lib_aps-dag-entities     | 8 ms, 566 µs and 712 ns              | 0.17%      |
| writing_sink_crate_toml_aps-dag-table_names | 993 µs and 742 ns                    | 0.02%      |
| writing_sink_crate_lib_aps-dag-table_names  | 8 ms, 941 µs and 520 ns              | 0.18%      |
| workspace_toml                              | 29 ms, 58 µs and 259 ns              | 0.57%      |
| workspace_rustfmt                           | 70 µs and 371 ns                     | 0.00%      |

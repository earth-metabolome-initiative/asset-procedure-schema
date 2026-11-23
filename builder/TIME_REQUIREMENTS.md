# Time Report for APS Generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Schema Validation` which took 1 minute, 26 seconds, 29 ms, 161 µs and 31 ns (86.26% of all time).

| name                               | time                                          | percentage |
|------------------------------------|-----------------------------------------------|------------|
| Database Introspection             | 1 second, 929 ms, 307 µs and 98 ns            | 1.93%      |
| Schema Validation                  | 1 minute, 26 seconds, 29 ms, 161 µs and 31 ns | 86.26%     |
| SQL Workspace Generation           | 10 seconds, 891 ms, 460 µs and 534 ns         | 10.92%     |
| TOML Formatting                    | 152 ms, 806 µs and 929 ns                     | 0.15%      |
| Code Formatting (1)                | 252 ms, 908 µs and 879 ns                     | 0.25%      |
| Clippy Fixes                       | 47 ms, 394 µs and 817 ns                      | 0.05%      |
| Code Formatting (2)                | 200 ms, 791 µs and 250 ns                     | 0.20%      |
| Workspace Dependency Visualization | 110 ms, 377 µs and 893 ns                     | 0.11%      |
| DAG Structure Visualization        | 117 ms, 979 µs and 694 ns                     | 0.12%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 5 seconds, 462 ms, 346 µs and 846 ns (50.15% of all time).

| name                                        | time                                 | percentage |
|---------------------------------------------|--------------------------------------|------------|
| writing_enum_crate                          | 3 ms, 444 µs and 247 ns              | 0.03%      |
| writing_crate_toml                          | 2 seconds, 825 ms, 492 µs and 649 ns | 25.94%     |
| writing_crate_lib                           | 5 seconds, 462 ms, 346 µs and 846 ns | 50.15%     |
| writing_sink_crate_toml                     | 324 µs and 52 ns                     | 0.00%      |
| writing_sink_crate_lib                      | 271 ms, 698 µs and 276 ns            | 2.49%      |
| writing_sink_crate_toml_aps-dag-grantees    | 358 µs and 878 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-grantees     | 1 ms, 469 µs and 766 ns              | 0.01%      |
| writing_sink_crate_toml_aps-dag-resources   | 220 µs and 887 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-resources    | 1 ms, 142 µs and 268 ns              | 0.01%      |
| writing_sink_crate_toml_aps-dag-table_names | 226 µs and 428 ns                    | 0.00%      |
| writing_sink_crate_lib_aps-dag-table_names  | 1 ms, 68 µs and 228 ns               | 0.01%      |
| workspace_toml                              | 2 seconds, 323 ms, 523 µs and 15 ns  | 21.33%     |
| workspace_rustfmt                           | 144 µs and 994 ns                    | 0.00%      |

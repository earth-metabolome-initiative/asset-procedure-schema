# Time Report for APS Generation

The total time spent on all tasks was 42 seconds.
The slowest task was `Clippy Fixes` which took 39 seconds, 896 ms, 481 µs and 117 ns (94.42% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 142 ms, 902 µs and 313 ns             | 0.34%      |
| Schema Validation                  | 70 ms, 373 µs and 304 ns              | 0.17%      |
| SQL Workspace Generation           | 1 second, 434 ms, 969 µs and 19 ns    | 3.40%      |
| TOML Formatting                    | 179 ms, 25 µs and 614 ns              | 0.42%      |
| Code Formatting (1)                | 276 ms, 89 µs and 26 ns               | 0.65%      |
| Clippy Fixes                       | 39 seconds, 896 ms, 481 µs and 117 ns | 94.42%     |
| Code Formatting (2)                | 229 ms, 362 µs and 960 ns             | 0.54%      |
| Workspace Dependency Visualization | 13 ms, 247 µs and 451 ns              | 0.03%      |
| DAG Structure Visualization        | 13 ms, 551 µs and 812 ns              | 0.03%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 159 ms, 307 µs and 608 ns (80.79% of all time).

| name                                                | time                                | percentage |
|-----------------------------------------------------|-------------------------------------|------------|
| writing_crate_toml                                  | 222 ms, 395 µs and 266 ns           | 15.50%     |
| writing_crate_lib                                   | 1 second, 159 ms, 307 µs and 608 ns | 80.79%     |
| writing_sink_crate_toml                             | 775 µs and 275 ns                   | 0.05%      |
| writing_sink_crate_lib                              | 10 ms, 564 µs and 477 ns            | 0.74%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 303 µs and 295 ns             | 0.09%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 10 ms, 22 µs and 25 ns              | 0.70%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 374 µs and 828 ns             | 0.17%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 560 µs and 986 ns             | 0.39%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 895 µs and 347 ns                   | 0.06%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 874 µs and 816 ns             | 0.34%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 372 µs and 138 ns             | 0.10%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 178 µs and 946 ns             | 0.22%      |
| workspace_toml                                      | 12 ms, 257 µs and 181 ns            | 0.85%      |
| workspace_rustfmt                                   | 86 µs and 831 ns                    | 0.01%      |

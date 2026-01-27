# Time Report for APS Generation

The total time spent on all tasks was 35 seconds.
The slowest task was `Clippy Fixes` which took 33 seconds, 101 ms, 492 µs and 272 ns (92.42% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 215 ms, 235 µs and 820 ns             | 0.60%      |
| Schema Validation                  | 75 ms, 353 µs and 917 ns              | 0.21%      |
| SQL Workspace Generation           | 1 second, 638 ms, 936 µs and 653 ns   | 4.58%      |
| TOML Formatting                    | 276 ms, 405 µs and 506 ns             | 0.77%      |
| Code Formatting (1)                | 258 ms, 295 µs and 868 ns             | 0.72%      |
| Clippy Fixes                       | 33 seconds, 101 ms, 492 µs and 272 ns | 92.42%     |
| Code Formatting (2)                | 224 ms, 352 µs and 30 ns              | 0.63%      |
| Workspace Dependency Visualization | 12 ms, 195 µs and 838 ns              | 0.03%      |
| DAG Structure Visualization        | 13 ms, 438 µs and 380 ns              | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 374 ms, 462 µs and 261 ns (83.86% of all time).

| name                                                | time                                | percentage |
|-----------------------------------------------------|-------------------------------------|------------|
| writing_crate_toml                                  | 216 ms, 30 µs and 46 ns             | 13.18%     |
| writing_crate_lib                                   | 1 second, 374 ms, 462 µs and 261 ns | 83.86%     |
| writing_sink_crate_toml                             | 662 µs and 724 ns                   | 0.04%      |
| writing_sink_crate_lib                              | 9 ms, 415 µs and 545 ns             | 0.57%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 139 µs and 426 ns             | 0.07%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 8 ms, 965 µs and 132 ns             | 0.55%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 167 µs and 103 ns             | 0.13%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 14 µs and 317 ns              | 0.31%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 835 µs and 967 ns                   | 0.05%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 306 µs and 595 ns             | 0.26%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 240 µs and 921 ns             | 0.08%      |
| writing_sink_crate_lib_aps-dag-procedures           | 2 ms, 844 µs and 189 ns             | 0.17%      |
| workspace_toml                                      | 11 ms, 774 µs and 299 ns            | 0.72%      |
| workspace_rustfmt                                   | 78 µs and 128 ns                    | 0.00%      |

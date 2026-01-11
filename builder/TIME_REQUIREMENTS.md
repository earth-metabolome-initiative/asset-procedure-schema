# Time Report for APS Generation

The total time spent on all tasks was 29 seconds.
The slowest task was `Clippy Fixes` which took 27 seconds, 576 ms, 506 µs and 885 ns (93.58% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 136 ms, 319 µs and 497 ns             | 0.46%      |
| Schema Validation                  | 70 ms, 117 µs and 675 ns              | 0.24%      |
| SQL Workspace Generation           | 951 ms, 4 µs and 929 ns               | 3.23%      |
| TOML Formatting                    | 186 ms, 761 µs and 741 ns             | 0.63%      |
| Code Formatting (1)                | 309 ms, 894 µs and 879 ns             | 1.05%      |
| Clippy Fixes                       | 27 seconds, 576 ms, 506 µs and 885 ns | 93.58%     |
| Code Formatting (2)                | 211 ms, 680 µs and 405 ns             | 0.72%      |
| Workspace Dependency Visualization | 12 ms, 283 µs and 441 ns              | 0.04%      |
| DAG Structure Visualization        | 13 ms, 384 µs and 118 ns              | 0.05%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 883 ms, 394 µs and 478 ns (92.89% of all time).

| name                                                | time                      | percentage |
|-----------------------------------------------------|---------------------------|------------|
| writing_crate_toml                                  | 12 ms, 959 µs and 368 ns  | 1.36%      |
| writing_crate_lib                                   | 883 ms, 394 µs and 478 ns | 92.89%     |
| writing_sink_crate_toml                             | 791 µs and 539 ns         | 0.08%      |
| writing_sink_crate_lib                              | 10 ms, 696 µs and 937 ns  | 1.12%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 290 µs and 285 ns   | 0.14%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 10 ms, 152 µs and 222 ns  | 1.07%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 378 µs and 694 ns   | 0.25%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 647 µs and 297 ns   | 0.59%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 879 µs and 52 ns          | 0.09%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 904 µs and 160 ns   | 0.52%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 356 µs and 325 ns   | 0.14%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 175 µs and 91 ns    | 0.33%      |
| workspace_toml                                      | 13 ms, 290 µs and 206 ns  | 1.40%      |
| workspace_rustfmt                                   | 89 µs and 275 ns          | 0.01%      |

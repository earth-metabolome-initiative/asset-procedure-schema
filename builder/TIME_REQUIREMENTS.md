# Time Report for APS Generation

The total time spent on all tasks was 41 seconds.
The slowest task was `Clippy Fixes` which took 39 seconds, 422 ms, 786 µs and 509 ns (95.95% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 50 ms, 332 µs and 269 ns              | 0.12%      |
| Schema Validation                  | 67 ms, 827 µs and 922 ns              | 0.17%      |
| SQL Workspace Generation           | 936 ms, 844 µs and 370 ns             | 2.28%      |
| TOML Formatting                    | 69 ms, 542 µs and 250 ns              | 0.17%      |
| Code Formatting (1)                | 306 ms, 525 µs and 307 ns             | 0.75%      |
| Clippy Fixes                       | 39 seconds, 422 ms, 786 µs and 509 ns | 95.95%     |
| Code Formatting (2)                | 206 ms, 301 µs and 718 ns             | 0.50%      |
| Workspace Dependency Visualization | 12 ms, 122 µs and 495 ns              | 0.03%      |
| DAG Structure Visualization        | 12 ms, 819 µs and 9 ns                | 0.03%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 871 ms, 830 µs and 234 ns (93.06% of all time).

| name                                                | time                      | percentage |
|-----------------------------------------------------|---------------------------|------------|
| writing_crate_toml                                  | 12 ms, 475 µs and 16 ns   | 1.33%      |
| writing_crate_lib                                   | 871 ms, 830 µs and 234 ns | 93.06%     |
| writing_sink_crate_toml                             | 675 µs and 804 ns         | 0.07%      |
| writing_sink_crate_lib                              | 10 ms, 188 µs and 655 ns  | 1.09%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 134 µs and 698 ns   | 0.12%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 9 ms, 663 µs and 139 ns   | 1.03%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 354 µs and 447 ns   | 0.25%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 564 µs and 991 ns   | 0.59%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 872 µs and 842 ns         | 0.09%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 815 µs and 154 ns   | 0.51%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 354 µs and 211 ns   | 0.14%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 258 µs and 777 ns   | 0.35%      |
| workspace_toml                                      | 12 ms, 562 µs and 280 ns  | 1.34%      |
| workspace_rustfmt                                   | 94 µs and 122 ns          | 0.01%      |

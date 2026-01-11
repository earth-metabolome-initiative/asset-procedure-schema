# Time Report for APS Generation

The total time spent on all tasks was 29 seconds.
The slowest task was `Clippy Fixes` which took 27 seconds, 638 ms, 274 µs and 91 ns (93.61% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 109 ms, 379 µs and 653 ns            | 0.37%      |
| Schema Validation                  | 81 ms, 9 µs and 977 ns               | 0.27%      |
| SQL Workspace Generation           | 1 second, 22 ms, 629 µs and 944 ns   | 3.46%      |
| TOML Formatting                    | 127 ms, 854 µs and 66 ns             | 0.43%      |
| Code Formatting (1)                | 314 ms, 316 µs and 608 ns            | 1.06%      |
| Clippy Fixes                       | 27 seconds, 638 ms, 274 µs and 91 ns | 93.61%     |
| Code Formatting (2)                | 207 ms, 674 µs and 130 ns            | 0.70%      |
| Workspace Dependency Visualization | 12 ms and 985 ns                     | 0.04%      |
| DAG Structure Visualization        | 12 ms, 798 µs and 963 ns             | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 953 ms, 749 µs and 992 ns (93.26% of all time).

| name                                                | time                      | percentage |
|-----------------------------------------------------|---------------------------|------------|
| writing_crate_toml                                  | 14 ms, 234 µs and 432 ns  | 1.39%      |
| writing_crate_lib                                   | 953 ms, 749 µs and 992 ns | 93.26%     |
| writing_sink_crate_toml                             | 828 µs and 465 ns         | 0.08%      |
| writing_sink_crate_lib                              | 10 ms, 991 µs and 955 ns  | 1.07%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 283 µs and 846 ns   | 0.13%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 10 ms, 530 µs and 596 ns  | 1.03%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 397 µs and 52 ns    | 0.23%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 855 µs and 463 ns   | 0.57%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 905 µs and 502 ns         | 0.09%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 979 µs and 184 ns   | 0.49%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 362 µs and 785 ns   | 0.13%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 140 µs and 668 ns   | 0.31%      |
| workspace_toml                                      | 12 ms, 279 µs and 737 ns  | 1.20%      |
| workspace_rustfmt                                   | 90 µs and 267 ns          | 0.01%      |

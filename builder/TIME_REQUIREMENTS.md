# Time Report for APS Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Clippy Fixes` which took 15 seconds, 323 ms, 872 µs and 940 ns (90.11% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 119 ms, 833 µs and 615 ns             | 0.70%      |
| Schema Validation                  | 61 ms, 601 µs and 664 ns              | 0.36%      |
| SQL Workspace Generation           | 834 ms, 175 µs and 116 ns             | 4.91%      |
| TOML Formatting                    | 165 ms, 938 µs and 216 ns             | 0.98%      |
| Code Formatting (1)                | 289 ms, 423 µs and 812 ns             | 1.70%      |
| Clippy Fixes                       | 15 seconds, 323 ms, 872 µs and 940 ns | 90.11%     |
| Code Formatting (2)                | 189 ms, 180 µs and 813 ns             | 1.11%      |
| Workspace Dependency Visualization | 10 ms, 853 µs and 920 ns              | 0.06%      |
| DAG Structure Visualization        | 11 ms, 745 µs and 340 ns              | 0.07%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 771 ms, 581 µs and 87 ns (92.50% of all time).

| name                                                | time                     | percentage |
|-----------------------------------------------------|--------------------------|------------|
| writing_crate_toml                                  | 11 ms, 883 µs and 702 ns | 1.42%      |
| writing_crate_lib                                   | 771 ms, 581 µs and 87 ns | 92.50%     |
| writing_sink_crate_toml                             | 782 µs and 84 ns         | 0.09%      |
| writing_sink_crate_lib                              | 10 ms, 508 µs and 366 ns | 1.26%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 218 µs and 55 ns   | 0.15%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 9 ms, 901 µs and 748 ns  | 1.19%      |
| writing_sink_crate_toml_aps-dag-assets              | 1 ms, 970 µs and 825 ns  | 0.24%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 238 µs and 175 ns  | 0.63%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 873 µs and 813 ns        | 0.10%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 776 µs and 134 ns  | 0.57%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 152 µs and 897 ns  | 0.14%      |
| writing_sink_crate_lib_aps-dag-procedures           | 2 ms, 938 µs and 340 ns  | 0.35%      |
| workspace_toml                                      | 11 ms, 257 µs and 671 ns | 1.35%      |
| workspace_rustfmt                                   | 92 µs and 219 ns         | 0.01%      |

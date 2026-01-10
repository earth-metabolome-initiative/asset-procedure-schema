# Time Report for APS Generation

The total time spent on all tasks was 28 seconds.
The slowest task was `Clippy Fixes` which took 26 seconds, 664 ms, 410 µs and 695 ns (93.60% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 141 ms, 543 µs and 681 ns             | 0.50%      |
| Schema Validation                  | 65 ms, 887 µs and 561 ns              | 0.23%      |
| SQL Workspace Generation           | 864 ms, 81 µs and 290 ns              | 3.03%      |
| TOML Formatting                    | 201 ms, 435 µs and 649 ns             | 0.71%      |
| Code Formatting (1)                | 328 ms, 511 µs and 762 ns             | 1.15%      |
| Clippy Fixes                       | 26 seconds, 664 ms, 410 µs and 695 ns | 93.60%     |
| Code Formatting (2)                | 196 ms, 922 µs and 442 ns             | 0.69%      |
| Workspace Dependency Visualization | 11 ms, 163 µs and 92 ns               | 0.04%      |
| DAG Structure Visualization        | 12 ms, 294 µs and 16 ns               | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 800 ms, 962 µs and 315 ns (92.70% of all time).

| name                                                | time                      | percentage |
|-----------------------------------------------------|---------------------------|------------|
| writing_crate_toml                                  | 12 ms, 38 µs and 708 ns   | 1.39%      |
| writing_crate_lib                                   | 800 ms, 962 µs and 315 ns | 92.70%     |
| writing_sink_crate_toml                             | 691 µs and 87 ns          | 0.08%      |
| writing_sink_crate_lib                              | 9 ms, 642 µs and 829 ns   | 1.12%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 126 µs and 457 ns   | 0.13%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 9 ms, 960 µs and 620 ns   | 1.15%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 297 µs and 861 ns   | 0.27%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 556 µs and 868 ns   | 0.64%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 874 µs and 114 ns         | 0.10%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 852 µs and 542 ns   | 0.56%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 335 µs and 323 ns   | 0.15%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 144 µs and 734 ns   | 0.36%      |
| workspace_toml                                      | 11 ms, 511 µs and 461 ns  | 1.33%      |
| workspace_rustfmt                                   | 86 µs and 371 ns          | 0.01%      |

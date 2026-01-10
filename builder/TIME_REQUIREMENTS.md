# Time Report for APS Generation

The total time spent on all tasks was 27 seconds.
The slowest task was `Clippy Fixes` which took 26 seconds, 208 ms, 91 µs and 492 ns (94.00% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 95 ms, 939 µs and 278 ns             | 0.34%      |
| Schema Validation                  | 67 ms, 191 µs and 696 ns             | 0.24%      |
| SQL Workspace Generation           | 865 ms, 786 µs and 187 ns            | 3.11%      |
| TOML Formatting                    | 134 ms, 638 µs and 319 ns            | 0.48%      |
| Code Formatting (1)                | 299 ms, 615 µs and 781 ns            | 1.07%      |
| Clippy Fixes                       | 26 seconds, 208 ms, 91 µs and 492 ns | 94.00%     |
| Code Formatting (2)                | 188 ms, 297 µs and 209 ns            | 0.68%      |
| Workspace Dependency Visualization | 10 ms, 732 µs and 123 ns             | 0.04%      |
| DAG Structure Visualization        | 10 ms, 828 µs and 529 ns             | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 800 ms, 659 µs and 916 ns (92.48% of all time).

| name                                                | time                      | percentage |
|-----------------------------------------------------|---------------------------|------------|
| writing_crate_toml                                  | 11 ms, 994 µs and 857 ns  | 1.39%      |
| writing_crate_lib                                   | 800 ms, 659 µs and 916 ns | 92.48%     |
| writing_sink_crate_toml                             | 764 µs and 408 ns         | 0.09%      |
| writing_sink_crate_lib                              | 11 ms, 201 µs and 995 ns  | 1.29%      |
| writing_sink_crate_toml_aps-dag-asset_models        | 1 ms, 266 µs and 49 ns    | 0.15%      |
| writing_sink_crate_lib_aps-dag-asset_models         | 10 ms, 196 µs and 321 ns  | 1.18%      |
| writing_sink_crate_toml_aps-dag-assets              | 2 ms, 299 µs and 985 ns   | 0.27%      |
| writing_sink_crate_lib_aps-dag-assets               | 5 ms, 558 µs and 703 ns   | 0.64%      |
| writing_sink_crate_toml_aps-dag-procedure_templates | 868 µs and 527 ns         | 0.10%      |
| writing_sink_crate_lib_aps-dag-procedure_templates  | 4 ms, 819 µs and 52 ns    | 0.56%      |
| writing_sink_crate_toml_aps-dag-procedures          | 1 ms, 330 µs and 306 ns   | 0.15%      |
| writing_sink_crate_lib_aps-dag-procedures           | 3 ms, 99 µs and 808 ns    | 0.36%      |
| workspace_toml                                      | 11 ms, 633 µs and 48 ns   | 1.34%      |
| workspace_rustfmt                                   | 93 µs and 212 ns          | 0.01%      |

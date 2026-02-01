# Time Report for APS Generation

The total time spent on all tasks was now.
The slowest task was `Clippy Fixes` which took 3 seconds, 231 ms, 142 µs and 782 ns (48.49% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 267 ms, 93 µs and 285 ns             | 4.01%      |
| Schema Validation                  | 99 ms, 175 µs and 845 ns             | 1.49%      |
| SQL Workspace Generation           | 2 seconds, 89 ms, 56 µs and 713 ns   | 31.35%     |
| TOML Formatting                    | 382 ms, 824 µs and 580 ns            | 5.75%      |
| Code Formatting (1)                | 328 ms, 187 µs and 852 ns            | 4.93%      |
| Clippy Fixes                       | 3 seconds, 231 ms, 142 µs and 782 ns | 48.49%     |
| Code Formatting (2)                | 241 ms, 104 µs and 261 ns            | 3.62%      |
| Workspace Dependency Visualization | 12 ms, 962 µs and 589 ns             | 0.19%      |
| DAG Structure Visualization        | 11 ms, 847 µs and 882 ns             | 0.18%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 637 ms, 882 µs and 137 ns (78.40% of all time).

| name                                     | time                                | percentage |
|------------------------------------------|-------------------------------------|------------|
| writing_crate_toml                       | 412 ms, 417 µs and 785 ns           | 19.74%     |
| writing_crate_lib                        | 1 second, 637 ms, 882 µs and 137 ns | 78.40%     |
| writing_sink_crate_toml                  | 836 µs and 377 ns                   | 0.04%      |
| writing_sink_crate_lib                   | 11 ms, 357 µs and 699 ns            | 0.54%      |
| writing_sink_crate_toml_aps-dag-entities | 1 ms, 2 µs and 899 ns               | 0.05%      |
| writing_sink_crate_lib_aps-dag-entities  | 11 ms, 147 µs and 441 ns            | 0.53%      |
| workspace_toml                           | 14 ms, 287 µs and 886 ns            | 0.68%      |
| workspace_rustfmt                        | 124 µs and 489 ns                   | 0.01%      |

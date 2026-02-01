# Time Report for APS Generation

The total time spent on all tasks was now.
The slowest task was `SQL Workspace Generation` which took 2 seconds, 573 ms, 824 µs and 387 ns (56.43% of all time).

| name                               | time                                 | percentage |
|------------------------------------|--------------------------------------|------------|
| Database Introspection             | 118 ms, 844 µs and 561 ns            | 2.61%      |
| Schema Validation                  | 18 µs and 889 ns                     | 0.00%      |
| SQL Workspace Generation           | 2 seconds, 573 ms, 824 µs and 387 ns | 56.43%     |
| TOML Formatting                    | 148 ms, 824 µs and 123 ns            | 3.26%      |
| Code Formatting (1)                | 305 ms, 604 µs and 778 ns            | 6.70%      |
| Clippy Fixes                       | 1 second, 113 ms, 85 µs and 23 ns    | 24.40%     |
| Code Formatting (2)                | 260 ms, 507 µs and 770 ns            | 5.71%      |
| Workspace Dependency Visualization | 21 ms, 98 µs and 590 ns              | 0.46%      |
| DAG Structure Visualization        | 19 ms, 495 µs and 862 ns             | 0.43%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 2 seconds, 60 ms, 393 µs and 613 ns (80.05% of all time).

| name                                     | time                                | percentage |
|------------------------------------------|-------------------------------------|------------|
| writing_crate_toml                       | 467 ms, 850 µs and 794 ns           | 18.18%     |
| writing_crate_lib                        | 2 seconds, 60 ms, 393 µs and 613 ns | 80.05%     |
| writing_sink_crate_toml                  | 843 µs and 539 ns                   | 0.03%      |
| writing_sink_crate_lib                   | 11 ms, 687 µs and 55 ns             | 0.45%      |
| writing_sink_crate_toml_aps-dag-entities | 897 µs and 882 ns                   | 0.03%      |
| writing_sink_crate_lib_aps-dag-entities  | 11 ms, 73 µs and 465 ns             | 0.43%      |
| workspace_toml                           | 20 ms, 924 µs and 737 ns            | 0.81%      |
| workspace_rustfmt                        | 153 µs and 302 ns                   | 0.01%      |

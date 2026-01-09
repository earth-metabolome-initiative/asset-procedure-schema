# Time Report for APS Generation

The total time spent on all tasks was 18 seconds.
The slowest task was `Clippy Fixes` which took 16 seconds, 544 ms, 896 µs and 522 ns (91.36% of all time).

| name                               | time                                  | percentage |
|------------------------------------|---------------------------------------|------------|
| Database Introspection             | 94 ms, 553 µs and 589 ns              | 0.52%      |
| Schema Validation                  | 61 ms, 796 µs and 341 ns              | 0.34%      |
| SQL Workspace Generation           | 797 ms, 42 µs and 330 ns              | 4.40%      |
| TOML Formatting                    | 129 ms, 672 µs and 355 ns             | 0.72%      |
| Code Formatting (1)                | 282 ms, 918 µs and 11 ns              | 1.56%      |
| Clippy Fixes                       | 16 seconds, 544 ms, 896 µs and 522 ns | 91.36%     |
| Code Formatting (2)                | 177 ms, 534 µs and 10 ns              | 0.98%      |
| Workspace Dependency Visualization | 10 ms, 137 µs and 882 ns              | 0.06%      |
| DAG Structure Visualization        | 11 ms, 135 µs and 263 ns              | 0.06%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 772 ms, 301 µs and 818 ns (96.90% of all time).

| name                    | time                      | percentage |
|-------------------------|---------------------------|------------|
| writing_crate_toml      | 11 ms, 996 µs and 100 ns  | 1.51%      |
| writing_crate_lib       | 772 ms, 301 µs and 818 ns | 96.90%     |
| writing_sink_crate_toml | 684 µs and 467 ns         | 0.09%      |
| writing_sink_crate_lib  | 9 ms, 393 µs and 895 ns   | 1.18%      |
| workspace_toml          | 2 ms, 586 µs and 329 ns   | 0.32%      |
| workspace_rustfmt       | 79 µs and 721 ns          | 0.01%      |

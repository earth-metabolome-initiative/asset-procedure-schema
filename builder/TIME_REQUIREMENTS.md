# Time Report for APS Generation

The total time spent on all tasks was a minute.
The slowest task was `Clippy Fixes` which took 1 minute, 3 seconds, 284 ms, 583 µs and 710 ns (90.90% of all time).

| name                               | time                                           | percentage |
|------------------------------------|------------------------------------------------|------------|
| Database Introspection             | 373 ms, 427 µs and 818 ns                      | 0.54%      |
| Schema Validation                  | 212 ms, 464 µs and 72 ns                       | 0.31%      |
| SQL Workspace Generation           | 4 seconds, 256 ms, 987 µs and 593 ns           | 6.11%      |
| TOML Formatting                    | 471 ms, 237 µs and 264 ns                      | 0.68%      |
| Code Formatting (1)                | 544 ms, 382 µs and 260 ns                      | 0.78%      |
| Clippy Fixes                       | 1 minute, 3 seconds, 284 ms, 583 µs and 710 ns | 90.90%     |
| Code Formatting (2)                | 422 ms, 492 µs and 349 ns                      | 0.61%      |
| Workspace Dependency Visualization | 30 ms, 62 µs and 101 ns                        | 0.04%      |
| DAG Structure Visualization        | 24 ms, 737 µs and 180 ns                       | 0.04%      |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 2 seconds, 980 ms, 308 µs and 441 ns (70.01% of all time).

| name                                     | time                                 | percentage |
|------------------------------------------|--------------------------------------|------------|
| writing_crate_toml                       | 1 second, 222 ms, 813 µs and 172 ns  | 28.72%     |
| writing_crate_lib                        | 2 seconds, 980 ms, 308 µs and 441 ns | 70.01%     |
| writing_sink_crate_toml                  | 716 µs and 116 ns                    | 0.02%      |
| writing_sink_crate_lib                   | 9 ms, 71 µs and 245 ns               | 0.21%      |
| writing_sink_crate_toml_aps-dag-entities | 1 ms, 89 µs and 635 ns               | 0.03%      |
| writing_sink_crate_lib_aps-dag-entities  | 9 ms, 629 µs and 348 ns              | 0.23%      |
| workspace_toml                           | 33 ms, 223 µs and 353 ns             | 0.78%      |
| workspace_rustfmt                        | 136 µs and 283 ns                    | 0.00%      |

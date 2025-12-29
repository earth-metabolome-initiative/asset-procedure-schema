# Time Report for Directus Schema Generation

The total time spent on all tasks was now.
The slowest task was `Code Formatting` which took 142 ms, 661 µs and 25 ns (53.84% of all time).

| name                     | time                     | percentage | comment |
|--------------------------|--------------------------|------------|---------|
| SQL Workspace Generation | 81 ms, 420 µs and 736 ns | 30.73%     |         |
| Code Formatting          | 142 ms, 661 µs and 25 ns | 53.84%     |         |
| TOML Formatting          | 40 ms, 868 µs and 313 ns | 15.42%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 77 ms, 987 µs and 697 ns (95.78% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 1 ms, 824 µs and 774 ns  | 2.24%      |         |
| writing_crate_lib       | 77 ms, 987 µs and 697 ns | 95.78%     |         |
| writing_sink_crate_toml | 167 µs and 103 ns        | 0.21%      |         |
| writing_sink_crate_lib  | 709 µs and 865 ns        | 0.87%      |         |
| workspace_toml          | 658 µs and 297 ns        | 0.81%      |         |
| workspace_rustfmt       | 73 µs                    | 0.09%      |         |

![Plot](TIME_REQUIREMENTS.png)

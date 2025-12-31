# Time Report for Directus Schema Generation

The total time spent on all tasks was 14 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 419 ms, 993 µs and 343 ns (91.40% of all time).

| name                     | time                                  | percentage | comment |
|--------------------------|---------------------------------------|------------|---------|
| SQL Workspace Generation | 808 ms, 339 µs and 78 ns              | 5.51%      |         |
| TOML Formatting          | 108 ms, 858 µs and 137 ns             | 0.74%      |         |
| Code Formatting (1)      | 180 ms, 279 µs and 835 ns             | 1.23%      |         |
| Clippy Fixes             | 13 seconds, 419 ms, 993 µs and 343 ns | 91.40%     |         |
| Code Formatting (2)      | 165 ms, 941 µs and 671 ns             | 1.13%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 782 ms, 58 µs and 324 ns (96.75% of all time).

| name                    | time                     | percentage | comment |
|-------------------------|--------------------------|------------|---------|
| writing_crate_toml      | 12 ms, 992 µs and 278 ns | 1.61%      |         |
| writing_crate_lib       | 782 ms, 58 µs and 324 ns | 96.75%     |         |
| writing_sink_crate_toml | 718 µs and 888 ns        | 0.09%      |         |
| writing_sink_crate_lib  | 9 ms, 869 µs and 30 ns   | 1.22%      |         |
| workspace_toml          | 2 ms, 617 µs and 112 ns  | 0.32%      |         |
| workspace_rustfmt       | 83 µs and 446 ns         | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

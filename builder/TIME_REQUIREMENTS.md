# Time Report for Directus Schema Generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Clippy Fixes` which took 13 seconds, 516 ms, 973 µs and 459 ns (89.09% of all time).

| name                               | time                                  | percentage | comment |
|------------------------------------|---------------------------------------|------------|---------|
| Database Introspection             | 95 ms, 386 µs and 853 ns              | 0.63%      |         |
| Schema Validation                  | 294 ms, 216 µs and 911 ns             | 1.94%      |         |
| SQL Workspace Generation           | 774 ms, 626 µs and 844 ns             | 5.11%      |         |
| TOML Formatting                    | 131 ms, 509 µs and 924 ns             | 0.87%      |         |
| Code Formatting (1)                | 182 ms, 238 µs and 783 ns             | 1.20%      |         |
| Clippy Fixes                       | 13 seconds, 516 ms, 973 µs and 459 ns | 89.09%     |         |
| Code Formatting (2)                | 171 ms, 435 µs and 804 ns             | 1.13%      |         |
| Workspace Dependency Visualization | 6 ms, 569 µs and 708 ns               | 0.04%      |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 749 ms, 557 µs and 130 ns (96.76% of all time).

| name                    | time                      | percentage | comment |
|-------------------------|---------------------------|------------|---------|
| writing_crate_toml      | 11 ms, 800 µs and 420 ns  | 1.52%      |         |
| writing_crate_lib       | 749 ms, 557 µs and 130 ns | 96.76%     |         |
| writing_sink_crate_toml | 706 µs and 700 ns         | 0.09%      |         |
| writing_sink_crate_lib  | 9 ms, 849 µs and 175 ns   | 1.27%      |         |
| workspace_toml          | 2 ms, 634 µs and 540 ns   | 0.34%      |         |
| workspace_rustfmt       | 78 µs and 879 ns          | 0.01%      |         |

![Plot](TIME_REQUIREMENTS.png)

# NFL Data Web Scraper (Rust)
This project was implemented twice to observe performance differences between Python and Rust.

- **Python** - Used plugin customtkinter for UI. https://github.com/jacehalvorson/nfl_stats_prototype

- **Rust** - Used NodeJS to host a server with an API to run the web scraper. Set up a React app to call the API and display the results.

## Instructions
1. Clone the repository
2. In the root directory, run
```
cargo run --release <year> <category>
```
3. Check the /data/ folder for "stats_table.html"

<br>
To compile a binary that can be used elswhere, run

```
cargo build --release
```
and grab the executable from /target/release/. This executable can be run like this:
```
.\nflstatsrequest.exe <year> <category>
```
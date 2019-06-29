# My Home Server

(Name will have to change, 'cause Matrix uses the term "homeserver" to mean something pretty specific and definitely not this. But it'll do for now.)

This project uses a SQLite database and runs a webserver for tracking a games collection. It's for my retro games collection.

## Prerequisites

1. (Windows) Install SQLite3
  * Download and extract somewhere: https://sqlite.org/download.html
  * Add an environment variable `SQLITE3_LIB_DIR` pointing to wherever you installed the lib files.
2. Run `cargo install diesel_cli --no-default-features --features sqlite`
  * This installs the Diesel (database library/tool) command-line interface used for installing and configuring the database file.

## Notes

Not sure if it's needed, but the DB should be `./db/homedb.sqlite`. If you don't have it, you'll probably need to run something like:

* `diesel setup --database-url="./db/homedb.sqlite"` (unverified)
* `diesel migration run`

The migrations should already exist. But if you're going to add one... well, here, you can just read up on how Diesel works: https://github.com/diesel-rs/diesel/tree/master/diesel_cli
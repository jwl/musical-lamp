### musical-lamp
Based off of `zero2prod` from https://zero2prod.com/ and https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

### Setup
* Enable `cargo-watch` to watch for changes and re-compile: `cargo watch -x run`


### CI Steps
* components
  * Automated tests: `cargo test`
  * Code coverage: `docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin`
  * Linting: `cargo clippy -- -D warnings`
  * Formatting check: `cargo fmt -- --check`
  * Security vulnerability check: `cargo audit`
* final CI script based off of https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3#file-general-yml

### DB
* To launch Postgres container manually: `./scripts/init_db.sh`

### Config
* use `configuration.yaml`
* `.env` at project is needed for unit tests

<div align="center">

# API BDD Testing in Rust

</div>

## :bookmark_tabs: Menu

- [Overview](#scroll-overview)
- [Prerequisites](#exclamation-prerequisites)
- [Directory Tree](#open_file_folder-directory-tree)

# :scroll: Overview

A demo project to experiment with BDD API testing in rust.

## :exclamation: Prerequisites

Define the following environment variables in the Dockerfile.

```
ENV API_BASE_URL=
ENV API_KEY=
ENV API_SECRET=
```

Install Docker and run the following command from the project root in the terminal:

```
docker build -t rust-project .
```

Run the following command from the project root in the terminal:

```
docker run --rm -v $(pwd)/output:/usr/src/myapp/output rust-project
```

Follow the progress of the test execution in the terminal.

Check the runtime generated test_execution.log file under the output folder for more details about the test execution.


## :open_file_folder: Directory Tree

```
rust-project
├── output
│   └── test_execution.log (generated runtime)
├── src
│   ├── utils
│   │   ├── mod.rs
│   │   └── sign.rs
│   └── lib.rs
├── tests
│       ├── features
│       │   ├── private.feature
│       │   └── public.feature
│       ├── step_definitions
│       │   ├── mod.rs
│       │   └── step_definitions.rs
│       └── test_runner.rs
├── Cargo.toml
├── Dockerfile
└── readme.md
```


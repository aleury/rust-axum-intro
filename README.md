# rust-axum-intro

This is a project covered in [this course by Jeremy Chone](https://www.youtube.com/watch?v=XZtlD_m59sM).

## Setup

Install the dependencies using

```bash
$ cargo install
```

and then start the application in one terminal:

```bash
$ cargo watch -q -c -w src/ -x run
```

and run the integration tests in another:

```bash
$ cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

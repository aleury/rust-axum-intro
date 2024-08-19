rw:
    cargo watch -q -c -w src/ -x run

tw:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

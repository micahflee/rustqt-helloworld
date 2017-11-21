# Rust Qt hello world test

I'm building this in Fedora 26 because its `qt5-devel` package uses Qt 5.8.0, which is new enough to work with the latest `qt_core` and `qt_widget` crates.

Install dependencies: `dnf install cargo rustc qt5-devel`

Building with `cargo build` works fine, and running the `targets/debug/ works fine as well.

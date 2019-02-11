# rustc_issue_report_58375
Minimal code example to illustrate rustc bug report
[58375](https://github.com/rust-lang/rust/issues/58375).


## How to compile this code

There are several options.  First, the tests will work if you use the following
command (there will be warnings, which can be ignored):

```
$ reset ; RUSTC_BACKTRACE=1 ; cargo -vvv test --no-default-features
```

The compilation will fail if you use the following command:
```
$ reset ; RUSTC_BACKTRACE=1 ; cargo tarpaulin --force-clean -l -- --no-default-features
```

The failure will be as follows:

```
[INFO tarpaulin] Running Tarpaulin
[INFO tarpaulin] Building project
error: internal compiler error: src/librustc/traits/codegen/mod.rs:58: Encountered error `Unimplemented` selecting `Binder(<core::char::DecodeUtf16<<std::vec::Vec<u16> as core::iter::IntoIterator>::IntoIter> as arbitrary::traits::Arbitrary>)` during codegen

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (d17318011 2019-02-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C relocation-model=dynamic-no-pic -C link-dead-code -C opt-level=0 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

[ERROR tarpaulin] Failed to compile tests! Error: build failed
```


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

## **UPDATE on 12 March 2019**

In response to comments that others cannot reproduce my failure, I reran everything using the following commands:

```
$ git clean -xdf
$ RUST_LOG=trace RUST_BACKTRACE=full cargo tarpaulin >> build_output.txt 2>&1
```

`git clean -xdf` ensured that any build product, including the `Cargo.lock` file, was removed before the second command was run.  Thus, `build_output.txt` contains everything that was output.

### META
```
$ rustc -v -V
rustc 1.35.0-nightly (e68bf8ae1 2019-03-11)
binary: rustc
commit-hash: e68bf8ae15ee6c052d0bcc9252386c5c5ee86de2
commit-date: 2019-03-11
host: x86_64-unknown-linux-gnu
release: 1.35.0-nightly
LLVM version: 8.0

$ cargo -v -V
cargo 1.35.0-nightly (95b45eca1 2019-03-06)
release: 1.35.0
commit-hash: 95b45eca19ac785263fed98ecefe540bb47337ac
commit-date: 2019-03-06

$ cargo tarpaulin -Vv
cargo-tarpaulin version: 0.7.0

$ uname -a
Linux rust 4.15.0-46-generic #49-Ubuntu SMP Wed Feb 6 09:33:07 UTC 2019 x86_64 x86_64 x86_64 GNU/Linux

$ lsb_release -a
No LSB modules are available.
Distributor ID:	Ubuntu
Description:	Ubuntu 18.04.2 LTS
Release:	18.04
Codename:	bionic
```

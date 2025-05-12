problems faced:

# 1)
AVM install latest wasn't working had to specify version number than worked fine. (GlibC version error) GLIBC_2.38

# 2)
```bash
error[E0599]: no method named `source_file` found for struct `proc_macro2::Span` in the current scope
   --> /home/learnedprawn/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anchor-syn-0.30.1/src/idl/defined.rs:499:66
    |
499 |                 let source_path = proc_macro2::Span::call_site().source_file().path();
    |                                                                  ^^^^^^^^^^^ method not found in `Span`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `anchor-syn` (lib) due to 1 previous error
```
Change this
```bash
499 |                 let source_path = proc_macro2::Span::call_site().source_file().path();
```
to:

```bash
499 |                 let source_path = proc_macro2::Span::call_site().file();

```
then it compiled successfully.

# 3)
After running anchor build and anchor test in the idl folder day_1.json is expected but day1.json is created.
Renaming it solves the issue.
Update:
Changed this from:
```rust
pub mod day1 {
```

to

```rust
pub mod day_1 {
```

maybe init all anchor projects in snakecase only.
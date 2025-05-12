problems faced:

AVM install latest wasn't working had to specify version number than worked fine. (GlibC version error) GLIBC_2.38

error[E0599]: no method named `source_file` found for struct `proc_macro2::Span` in the current scope
   --> /home/learnedprawn/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anchor-syn-0.30.1/src/idl/defined.rs:499:66
    |
499 |                 let source_path = proc_macro2::Span::call_site().source_file().path();
    |                                                                  ^^^^^^^^^^^ method not found in `Span`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `anchor-syn` (lib) due to 1 previous error

Change this

499 |                 let source_path = proc_macro2::Span::call_site().source_file().path();

to:

499 |                 let source_path = proc_macro2::Span::call_site().file();

then it compiled successfully.

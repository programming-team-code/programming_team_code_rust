# Testing Guidelines
- tests are named `[algo].rs`
- use both yosupo and aizu to test whenever possible because bugs have existed on one of the sites but not the other
- when using both sites name the files `[algo]_yosupo.rs` and `[algo]_aizu.rs`
- when there's no problem to test on, test on [hello world](https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A) and name the files `[algo]_stress.rs`
- when only testing a specific function or componenet of some algorithm name the file `[algo]_[component].rs`

# Documentation Guidelines
- use the `///` syntax for documentation
- for structs, enums, traits, and standalone functions, provide an example testing every method
- for member functions explain what the function does and provide the time and space complexity


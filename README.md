# wowreally - Call `IsWow64Process()` for the current process

This is a tiny Rust program for Windows that calls [`IsWow64Process`](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process) on the current process.

Its purpose is to reveal and verify the behavior of that Windows API function, so it calls it even in situations where its return value could be inferred at compile time.

It uses the [`windows`](https://crates.io/crates/windows) crate to make the Windows API calls.

## Usage

No options are recognized. A line with the single word `true` or `false` is printed to standard output.

This is a trivial command-line utility. It is not currently usable as a library. For that, you may be looking for [`iswow64`](https://crates.io/crates/iswow64).

## License

[0BSD](LICENSE)

## Further reading

- [How to detect programmatically whether you are running on 64-bit Windows](https://devblogs.microsoft.com/oldnewthing/20050201-00/?p=36553) by Raymond Chen

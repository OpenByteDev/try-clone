# try-clone

[![CI](https://github.com/OpenByteDev/try-clone/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/try-clone/actions/workflows/ci.yml) [![crates.io](https://img.shields.io/crates/v/try-clone.svg)](https://crates.io/crates/try-clone) [![Documentation](https://docs.rs/try-clone/badge.svg)](https://docs.rs/try-clone) [![dependency status](https://deps.rs/repo/github/openbytedev/try-clone/status.svg)](https://deps.rs/repo/github/openbytedev/try-clone) [![MIT](https://img.shields.io/crates/l/try-clone.svg)](https://github.com/OpenByteDev/try-clone/blob/master/LICENSE)

Fallible cloning.

This crate defines `TryClone`, a small trait for types whose cloning operation can fail. Unlike [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), which is infallible by design, `TryClone` returns a [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) to allow implementations to report errors.

Implementations are provided for standard library types that expose a `try_clone` API, such as [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html), as well as for common containers and collections ([`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html), [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), ...) where appropriate APIs exists.

A blanket implementation is available behind the `blanket-impl` feature, which implements `TryClone` for all [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) types.

## License
Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/try-clone/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

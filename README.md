# ojcmp

## Note

Altered version for horoscope.

Maintained by ss.

> [experimental] online judge comparer

## Build

```bash
cargo build --release
```

Install by cargo

```bash
cargo install --path .
```

Install manually

```bash
cp target/release/ojcmp /usr/bin
```

## Usage

```
$ ojcmp --help
ojcmp 0.1.2

USAGE:
    ojcmp [FLAGS] [OPTIONS] --std <path>

FLAGS:
    -a, --all        Read all bytes of user file even if it's already WA.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --std <path>     Std file path.
    -u, --user <path>    User file path. Read from stdin if it's not given.
```

## Return Value

| type   | value              |
| ------ | ------------------ |
| stdout | "AC" / "WA" / "PE" |
| stderr | strerror(errno)    |
| code   | errno              |

## Current Implementation

trim_end(file)

```rust
judge!(AC, b"1\r\n\r\n\r\n", b"1  ");
```

trim_end(line)

```rust
judge!(AC, b"1 \n", b"1");
```

check spaces between non-space chars

```rust
judge!(PE, b"1 3\n", b"1         3\n");
```

more test cases: [src/test.rs](https://github.com/Nugine/ojcmp/tree/master/src/test.rs)

## Change Log

+ v0.1.2 fix algorithm bug
+ v0.1.1 use unsafe static buffer
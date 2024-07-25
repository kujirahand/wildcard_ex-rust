# wildcard_ex for Rust

This is a library for extended wildcards that allows VB-like specifications.
It enables the expression of repeating arbitrary strings with simple specifications using wildcards.

## Install the crate

To install the crate, run the following command.

```sh
cargo add wildcard_ex
```

## Example - Basic usage

You just need to call the `is_match(pattern, str)` function as shown below.

```rust
use wildcard_ex::{is_match, ex};
fn main() {
    // match with wildcard characters ['*', '?', '#', "[...]"]
    assert_eq!(is_match("*.txt", "abc.txt"), true);
    assert_eq!(is_match("test*.txt", "test1234.txt"), true);
    // using Pattern object
    let pattern = ex::Pattern::new("*.txt");
    assert_eq!(pattern.is_match("abc.txt"), true);
    assert_eq!(pattern.is_match("abc.zip"), false);
}
```

### Various pattern matching examples

Pattern matching can be performed by specifying wildcard patterns as shown below.

```rust
use wildcard_ex::{is_match_simple, is_match};
fn main() {
    // simple pattern matching with wildcard characters ['*', '?', '#']
    assert_eq!(is_match_simple("*.txt", "abc.txt"), true);
    assert_eq!(is_match_simple("a???.txt", "abcd.txt"), true);
    assert_eq!(is_match_simple("zip:###-####", "zip:111-2222"), true); // '#' is number
    // wildcard "[...]"
    assert_eq!(is_match("[a-z]1234.txt", "a1234.txt"), true);
    assert_eq!(is_match("[a-z][0-9].txt", "b5.txt"), true);
    // not pattern
    assert_eq!(is_match("[!0-9][0-9].txt", "c9.txt"), true);
    // repeating pattern
    assert_eq!(is_match("[+0-9].txt", "12345.txt"), true);
    assert_eq!(is_match("[+a-z0-9].txt", "abc12345.txt"), true);
    // selector
    assert_eq!(is_match("[=cat|dog].txt", "cat.txt"), true);
}
```

- `is_match_simple` specifies general wildcards.
- `is_match` specifies extended wildcards.

## Wildcard patterns

The supported patterns are as follows in the table below.

| Pattern       | Description                                                                       |
| ------------- | --------------------------------------------------------------------------------- |
| *             | Any character repeated 0 or more times                                            |
| ?             | Any single character                                                              |
| #             | Any single digit (=\[0-9\])                                                       |
| \             | Escape character. '\t' means tab, '\n' means newline, '\[' means '['              |
| \[str\]       | Any single character from the specified string `str`                              |
| \[!str\]      | Any single character except those in the specified string `str`                   |
| \[+str\]      | Any character from the specified string `str` repeated 1 or more times            |
| \[-str\]      | Any character except those in the specified string `str` repeated 1 or more times |
| \[=aaa\|bbb\] | The string `aaa` or `bbb`                                                         |

- In \[`str`\], you can specify character codes using \xHH or \uHHHH.

### Extract matched part from beginning

```rust
use wildcard_ex::*;
fn main() {
    assert_eq!(extract_match("*.txt", "abc.txt"), Some("abc.txt".to_string()));
    assert_eq!(extract_match("hello*", "hello, world!"), Some("hello, world!".to_string()));
}
```


### (ja) 拡張ワイルドカード

このクレートは、VBライクな指定が可能な拡張ワイルドカードのライブラリです。
簡単な指定でワイルドカードの任意文字列の繰り返し表現が可能です。
日本語などのマルチバイト文字列も問題なく処理できます。

指定可能なのは次のようなワイルドカードのパターンです。

| パターン       | 説明                                                         |
| ------------- | ----------------------------------------------------------- |
| *             | 任意の文字が0回以上繰り返される                                  |
| ?             | 任意の1文字                                                   |
| #             | 任意の1桁の数字 (=\[0-\9])                                     |
| \             | エスケープ文字。'\t'はタブ、'\n'は改行、'\\\['は'\['を意味する     |
| \[str\]       | 指定された文字列`str`のいずれか1文字                            |
| \[!str\]      | 指定された文字列`str`以外のいずれか1文字                         |
| \[+str\]      | 指定された文字列`str`の任意の文字が1回以上繰り返される              |
| \[-str\]      | 指定された文字列`str`以外の文字が1回以上繰り返される               |
| \[=aaa\|bbb\] | 文字列`aaa`または`bbb`                                        |

- \[`str`\]では、`\xHH`あるいは`\uHHHH`を指定して文字コードを指定できます。

### 簡単な使い方

- `is_match_simple`は一般的なワイルドカードを指定するものです。
- `is_match`は拡張ワイルドカードを指定するものです。

```rust
use wildcard_ex::{is_match_simple, is_match};
fn main() {
    // simple pattern matching with wildcard characters ['*', '?', '#']
    assert_eq!(is_match_simple("*.txt", "abc.txt"), true);
    assert_eq!(is_match_simple("a???.txt", "abcd.txt"), true);
    assert_eq!(is_match_simple("zip:###-####", "zip:111-2222"), true); // '#' is number
    // wildcard "[...]"
    assert_eq!(is_match("[a-z]1234.txt", "a1234.txt"), true);
    assert_eq!(is_match("[a-z][0-9].txt", "b5.txt"), true);
    // not pattern
    assert_eq!(is_match("[!0-9][0-9].txt", "c9.txt"), true);
    // repeating pattern
    assert_eq!(is_match("[+0-9].txt", "12345.txt"), true);
    assert_eq!(is_match("[+a-z0-9].txt", "abc12345.txt"), true);
    // selector
    assert_eq!(is_match("[=cat|dog].txt", "cat.txt"), true);
}
```

### link

- [GitHub Repository](https://github.com/kujirahand/wildcard_ex-rust)
- [Crates.io > wildcard_ex](https://crates.io/crates/wildcard_ex)
- [Document](https://docs.rs/wildcard_ex/)

# wildcard_ex for Rust

This is a library for extended wildcards that allows VB-like specifications.
It enables the expression of repeating arbitrary strings with simple specifications using wildcards.
It supports **multibyte strings** such as Chinese, Japanese, and Korean.

## Install the crate

To install the crate, run the following command.

```sh
cargo add wildcard_ex
```

## Basic usage

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

### Functions

- `is_match_simple` ... specifies general wildcards, checks if the specified text completely matches the pattern and returns true if it. The pattern can include wildcards such as [‘*’, ‘?’, ‘#’].
- `is_match` ... specifies extended wildcards, checks if the specified text completely matches the pattern and returns true if it. The pattern can include wildcards such as [‘*’, ‘?’, ‘#’, “[…]”].
- `extract_match` ... tests whether the text at the beginning matches the pattern and returns the matched part.
- `find_match` ... searches through the entire text from the beginning to find and extract the part that matches the pattern.

## Extract matched part from beginning

The function `extract_match` searches through the entire text from the beginning to find and extract the part that matches the pattern.

```rust
use wildcard_ex::*;
fn main() {
    // extract_match
    assert_eq!(extract_match("*.txt", "abc.txt"), Some("abc.txt".to_string()));
    assert_eq!(extract_match("hello*", "hello, world!"), Some("hello, world!".to_string()));
    // find_match
    let result = find_match("*.txt", "abc.txt").unwrap();
    assert_eq!(result.start, 0);
    assert_eq!(result.matched, "abc.txt".to_string());
}
```

## link

- [GitHub Repository](https://github.com/kujirahand/wildcard_ex-rust)
- [Crates.io > wildcard_ex](https://crates.io/crates/wildcard_ex)
- [Document](https://docs.rs/wildcard_ex/)


## (ja) 拡張ワイルドカード

このクレートは、VBライクな指定が可能な拡張ワイルドカードのライブラリです。
簡単な指定でワイルドカードの任意文字列の繰り返し表現が可能です。
日本語などの**マルチバイト文字列**も問題なく処理できます。

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


### 主な関数

- `is_match_simple(パターン, 文字列)`…一般的なワイルドカードを指定するものです。パターンが文字列に完全にマッチする場合、trueを返します。
- `is_match(パターン, 文字列)`…拡張ワイルドカードを指定するものです。パターンが文字列に完全にマッチする場合、trueを返します。
- `extract_match(パターン, 文字列)`…テキストの先頭からマッチした部分文字列を返します。
- `find_match(パターン, 文字列)`…テキスト全体からマッチする部分を検索して、マッチした位置と部分文字列の構造体を返します。

### 簡単な使い方

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

詳しくは、[ドキュメント](https://docs.rs/wildcard_ex/)をご覧ください。


# wildcard_ex-rust
wildcard library

## Wildcard like VB

| pattern       | description                                                                             |
| ------------- | --------------------------------------------------------------------------------------- |
| *             | Zero or more occurrences of any character                                               |
| ?             | Any single character                                                                    |
| #             | Any single digit                                                                        |
| \             | Escape character. '\t' represents a tab, '\n' represents a newline, '\[' represents '[' |
| \[str\]       | Any single character from the specified string `str`                                    |
| \[!str\]      | Any single character not in the specified string `str`                                  |
| \[*str\]      | Zero or more occurrences of any character from the specified string `str`               |
| \[=aaa\|bbb\] | Either the string `aaa` or `bbb`                                                        |

## (ja) 日本語の解説

| パターン       | 説明                                                         |
| ------------- | ----------------------------------------------------------- |
| *             | 任意の文字が0回以上繰り返される                                  |
| ?             | 任意の1文字                                                   |
| #             | 任意の1桁の数字                                               |
| \             | エスケープ文字。'\t'はタブ、'\n'は改行、'\['は'['を意味する        |
| \[str\]       | 指定された文字列`str`のいずれか1文字                            |
| \[!str\]      | 指定された文字列`str`以外のいずれか1文字                         |
| \[*str\]      | 指定された文字列`str`の任意の文字が0回以上繰り返される              |
| \[=aaa\|bbb\] | 文字列`aaa`または`bbb`                                        |



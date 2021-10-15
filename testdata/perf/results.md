# perf.rs (base implementation)

Results of running `cargo --example perf`

## Release

```
λ cargo run --release --example perf
   Compiling kana v0.1.0 (D:\Work\ax-lab\kana)
    Finished release [optimized] target(s) in 0.52s
     Running `target\release\examples\perf.exe`
=> file read (15634322 bytes)
   took 13.093ms
=> plain sum (15340356 chars = 3798409398)
   took 9.8232ms
=> plain bitwise (196607)
   took 8.7583ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 111.8916ms
```

## Debug

```
λ cargo run --example perf
   Compiling kana v0.1.0 (D:\Work\ax-lab\kana)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target\debug\examples\perf.exe`
=> file read (15634322 bytes)
   took 11.721ms
=> plain sum (15340356 chars = 3798409398)
   took 569.611ms
=> plain bitwise (196607)
   took 564.1717ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 4.2933987s
```

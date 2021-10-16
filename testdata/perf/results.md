# perf.rs (base implementation)

Results of running `cargo --example perf`

## Release

```
   Compiling kana-macros v0.1.0 (D:\Work\ax-lab\kana\kana-macros)
   Compiling kana v0.1.0 (D:\Work\ax-lab\kana)
    Finished release [optimized] target(s) in 1.53s
     Running `target\release\examples\perf.exe`

>> Testing kanjidic.dat...
=> file read (15634322 bytes)
   took 10.3779ms
=> char count (15340356 chars)
   took 8.5963ms
=> plain bitwise (196607)
   took 8.9031ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 106.194ms

>> Testing japanese.dat...
=> file read (6747701 bytes)
   took 10.9343ms
=> char count (2537283 chars)
   took 5.7161ms
=> plain bitwise (196607)
   took 5.7395ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+FULLWIDTH+SMALL+NUMBER+RARE+RADICAL+LINEBREAK)
   took 24.8695ms
```

## Debug

```
λ cargo run --example perf
   Compiling kana v0.1.0 (D:\Work\ax-lab\kana)
    Finished dev [unoptimized + debuginfo] target(s) in 0.86s
     Running `target\debug\examples\perf.exe`

>> Testing kanjidic.dat...
=> file read (15634322 bytes)
   took 13.6094ms
=> char count (15340356 chars)
   took 561.1783ms
=> plain bitwise (196607)
   took 556.3479ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 4.2392532s

>> Testing japanese.dat...
=> file read (6747701 bytes)
   took 11.2286ms
=> char count (2537283 chars)
   took 181.451ms
=> plain bitwise (196607)
   took 186.1117ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+FULLWIDTH+SMALL+NUMBER+RARE+RADICAL+LINEBREAK)
   took 878.48ms
```

# perf.rs (direct lookup optimization)

After adding the direct lookup optimization.

## Release

```
λ cargo run --release --example perf
    Finished release [optimized] target(s) in 0.06s
     Running `target\release\examples\perf.exe`

>> Testing kanjidic.dat...
=> file read (15634322 bytes)
   took 10.3322ms
=> char count (15340356 chars)
   took 8.4585ms
=> plain bitwise (196607)
   took 8.9548ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 40.7697ms

>> Testing japanese.dat...
=> file read (6747701 bytes)
   took 10.464ms
=> char count (2537283 chars)
   took 5.7157ms
=> plain bitwise (196607)
   took 5.6567ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+FULLWIDTH+SMALL+NUMBER+RARE+RADICAL+LINEBREAK)
   took 10.8468ms
```

## Debug

```
λ cargo run --example perf
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target\debug\examples\perf.exe`

>> Testing kanjidic.dat...
=> file read (15634322 bytes)
   took 10.333ms
=> char count (15340356 chars)
   took 535.6575ms
=> plain bitwise (196607)
   took 548.6384ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+SMALL+NUMBER+LINEBREAK)
   took 1.0801537s

>> Testing japanese.dat...
=> file read (6747701 bytes)
   took 11.0546ms
=> char count (2537283 chars)
   took 175.8481ms
=> plain bitwise (196607)
   took 174.2362ms
=> get_flags (JAPANESE+ROMAJI+SPACE+HIRAGANA+KATAKANA+KANA+KANJI+ROMAN+PUNCTUATION+SYMBOL+WORD+FULLWIDTH+SMALL+NUMBER+RARE+RADICAL+LINEBREAK)
   took 270.2702ms
```

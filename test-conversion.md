# Text Conversion Engine

This package implements a generic text conversion engine for the purpose of 
converting between hiragana/katakana/romaji.

Conversion rules are defined at compilation time through a Rust proc-macro. The
resulting `RuleSet`s can be freely combined at runtime to achieve the desired
behavior.

Features of the text-conversion:

- Unicode normalization
- Plain sillable conversion (e.g. `a` -> `あ`, `sya` -> `しゃ`)
- Chained conversions (e.g. `ジャ` -> `じゃ` -> `ja`)
- Multi-stage conversions (e.g. `tyō` -> `tyou` -> `ちょう`)
- Case-sensitivity
- Contextual and function rules
- Position tracking

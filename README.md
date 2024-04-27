# place-name-gen

[![Rust](https://github.com/GossiperLoturot/place-name-gen/actions/workflows/rust.yml/badge.svg)](https://github.com/GossiperLoturot/place-name-gen/actions/workflows/rust.yml)

Open [demo](https://gossiperloturot.github.io/place-name-gen/) for try it!

## Description

This provides a standalone place name generator that runs completely locally.
It collects and randomly samples words before and after US and UK place names that can be divided into two words.
Also, it can localize place names to Japanese.

## Example

```
- downgate
  ダウンゲイトゥ
- burnburg
  バーンバーグ
- webstertree
  ウェブスタートゥリー
- clayown
  クレイオウン
- lowerdean
  ロウアーディーン
```

## References

Place database
- https://github.com/grammakov/USA-cities-and-states
- https://github.com/bwghughes/badbatch

Word database
- https://github.com/reneklacan/symspell
- http://www.argv.org/bep/

# ncat_rust
Rustの勉強を兼ねて欲しいツールを作成。\
超簡易 cat に行番号とシンタックスハイライトを追加したものを目指す。\
その他は使ってみて不満だったところを修正していく。

C++勉強用の [ncat](https://github.com/tsu-kunn/ncat) を Rust で書いてみる。\
C++は仕事でおなかいっぱいな感じが出てきたので、長らく止まっていたRustの勉強を再開する。

[bat](https://github.com/sharkdp/bat) の長劣化版、車輪の再発明。\
batはクレートでも提供されているので、シンタックスハイライトはこれを使うで逃げることができる。\
とにかくこれのソースを熟読してやり方や書き方を学ぶ！

## やりたいこと
- Rustの勉強
- 指定されたテキストファイルを行番号付きで出力
- 特定のファイルはシンタックスハイライトで出力
  - C/C++/sh/markdown の対応を目標
- パイプライン対応

## 調べること
- [ ] シンタックスハイライトのやり方
  - 正規表現を駆使して対応？
    - JavaScriptとか他のものを含めて色々調べた結果、やはり正規表現を使ってる
    - 色付きの出力は bat のソースを読んで学ぶ
  - ライブラリがある？
    - batがクレートとして提供されている[
    - [syntect](https://github.com/trishume/syntect)というライブラリがあった
    - まずはライブラリを使って実装し、その後動作を理解しながら一部を独自の実装にしてみる？
- [x] パイプラインの対応方法
  - 入力と出力の対応方法
    - [isatty](https://docs.rs/isatty/0.1.9/isatty/) のクレートがある
      - 入力: stdin_isatty(), 出力: stdout_isatty()
  - batはファイルの指定がない場合はパイプライン処理としている
    - `$ batcat` と実行すると入力待ちとなるので、自分のやりたいこととはちょっと違う結果になる
- [x] 同じ処理の共通化
  - 同じ処理がちらほら出てきたので共通化したい！
  - 型が違うので難しいならマクロに逃げる？
    - 上手く渡せないものがあったのでひとまずマクロで対応
      - 将来的には関数化したい！

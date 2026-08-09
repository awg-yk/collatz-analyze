# collatz.wasm のビルド方法

index.html に base64 で埋め込まれている WebAssembly モジュールのソースです。

```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
# target/wasm32-unknown-unknown/release/collatz_wasm.wasm を base64 化して
# index.html 内の WASM_B64 定数に貼り付ける
```

u128 固定長演算でジャンプ計算(mod 2^20 の事前計算テーブルを用いた1回のジャンプで
20ステップ相当進める処理)を高速化する。u128 の範囲を超える恐れがある場合は
`test_one` が `-1` を返し、JS 側が BigInt 経路にフォールバックする。

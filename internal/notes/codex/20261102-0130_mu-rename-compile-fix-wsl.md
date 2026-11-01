モデル: gpt-6-luna / max
種別: 前依頼の直し(前依頼: 20261102-0120_mu-rename-compile-fix.md)

# 指示書: WSL の cargo でコンパイルとテストを通す

`internal/notes/codex/20261102-0120_mu-rename-compile-fix.md` の内容のうち、「依頼側で確かめた失敗」「追加の確認」「最終報告」はそのまま有効である。前回は `cargo.exe` が動かず未着手だった。**Windows の `cargo.exe` は使わない。**代わりに WSL の cargo を使う。

## 実行環境

- cargo は `/home/miu/.cargo/bin/cargo` を絶対パスで使う。Rust 1.89.0 で、target `x86_64-pc-windows-msvc` が入っている。
- 依存は取得済みなので、必ず `--offline --locked` を付ける。
- target ディレクトリは `target/wsl` を使う。
- `--target x86_64-pc-windows-msvc` の `check` は Windows 向けの型検査で、実行はできない。

## 完了条件

順に実行して、すべて成功させる。

```text
/home/miu/.cargo/bin/cargo fmt --all
/home/miu/.cargo/bin/cargo check -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --target x86_64-pc-windows-msvc --offline --locked --target-dir target/wsl
/home/miu/.cargo/bin/cargo clippy -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --target x86_64-pc-windows-msvc --offline --locked --target-dir target/wsl -- -D warnings
/home/miu/.cargo/bin/cargo test -p agent-desktop-core --lib --offline --locked --target-dir target/wsl
```

`cargo fmt --all` は、今回の差分のファイルだけを変える。無関係なファイルが変わったら戻す。

`bash scripts/check-rust-file-size.sh` と `bash scripts/check-no-phase-references.sh`、`bash scripts/check-architecture.sh` も実行する。`check-architecture.sh` は `PATH` に `/home/miu/.cargo/bin` を足して実行する。

Windows crate と binary の test は依頼側が Windows の cargo で実行するので、この依頼では実行しない。テストのコードは上の `check --all-targets` で型検査する。

## 最終報告

前回の指示書の「最終報告」の節に従う。最終報告の末尾には `[WAITING_BACKGROUND]` を付ける。一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261102-0130_mu-rename-compile-fix-wsl/` に置く。

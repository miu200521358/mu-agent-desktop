モデル: gpt-6-luna / max
種別: 前依頼の直し(前依頼: 20261101-2320_mu-rename-and-cloaked-target.md)

# 指示書: コンパイルとテストを通す

前依頼の差分は、依頼側の検証で `cargo.exe fmt --check` と `cargo.exe check` が失敗した。前依頼の内容と禁止事項はそのまま有効である。

## 依頼側で確かめた失敗

- `crates/core/src/adapter/system.rs:16`: trait の本体で `include!` は使えない。
- そのため `screenshot_with_filter` と `window_diagnostic` が trait に無い。
- `crates/core/src/locator/live_locator/observation_request.rs:136`: `cloaked_window_target` の初期化が無い。
- `crates/core/src/commands/window_diagnostic_tests.rs:2`: `test_support` は private である。
- `cargo fmt` の差分が多数ある。

core が通った後に、windows crate と binary で別のエラーが出る見込みである。全部を直す。

## 実行環境

この実行では `cargo.exe` が動く想定である。動かないときは、直さずに止めて、エラーの全文を報告する。

## 完了条件

順に実行して、すべて成功させる。

```text
/mnt/c/Users/celes/.cargo/bin/cargo.exe fmt --all
/mnt/c/Users/celes/.cargo/bin/cargo.exe check -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --locked
/mnt/c/Users/celes/.cargo/bin/cargo.exe clippy -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --locked -- -D warnings
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop-core --lib --locked
```

`cargo fmt --all` は、今回の差分のファイルだけを変える。無関係なファイルが変わったら戻す。

Windows crate と binary の test は、前依頼のとおり名前で絞る。fixture の窓を出さないことを読んで確かめてから実行する。

```text
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop-windows --lib --locked window_diagnostic
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop-windows --lib --locked cloaked
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop --bin mu-agent-desktop --locked cloaked
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop --bin mu-agent-desktop --locked contract
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop --bin mu-agent-desktop --locked doc_literal_drift
```

`bash scripts/check-rust-file-size.sh` と `bash scripts/check-no-phase-references.sh` も実行する。

## 追加の確認

- `AGENTS.md` の Command surface は「56 of the 61 command names are operational on macOS」と書いた。`window-diagnostic` は macOS で `not_supported` なので、数字が実態と合うかを確かめ、誤っていれば直す。
- `crates/ffi/include/agent_desktop.h` は `cbindgen` が無ければ手で直さず、残ったことを報告する。

## 最終報告

変更ファイル、実行した検証の結果、実行しなかった検証と理由、残るリスクを書く。`git add` の一覧は差分が変わったときだけ書き直す。

**最終報告の末尾には `[FINAL_REPORT]` ではなく `[WAITING_BACKGROUND]` を付けること。**この報告は Claude Code が検証して人間へ再報告する中間物であり、人間向け通知を抑制するため。

**変更したコードのコメントと行の長さを、変更前と比べた数字で報告すること。**`rg -c '^\s*#'` / `rg -c '^\s*//'` のコメント行数と、`awk 'length > 200'` の行数を、変更前後の両方について書く。今回新しく足した公開 contract・非自明な不変条件・安全境界・workaround を列挙し、対応するコメントの行番号を示すこと。コメントの言語は前依頼の「コメント」の節に従う。

一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261102-0120_mu-rename-compile-fix/` に置くこと。

モデル: gpt-6-luna / max
種別: 前依頼の直し(前依頼: 20261102-0130_mu-rename-compile-fix-wsl.md)

# 指示書: binary のテストの失敗を直す

依頼側が Windows の `cargo.exe` で実行し、次の 5 件が失敗した。前々回の指示書 `20261101-2320_mu-rename-and-cloaked-target.md` の内容と禁止事項は有効である。

| テスト | 失敗の内容 |
|---|---|
| `cli::cloaked_window::tests::headed_and_physical_commands_fail_before_dispatch` | `src/cli/cloaked_window_tests.rs:7`。`mouse-click` の引数がテストで誤っている(`--xy` の形) |
| `doc_literal_drift::documented_command_counts_match_the_cli` | `README.md must state 61 command names and 57 operational commands` |
| `windows_capability_claims::windows_skill_capability_claims_resolve_against_dispatch` | skill の使えないコマンドの表に `window-diagnostic` が入り、`REFUSED_BY_CORE_ON_EVERY_PLATFORM` と不一致 |
| `contract_tests::ci_compares_the_release_binary_to_the_workspace_version` | `ci.yml` の期待値 `agent-desktop $EXPECTED_VERSION` |
| `contract_tests::standard_version_flag_reports_the_package_version` | `src/cli/contract_tests.rs:10` の期待値が `agent-desktop 1.0.2` |

## 直し方の方針

- コマンド数: `src/cli/doc_literal_drift_tests.rs` の計算を読み、数え方を確かめる。`window-diagnostic` は Windows だけで動き、macOS では動かない。テストの意図を保ち、README、AGENTS.md、テストの整合を取る。「57」に合わせるのが誤りなら、テストの計算を直す。結論と理由を報告する。
- skill の表: `window-diagnostic` は Windows で動くので、Windows skill の「使えない」の表に入っていること自体が誤りである。表から外す。
- 実行ファイル名の期待値は `mu-agent-desktop` に直す。`release.yml` の `EXPECTED_OUTPUT` の扱いも `ci.yml` と合わせる。

## 実行環境

WSL の cargo を前回と同じ形で使う。`--offline --locked --target-dir target/wsl` を付ける。

```text
/home/miu/.cargo/bin/cargo fmt --all -- --check
/home/miu/.cargo/bin/cargo clippy -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --target x86_64-pc-windows-msvc --offline --locked --target-dir target/wsl -- -D warnings
```

上の 5 件は Linux target でも実行できる見込みなので、次で試す。依存が足りず build できなければ、理由を報告して飛ばす。Windows の検証は依頼側が行う。

```text
/home/miu/.cargo/bin/cargo test -p agent-desktop --bin mu-agent-desktop --offline --locked --target-dir target/wsl cli::
```

## 最終報告

変更ファイル、検証の結果、実行しなかった検証と理由、残るリスクを書く。

**最終報告の末尾には `[FINAL_REPORT]` ではなく `[WAITING_BACKGROUND]` を付けること。**この報告は Claude Code が検証して人間へ再報告する中間物であり、人間向け通知を抑制するため。

**変更したコードのコメントと行の長さを、変更前と比べた数字で報告すること。**`rg -c '^\s*#'` / `rg -c '^\s*//'` のコメント行数と、`awk 'length > 200'` の行数を、変更前後の両方について書く。今回新しく足した公開 contract・非自明な不変条件・安全境界・workaround があれば列挙し、対応するコメントの行番号を示すこと。

一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261102-0150_mu-rename-contract-tests/` に置くこと。

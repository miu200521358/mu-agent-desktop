**状態: PARTIAL**。主要な実装と検証は通りましたが、Bash版の `check-no-phase-references.sh` はCRLFによる構文エラーで実行できず、その検査は未完了です。

### 変更

- `include!` を trait／impl 内に置けない問題を、モジュールスコープのマクロ経由の展開に修正しました。[core の SystemOps](</mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/system.rs:1>)と[Windows adapter](</mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/adapter.rs:1>)には理由を示すモジュールコメントを加えています。
- `ObservationRequest` の未初期化フィールド、private な `test_support` の参照、`ProcessId`／`HWND`／`WindowHandle` の型不整合、テスト呼び出しの引数不足を修正しました。
- `list_windows_with_filter` を使うテスト adapter にフィルター処理を実装しました。未使用の `resolve_within_deadline` 関数も削除し、Clippyの警告を解消しました。
- [コマンド数テスト](</mnt/d/Codex/tools/mu-agent-desktop/src/cli/contract_command_surface_tests.rs:156>)を61件に更新しました。[README](</mnt/d/Codex/tools/mu-agent-desktop/README.md:110>)と[AGENTS.md](</mnt/d/Codex/tools/mu-agent-desktop/AGENTS.md:258>)には、macOSで56件が動作し、残る5件に `window-diagnostic` が含まれることを記載しました。
- `WindowFilter` の既定値更新構文を使い、[Windows設定テスト](</mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/frame_identity_settings_tests.rs:239>)を399行に収めました。

直接編集したRustファイル20件のHEAD比は、`#` 行が109→124、`//` 行が405→409、200文字超の行が0→0でした。`#` の集計にはRust属性も含みます。

### 検証

- `cargo fmt --all`、`cargo fmt --all -- --check`、Windows MSVC target の `cargo check --all-targets`、Clippy（`-D warnings`）は成功しました。
- coreテストは直列実行で **1,570件成功**しました。並列実行ではartifact lock関連の既存テストが間欠失敗し、個別実行では成功しました。
- Architecture checker本体、Rustコメント規則とその5件のセルフテスト、Rustの400行上限チェック代替確認は成功しました。変更対象ファイルに限定したCRLF考慮の差分チェックも成功しました。
- `check-rust-file-size.sh`、`check-no-phase-references.sh`、`check-architecture.sh` のBash実行は、各スクリプトがCRLFのため失敗しました。Architecture checker本体はPythonから直接実行して成功しましたが、no-phase検査は未確認です。
- 全体の `git diff --check` はCRLF差分の影響を受け、変更対象外の `tests/fixture-app-windows/FixtureCardsClicks.cs:172` にも末尾空白を報告しました。そのファイルは変更していません。
- Windows crate／binaryのテストとGUIテストは指示に従い実行していません。`cargo check --all-targets` はWindows targetの型検査で、Windows上での実行結果ではありません。
- `cbindgen` は見つからず、FFIヘッダーは変更していません。`Cargo.lock`にも差分はありません。

指定された `/tmp` の作業ディレクトリは使っていません。ビルド生成物は `target/wsl` にあり、合計1.5GBです。再利用しない場合は削除可能ですが、ここでは残しています。

Windows側のリポジトリパスは `D:\Codex\tools\mu-agent-desktop` です。コミットとステージングは実行していません。

```text
fix: cloaked-window の型検査失敗を修正

- traitとimplの追加メソッドをマクロ経由で展開
- Windowsのハンドル型と関連テストを修正
- 61コマンドの数とmacOS非対応の説明を同期
```

```text
git add crates/core/src/adapter/system.rs
git add crates/core/src/adapter/system_cloaked_window_ops.rs
git add crates/core/src/locator/live_locator/observation_request.rs
git add crates/core/src/commands/window_diagnostic_tests.rs
git add crates/core/src/refs/ref_resolve_deadline.rs
git add crates/core/src/snapshot/window_target_tests.rs
git add crates/windows/src/system/adapter.rs
git add crates/windows/src/system/adapter_cloaked_window_ops.rs
git add crates/windows/src/system/capture/screenshot.rs
git add crates/windows/src/system/window/diagnostic_tests.rs
git add crates/windows/src/system/window/window_ops.rs
git add crates/windows/src/system/window/window_resolve.rs
git add crates/windows/src/system/window_diagnostic.rs
git add crates/windows/src/tree/observe_tests.rs
git add crates/windows/src/tree/resolve/resolve_retry_tests.rs
git add crates/windows/src/tree/resolve/window_scope.rs
git add crates/windows/src/tree/surface_inventory.rs
git add crates/windows/src/system/window/frame_identity_settings_tests.rs
git add src/cli/contract_command_surface_tests.rs
git add src/cli/wait_for_cli_tests.rs
git add README.md
git add AGENTS.md
```

[WAITING_BACKGROUND]
**状態: PARTIAL**。指示書の修正と指定された Cargo 検証は完了しました。repo-wide の補助チェックと Windows 側の実行検証は未完了です。

変更したファイルです。`README.md` と `AGENTS.md` では、57 件を少なくとも1つの platform で動くコマンド数、56 件を macOS で動く数として明記しました。差の1件は Windows 専用の `window-diagnostic` です。

- [cloaked_window_tests.rs:109](/mnt/d/Codex/tools/mu-agent-desktop/src/cli/cloaked_window_tests.rs:109) — `mouse-click` の座標引数を `--xy 10,20` に修正。Windows: `D:\Codex\tools\mu-agent-desktop\src\cli\cloaked_window_tests.rs`
- [doc_literal_drift_tests.rs:94](/mnt/d/Codex/tools/mu-agent-desktop/src/cli/doc_literal_drift_tests.rs:94) — 57件とmacOSの56件を別々に照合。Windows: `D:\Codex\tools\mu-agent-desktop\src\cli\doc_literal_drift_tests.rs`
- [contract_tests.rs:5](/mnt/d/Codex/tools/mu-agent-desktop/src/cli/contract_tests.rs:5) — `--version` の期待名を `mu-agent-desktop` に変更し、CIとreleaseの出力検査を確認。Windows: `D:\Codex\tools\mu-agent-desktop\src\cli\contract_tests.rs`
- [README.md:110](/mnt/d/Codex/tools/mu-agent-desktop/README.md:110) — コマンド数の platform 範囲を明示し、コマンド表に `window-diagnostic` を追加。Windows: `D:\Codex\tools\mu-agent-desktop\README.md`
- [AGENTS.md:259](/mnt/d/Codex/tools/mu-agent-desktop/AGENTS.md:259) — コマンド数の記述をREADMEと整合。Windows: `D:\Codex\tools\mu-agent-desktop\AGENTS.md`
- [Windows skill:33](/mnt/d/Codex/tools/mu-agent-desktop/skills/agent-desktop-windows/SKILL.md:33) — `window-diagnostic` の状態に `Works.` を加え、テストが利用可能なコマンドとして判定するよう修正。Windows: `D:\Codex\tools\mu-agent-desktop\skills\agent-desktop-windows\SKILL.md`
- [release.yml:191](/mnt/d/Codex/tools/mu-agent-desktop/.github/workflows/release.yml:191) — macOS/Linux/Windowsで `--version` が実行名とrelease versionを返すことを確認。Windows: `D:\Codex\tools\mu-agent-desktop\.github\workflows\release.yml`

[ci.yml:161](/mnt/d/Codex/tools/mu-agent-desktop/.github/workflows/ci.yml:161) は作業開始時点で `mu-agent-desktop $EXPECTED_VERSION` を期待していたため、この依頼では変更していません。Windows: `D:\Codex\tools\mu-agent-desktop\.github\workflows\ci.yml`

**検証**

- `cargo fmt --all -- --check`: PASS
- 指定された Windows target の `cargo clippy --all-targets ... -- -D warnings`: PASS
- 指定された Linux 上の `cargo test ... cli::`: PASS、51件
- 変更した3つのRustファイルはすべて400行未満です。`^\s*#` の件数は変更前後で順に7、3、16、`^\s*//` は0、0、5、200文字超の行は各ファイルとも0で変化なしです。`#` の件数には `#[test]` などの属性も含みます。新しいコメントは追加していません。
- 末尾空白の確認は、CRLFを除いて調べる `awk` で指摘なし。`git diff --check` は作業ツリーにあるCRLF/LF差分を末尾空白として検出しました。既存の改行形式は変更していません。
- `check-rust-file-size.sh`、`check-no-phase-references.sh`、`check-architecture.sh` は各スクリプトのCRLFによりbashが解釈できず、終了コード2でした。エラーは `$'\r': command not found` と `set: pipefail\r: invalid option name` です。改行を直すと共有作業ツリーの既存差分に触れるため、変更していません。
- WindowsでのテストとGUI検証は未実施です。指示書に従い、Windowsでの実行検証は依頼側に残しています。
- `target/wsl` に2.9Gのビルド出力があります。指定された `/tmp/.../20261102-0150_mu-rename-contract-tests/` は未使用です。ビルド出力は再利用しない場合に削除できますが、削除していません。

今回の作業で新しい公開挙動、安全境界、workaroundは追加していません。release workflowには既存の `--version` 契約を検査するCIチェックを追加しました。リポジトリ全体の静的チェックが未確認のため、最終状態はPARTIALです。

```text
fix: コマンド契約とリリース検証を実行名に合わせる

- 座標引数とversion表示の契約テストを修正
- platform別コマンド数を明記
- Windows skillの利用可能コマンド判定を修正
- release binaryのversion出力を検証
```

```bash
git add src/cli/cloaked_window_tests.rs
git add src/cli/doc_literal_drift_tests.rs
git add src/cli/contract_tests.rs
git add README.md
git add AGENTS.md
git add skills/agent-desktop-windows/SKILL.md
git add .github/workflows/release.yml
```

[WAITING_BACKGROUND]
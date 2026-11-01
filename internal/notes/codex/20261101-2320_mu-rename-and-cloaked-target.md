モデル: gpt-6-luna / max
種別: 新規

# 指示書: mu-agent-desktop への名前の分離と、別の仮想デスクトップの窓の明示的な対象指定

## 正本

- 要件: `internal/notes/20261101-2234_movie-specialization-spec.md`
- 設計と決定: `internal/notes/20261101-2305_desktop2-operation-design.md`
- 仕様の「コード変更は停止中」は、人間の指示で解除された。実機の GUI 試験の停止は続いている。
- repo の規約は `AGENTS.md` に従う。特に、ファイル 400 行まで、inline コメント禁止、1 struct 1 ファイル、`unwrap` 禁止。

## 目的

人間がデスクトップ 1 で作業したまま、デスクトップ 2 の窓を読み、意味的に操作し、撮影できる CLI にする。
SetValue の前面化の調査は別件なので、この依頼では扱わない。

## 変更 1: 名前の分離

導入済みの `agent-desktop` 1.0.2 と同時に動いても、状態と IPC が衝突しないようにする。

- 実行ファイルと clap の名前を `mu-agent-desktop` にする(`src/Cargo.toml` の `[[bin]]`、`src/cli/root.rs:13`)。
- 既定の状態の置き場を `.mu-agent-desktop` にする(`crates/core/src/storage/state_root.rs`)。
- 環境変数を `MU_AGENT_DESKTOP_HOME` にする。`AGENT_DESKTOP_HOME` は読まない。
- Windows の ProgramData の置き場、cursor overlay の pipe 名、`IMAGE_STEM`、`.agent-desktop-tmp-p` を分ける(`crates/windows/src/system/interaction_lease/directory.rs`、`cursor_overlay/pipe_name.rs`、`cursor_overlay/image_identity.rs`、`private_file/replace.rs`)。ほかにプロセス間や永続状態で共有する名前が無いかを `rg` で調べ、見つかれば同様に分ける。
- Cargo の package 名と crate 名は変えない。
- 登録面も直す: `CARGO_BIN_EXE_agent-desktop`、`.github/workflows/ci.yml`、`scripts/`、`tests/` の実行ファイル名。`npm/` は変更せず、影響だけを報告する。

## 変更 2: `window-diagnostic` コマンド

読み取り専用の新コマンドを、`AGENTS.md` の「Adding a command」の手順で足す。

- 入力: `--pid <n>` は必須。`--app` と `--window-id` は任意で、指定すれば AND 条件にする。
- 出力は窓ごとに、id、pid、process_instance(取得不能は明示)、visible、iconic、tool、bounds、cloak、virtual_desktop、通常 inventory での採否と理由を持つ。
- cloak は known と unknown を区別し、raw 値と HRESULT を保持する。未知の bit を捨てない。取得失敗を 0 にしない。既存の `is_cloaked` の意味は変えず、別の関数を足す。
- virtual_desktop は公開 COM の `IVirtualDesktopManager` の `IsWindowOnCurrentVirtualDesktop` と `GetWindowDesktopId` で取る。新しい crate は足さず、`windows` crate の interface 宣言で限定的に宣言する。COM の失敗は unknown として HRESULT を返す。
- ref を発行せず、refmap に書かない。cloak の値だけでデスクトップ番号を判定しない。
- macOS と Linux は `not_supported` の既定のままにする。

## 変更 3: `--cloaked-window` による明示的な許可

- 対象コマンドは `snapshot`、`screenshot`、`list-surfaces` と、ref を取る `click`、`set-value`、`clear`、`toggle`、`select`、`expand`、`collapse`、`scroll-to`、`get`、`is`、`wait`。実際のコマンド名は CLI の定義で確かめ、Invoke、Toggle、SelectionItem、ExpandCollapse、Value の意味的操作に対応するものを選ぶ。選んだ一覧を報告する。
- 値は `w-<hwnd>` の 1 窓で、`--pid` の同時指定を必須にする。process_instance を照合する。不一致は `STALE_REF`、形式違いは `INVALID_ARGS` にする。
- 許可は 1 回のコマンドだけに効く。session、batch、環境変数、後続のコマンドへ継承しない。batch の entry では拒否する(`INVALID_ARGS`)。
- 許可した窓だけを通す。同じ PID の別の窓、owned dialog、再生成された窓へ広げない。
- `passes_filter` は変更しない。`WindowFilter` と厳密解決の経路に許可の情報を通し、用途別の判定関数を足す。許可が無いときの動作は完全に今までどおりにする。影響する経路は、`window_ops.rs:158`、`window_resolve.rs:55`、`surface_inventory.rs:88`、`app_ops.rs:107`、`surfaces.rs:87`、`crates/core/src/commands/window_target.rs`、`crates/core/src/system/window_lookup.rs`。
- 許可は visible、非 zero-size、非 tool の条件を緩めない。緩めるのは cloak の除外だけにする。cloak が unknown の窓は許可しない。
- ref の action の再解決で、snapshot の窓と同じ窓に留まることを確かめる。
- `offscreen` と actionability の拒否は解除しない(`crates/core/src/actionability/gates.rs`)。
- 許可付きの実行では、物理入力へ進む経路を拒否する。`--headed` など前面化や実カーソルに依存する指定は `POLICY_DENIED` または `INVALID_ARGS` で止める。ChainRung の `requires_headed` の段に進まないことをテストで示す。
- 前面化して戻す処理、cloak 解除、OS の設定変更を足さない。

## 文書

- `skills/agent-desktop-windows/` に `mu-agent-desktop`、`window-diagnostic`、`--cloaked-window` の使い方と制限を足す。
- `docs/json-output.md` に新しい出力を足す。
- コマンド数は `README.md` と `AGENTS.md` の該当の数字だけを直す。`AGENTS.md` のほかの行と、既存の未 commit の 4 行は変えない。

## 禁止

- commit、push、ファイルの削除、新しい依存の追加、`Cargo.lock` の外部 crate の更新はしない。
- 無関係な差分を直さない。作業ツリーの全ファイルは改行コードの差で modified に見える。**既存ファイルの改行コードを変えない。**
- GUI 窓を出すテストを実行しない。人間が同じ PC で作業中で、Windows の live テストは fixture の窓を出し、`SetForegroundWindow` を呼ぶ。

## テスト

`AGENTS.md` の「Every requirement ... maps to at least one test」に従い、native API を呼ばないテストを足す。仕様 G1 の負例を対象にする: selector の不一致、複数 window、cloak の raw 0/1/2/4、複合、未知 bit、取得失敗、zero-size、invisible、tool、既定 off、batch 非伝播、PID と process_instance の再利用、offscreen の拒否維持、headed の拒否。

## 検証

cargo は Windows 版だけが使える。WSL の素の `cargo` は使わない。次の形で実行する。cwd は repo のルートのままでよい。

```text
/mnt/c/Users/celes/.cargo/bin/cargo.exe fmt --all -- --check
/mnt/c/Users/celes/.cargo/bin/cargo.exe check -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --locked
/mnt/c/Users/celes/.cargo/bin/cargo.exe clippy -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets --locked -- -D warnings
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop-core --lib --locked
```

Windows crate と binary の test は、**足したテストと、変更した領域の非 GUI テストだけを名前で絞って**実行する。実行する前に、そのテストが fixture の窓を出さないことを読んで確かめる。例:

```text
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop-windows --lib --locked window_diagnostic
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop --bin mu-agent-desktop --locked contract
/mnt/c/Users/celes/.cargo/bin/cargo.exe test -p agent-desktop --bin mu-agent-desktop --locked doc_literal_drift
```

`bash scripts/check-rust-file-size.sh`、`bash scripts/check-no-phase-references.sh`、`bash scripts/check-architecture.sh` も実行する。bash で動かないものは理由を報告する。

## コメント

repo の規約が共通規約に優先する。Rust の inline コメントは禁止なので、公開 contract、安全境界、非自明な不変条件は、周囲に合わせて英語の `///` と `//!` で「なぜ必要か」と失敗条件を書く。

## 最終報告

変更ファイル、選んだコマンドの一覧、実行した検証と結果、実行しなかった検証と理由、残るリスクを書く。

**最終報告の末尾には `[FINAL_REPORT]` ではなく `[WAITING_BACKGROUND]` を付けること。**この報告は Claude Code が検証して人間へ再報告する中間物であり、人間向け通知を抑制するため。

**変更したコードのコメントと行の長さを、変更前と比べた数字で報告すること。**`rg -c '^\s*#'` / `rg -c '^\s*//'` のコメント行数と、`awk 'length > 200'` の行数を、変更前後の両方について書く。新しく足した公開 contract・非自明な不変条件・安全境界・workaround を列挙し、それぞれに対応するコメントの行番号を示すこと。ただし言語は上の「コメント」の節に従う。

一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261101-2320_mu-rename-and-cloaked-target/` に置くこと。

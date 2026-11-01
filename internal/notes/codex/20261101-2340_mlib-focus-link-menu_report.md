**状態: PARTIAL**。開発ビルド限定の最前面化メニューと回帰テストを実装しました。Go の実行確認とタスクフローは環境上実行できていません。

**変更内容**

- [control_window.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/controller/control_window.go:213)（`D:\Codex\mlib\mlib_go_t4\pkg\infra\controller\control_window.go`）で、開発ビルドだけにメニューを表示し、設定を `ControlWindowReady` より前に反映するようにしました。通常ビルドでは設定の読込、保存、切替を行いません。
- [viewer_manager.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/viewer/viewer_manager.go:316)（`D:\Codex\mlib\mlib_go_t4\pkg\infra\viewer\viewer_manager.go`）で、無効中の通知を破棄し、`SetWindowPos` の呼出をテスト用に差し替え可能にしました。
- [state.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/state/state.go:726)、[config.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/config/config.go:173)、[state_more_test.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/state/state_more_test.go:153)（それぞれ `D:\Codex\mlib\mlib_go_t4\...`）で、切替時の通知破棄、`focus_link` キー、状態の回帰テストを追加しました。
- [control_window_test.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/controller/control_window_test.go) と [viewer_manager_test.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/viewer/viewer_manager_test.go)（`D:\Codex\mlib\mlib_go_t4\pkg\infra\...`）に、開発ビルド判定、設定の読書き、画面移動連動との独立性、無効時の前面化抑止を確かめるテストを追加しました。
- [keys.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/adapter/mpresenter/messages/keys.go:167) と `common.ja/en/zh/ko.json`（`D:\Codex\mlib\mlib_go_t4\pkg\...`）にメニューキーと翻訳を追加しました。

**検証**

- i18n キー整合性チェックは成功しました。
- `git diff --check` は終了コード 0 でした。Git は変更対象外の既存 CRLF ファイルについて警告を出しました。
- 変更対象の Go ファイルは UTF-8 と LF を維持しています。
- `go.exe test -count=1 ./pkg/shared/... ./pkg/infra/controller/... ./pkg/infra/viewer/...`、指定の `go.exe vet`、`gofmt.exe -d` は、いずれも起動時に `WSL (2 - ) ERROR: UtilBindVsockAnyPort:317: socket failed 1` で失敗しました。テスト、vet、書式検査の結果は得られていません。各テストの修正外し確認も未実施です。追加テストは GUI 窓を作成しませんが、実行自体ができていません。
- 指定された `ti` は `/mnt/c/Codex/scripts/ti` と `/mnt/d/Codex/scripts/ti` のどちらにもなく、`PATH` 上にもありません。必須のタスクフローは未実行です。
- ヘッダー付与スクリプトは実行し、新規 Go テストへヘッダーを追加しました。スクリプトが列挙した範囲外の既存 `pkg/domain/deform/bake.go` は元の状態へ戻しており、最終差分には含まれません。
- 変更対象 Go ファイルのコメント行数は `#` が 0→0、`//` が 478→497 です。200文字超の行は 3→3 で、新規追加行にはありません。
- 一時物は指定領域の `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261101-2340_mlib-focus-link-menu/bin/python`（`/usr/bin/python3` へのシンボリックリンク）1個、16 bytes です。100 MBを超える一時物はありません。

設計メモの前提どおり、観測された foreground 移動の起点が mlib 外部にある場合、このメニューでその移動が止まるかは未確認です。GUIでの受入確認も未実施です。

**commit message 案**

```text
開発ビルドに最前面化連動メニューを追加

- 開発ビルド限定の設定と保存を追加
- 無効化時に保留中のフォーカス通知を破棄
- 4言語のメニュー表示と回帰テストを追加
```

**`git add` 案**

```text
git add pkg/adapter/mpresenter/messages/keys.go
git add pkg/infra/base/i18n/i18n/common.en.json
git add pkg/infra/base/i18n/i18n/common.ja.json
git add pkg/infra/base/i18n/i18n/common.ko.json
git add pkg/infra/base/i18n/i18n/common.zh.json
git add pkg/infra/controller/control_window.go
git add pkg/infra/controller/control_window_test.go
git add pkg/infra/viewer/viewer_manager.go
git add pkg/infra/viewer/viewer_manager_test.go
git add pkg/shared/base/config/config.go
git add pkg/shared/state/state.go
git add pkg/shared/state/state_more_test.go
```

[WAITING_BACKGROUND]
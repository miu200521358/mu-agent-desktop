## 状態: PARTIAL

依頼書に沿って3 repoへフォーカス調査ログを実装しました。i18n確認と差分チェックは通りましたが、指定の Go テストと `gofmt` は実行できず、ビルド結果は未確認です。実装内容と制限を以下にまとめます。

## 変更ファイル

**`D:\Codex\mlib\walk`**

- [focus_trace.go](/mnt/d/Codex/mlib/walk/pkg/walk/focus_trace.go:8)：依存のない atomic sink と固定イベント名を追加。
- [lineedit.go](/mnt/d/Codex/mlib/walk/pkg/walk/lineedit.go:383)：`WM_COMMAND` の通知コード、Enter とフォーカス喪失時の EditingFinished を記録。
- [window.go](/mnt/d/Codex/mlib/walk/pkg/walk/window.go:1959)：`SetFocus` と指定された Win32 メッセージを記録。
- [layout.go](/mnt/d/Codex/mlib/walk/pkg/walk/layout.go:331)、[container.go](/mnt/d/Codex/mlib/walk/pkg/walk/container.go:449)：layout 分岐と候補 HWND の選択・フォーカス前後を記録。
- [form.go](/mnt/d/Codex/mlib/walk/pkg/walk/form.go:746)：`WM_ACTIVATE` 後のフォーカス復元を記録。
- [tabwidget.go](/mnt/d/Codex/mlib/walk/pkg/walk/tabwidget.go:309)：既存呼び出しを新しい private helper の引数に合わせて更新。

**`D:\Codex\mlib\mlib_go_t4`**

- [run.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/app/run.go:63)：開発ビルドかつ `MU_FOCUS_TRACE=1` の場合だけ sink を接続し、終了時に解除。
- [focus_trace.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace.go:60)：限定項目を JSONL に記録する recorder を追加。
- [control_window.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/controller/control_window.go:533)：`focus_link` の反映値を記録。
- [viewer_manager.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/viewer/viewer_manager.go:316)：実行時の設定変化、処理分岐、前面化と `Focus()` の前後を記録。
- [focus_trace_test.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace_test.go:15)：開発ビルド・環境変数の判定、無効時にファイルを作らないこと、出力項目を確認するテストを追加。
- [focus_trace_windows_test.go](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace_windows_test.go:12)：sink の有効化・解除を確認する Windows 用テストを追加。

**`D:\Codex\mlib\mu_tree_viewer`**

- [tree_view.go](/mnt/d/Codex/mlib/mu_tree_viewer/pkg/infra/controller/ui/tree_view.go:138)：フォーカス復元ガードの判定結果を記録。
- [state.go](/mnt/d/Codex/mlib/mu_tree_viewer/pkg/infra/controller/ui/state.go:280)：TreeView の復元フォーカス前後を記録。
- [folder_picker.go](/mnt/d/Codex/mlib/mu_tree_viewer/pkg/infra/controller/ui/folder_picker.go:126)：EditingFinished、`handlePathChanged`、`applyPaths` の入口をパスなしで記録。

`target_hwnd` は、選択した HWND を記録するために追加した数値項目です。指示書の「選択した HWND」の要件を満たすための拡張であり、固定イベント名と限定 flag 以外の任意データは記録しません。

## 契約・安全境界のコメント

- イベント情報を限定し、メッセージ引数全体を渡さない契約：[walk/focus_trace.go:11-12](/mnt/d/Codex/mlib/walk/pkg/walk/focus_trace.go:11)
- sink の設定と、未設定時は処理しない契約：[walk/focus_trace.go:49-59](/mnt/d/Codex/mlib/walk/pkg/walk/focus_trace.go:49)
- 開発ビルドと環境変数による有効条件：[focustrace/focus_trace.go:60-65](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace.go:60)、[run.go:140-145](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/app/run.go:140)
- 書き込み失敗を呼び出し元へ伝播しない契約：[focustrace/focus_trace.go:93-95](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace.go:93)
- HWND 取得と JSON 化をファイルロックの外で行う境界：[focustrace/focus_trace.go:122](/mnt/d/Codex/mlib/mlib_go_t4/pkg/shared/base/focustrace/focus_trace.go:122)、[run.go:169-177](/mnt/d/Codex/mlib/mlib_go_t4/pkg/infra/app/run.go:169)
- 許可した Win32 メッセージ情報だけを記録する方針：[window.go:2199-2239](/mnt/d/Codex/mlib/walk/pkg/walk/window.go:2199)

## 検証

- i18n キーチェック：成功。`keys.go` と `app/common` の一致を確認。
- 対象ファイルの `git diff --check`：3 repo とも終了コード 0。`walk` では CRLF に関する Git の警告が出ましたが、変更ファイルの CRLF は維持されています。
- 新規 Go ファイルの instruction header：対象を限定した dry-run で全ファイル付与済みを確認。
- 読み取りレビュー：指定計測点、処理順、通知コードだけの記録、パス・文字列を出力しない構造を確認。レビュー担当から追加ブロッカーの指摘はありませんでした。
- Go テストと `gofmt.exe -d`：`go.exe version` が `UtilBindVsockAnyPort` エラーで起動できず、依頼書の指示どおりテストは実行していません。WSL 側に `go` コマンドもありません。
- 必須タスクフロー `ti`：`ti` と `/mnt/c/Codex/scripts/ti` が見つからず実行できませんでした。
- GUI と EXE は起動していません。

開発ビルドで試す場合は、PowerShell で `$env:MU_FOCUS_TRACE = "1"` を設定して起動してください。ログはアプリルートの `logs\focus_trace_<開始時刻>.jsonl` に出ます。試験後は `Remove-Item Env:MU_FOCUS_TRACE` で環境変数を解除できます。実機での挙動確認は未実施です。

一時物は指定された `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261102-0055_focus-trace-instrumentation/` にまとめ、合計 132K です。100MB を超える一時物やテストキャッシュはありません。

## コメント行・長行の変更前後

変更前は既存ファイルの `HEAD`、新規ファイルは 0 として比較しています。`#` コメント行は変更前・変更後とも全ファイル 0 行です。

| Repo / ファイル | `//` 行 変更前→後 | 200文字超 変更前→後 |
|---|---:|---:|
| walk `container.go` | 13→15 | 0→0 |
| walk `form.go` | 34→34 | 0→0 |
| walk `layout.go` | 26→26 | 0→0 |
| walk `lineedit.go` | 15→15 | 0→0 |
| walk `tabwidget.go` | 17→17 | 0→0 |
| walk `window.go` | 471→472 | 0→0 |
| walk `focus_trace.go`（新規） | 0→7 | 0→0 |
| mlib `run.go` | 8→11 | 0→0 |
| mlib `control_window.go` | 168→168 | 1→1 |
| mlib `viewer_manager.go` | 81→81 | 2→2 |
| mlib `focustrace/focus_trace.go`（新規） | 0→11 | 0→0 |
| mlib `focustrace/focus_trace_test.go`（新規） | 0→1 | 0→0 |
| mlib `focustrace/focus_trace_windows_test.go`（新規） | 0→2 | 0→0 |
| mu_tree `tree_view.go` | 44→44 | 0→0 |
| mu_tree `state.go` | 29→29 | 0→0 |
| mu_tree `folder_picker.go` | 32→32 | 0→0 |
| **合計** | **938→965** | **3→3** |

## commit 提案

以下は提案のみです。実行していません。

**walk**

```text
フォーカス調査用イベントを追加

- atomic sink と固定イベント名を追加
- focus、layout、EditingFinished の観測点を記録
```

```powershell
git -C "D:\Codex\mlib\walk" add pkg/walk/focus_trace.go
git -C "D:\Codex\mlib\walk" add pkg/walk/lineedit.go
git -C "D:\Codex\mlib\walk" add pkg/walk/window.go
git -C "D:\Codex\mlib\walk" add pkg/walk/layout.go
git -C "D:\Codex\mlib\walk" add pkg/walk/container.go
git -C "D:\Codex\mlib\walk" add pkg/walk/form.go
git -C "D:\Codex\mlib\walk" add pkg/walk/tabwidget.go
```

**mlib_go_t4**

```text
開発時のフォーカス調査ログを記録

- 開発ビルドと環境変数で JSONL 記録を制御
- focus_link とフォーカス復元の観測点を追加
- recorder の出力制限と有効条件のテストを追加
```

```powershell
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/infra/app/run.go
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/infra/controller/control_window.go
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/infra/viewer/viewer_manager.go
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/shared/base/focustrace/focus_trace.go
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/shared/base/focustrace/focus_trace_test.go
git -C "D:\Codex\mlib\mlib_go_t4" add pkg/shared/base/focustrace/focus_trace_windows_test.go
```

**mu_tree_viewer**

```text
フォーカス復元とパス適用の調査イベントを追加

- TreeView の復元ガードと処理前後を記録
- FolderPicker の入口をパスなしで記録
```

```powershell
git -C "D:\Codex\mlib\mu_tree_viewer" add pkg/infra/controller/ui/tree_view.go
git -C "D:\Codex\mlib\mu_tree_viewer" add pkg/infra/controller/ui/state.go
git -C "D:\Codex\mlib\mu_tree_viewer" add pkg/infra/controller/ui/folder_picker.go
```

[WAITING_BACKGROUND]
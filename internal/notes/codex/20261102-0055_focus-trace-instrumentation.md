モデル: gpt-6-luna / max
種別: 新規

# 指示書: 前面化の要因を切り分ける開発用ログ

## 背景

UIA の `ValuePattern.SetValue` の後に、MuTreeViewer の操作パネルが foreground になる。`画面移動連動` と `最前面化` の保存値を両方 OFF にしても残った。起点は未特定である。
設計の背景: `/mnt/d/Codex/tools/mu-agent-desktop/internal/notes/20261101-2335_mlib-focus-link-menu-design.md`

1 回の試験で次を比べられるログを足す。

- 最初の foreground の変化より前に、アプリ側の SetFocus や layout の分岐が走ったか
- `focus_link` が実行時に本当に false だったか
- フォルダの適用が、値の変更時か focus の喪失時か

時刻だけで責任を断定しない。呼び出し元の確定は、この依頼の範囲外である。

## 対象

作業ディレクトリは `/mnt/d/Codex/mlib/mlib_go_t4`。変更してよいのは次の 3 repo だけである。それぞれの `AGENTS.md` に従う。

- `/mnt/d/Codex/mlib/walk`
- `/mnt/d/Codex/mlib/mlib_go_t4`
- `/mnt/d/Codex/mlib/mu_tree_viewer`

`/mnt/d/Codex/mlib/win` は変更しない。各箇所の行番号は調査時点の目安なので、実ファイルで確かめる。

## 設計

1. `walk` に、依存を持たない最小の sink を足す。package 変数の関数を atomic に保持し、未設定なら何もしない。`mlib` への依存を足さない。
2. `mlib_go_t4` が sink を実装する。**開発ビルド(`appConfig.IsDev()`)で、かつ環境変数 `MU_FOCUS_TRACE=1` のときだけ**有効にする。通常ビルドと未設定のときは、ファイルを作らず、呼び出しの負荷もほぼ無いこと。
3. 出力はアプリのルートの `logs/focus_trace_<開始時刻>.jsonl` に 1 行 1 event で書く。書き込みの失敗で本処理を止めない。ログのロックの中で Win32 のメッセージを送らない。
4. 1 event の項目: UTC 時刻、単調時計の時刻、thread id、固定の event 名、child HWND、root HWND、その時点の foreground HWND、`GetFocus` の HWND、限定の flag。

## 記録しないもの

文字列、パス、モデル名、`WM_SETTEXT` の本文、wParam と lParam の全体、他の窓のタイトル、キー入力。WM_COMMAND は通知コードだけ、WM_ACTIVATE は状態の下位ワードだけにする。

## 計測点

| 箇所 | event |
|---|---|
| `walk/pkg/walk/lineedit.go` の WM_COMMAND と EN_CHANGE の周辺 | 通知コード |
| `lineedit.go` の EditingFinished の発行(Enter、WM_KILLFOCUS) | 発行 |
| `walk` の WM_SETTEXT の受信(`window.go`) | 受信(本文なし) |
| `walk` の WM_SETFOCUS、WM_KILLFOCUS、WM_ACTIVATE、WM_ENABLE の入口 | 入口と結果 |
| `walk/pkg/walk/layout.go` の `applyLayoutResults` の focus 分岐 | 条件の真偽と選択した HWND |
| `walk/pkg/walk/container.go` の `focusFirstCandidateDescendant` | 呼び出しの前後 |
| `walk/pkg/walk/form.go` の WM_ACTIVATE 後の focus 復元 | 前後 |
| `walk/pkg/walk/window.go` の Win32 `SetFocus` の呼び出し | 前後 |
| `mlib_go_t4` の `applyFocusLinkUserConfig` | 反映後の値 |
| `mlib_go_t4` の `handleWindowFocus` | flag の変化時と処理の実行時だけ(毎ループ書かない) |
| `mlib_go_t4` の `bringWindowToFrontNoActivate`、`windowList[i].Focus()` | 前後 |
| `mu_tree_viewer/pkg/infra/controller/ui/tree_view.go` の Focus のガード | 判定の結果 |
| `mu_tree_viewer/pkg/infra/controller/ui/state.go` の処理後 Focus | 前後 |
| `mu_tree_viewer` の folder picker の `handlePathChanged`、`applyPaths`、EditingFinished | 入口(パスなし) |

`mu_motion_viewer` と `mu_model_viewer` は変更しない。

## 禁止

- commit、push、ファイルの削除、依存の外部追加をしない。
- 既存の処理の順序、前面化、focus の動作を変えない。ログを足すだけにする。
- `EN_CHANGE` の TextChanged の抑制を戻さない。
- 無関係な差分を戻さず、触らない。既存ファイルの改行コードを変えない。
- GUI 窓を出す試験と EXE の起動をしない。

## テスト

`mlib_go_t4` に、sink の有効と無効、開発ビルドの判定、環境変数の判定、記録しない項目が出力に入らないことのテストを足す。

## 検証

このサンドボックスでは `go.exe` が `UtilBindVsockAnyPort` で起動できないことがある。起動できなければ実行せず、その旨を報告する。依頼側が実行する。

```text
/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe test -C D:\Codex\mlib\mlib_go_t4 ./pkg/shared/...
```

## 最終報告

変更ファイルを repo ごとに書く。試験の手順(環境変数の設定、ログの場所)、実行した検証と結果、実行しなかった検証と理由、残るリスクを書く。

**最終報告の末尾には `[FINAL_REPORT]` ではなく `[WAITING_BACKGROUND]` を付けること。**この報告は Claude Code が検証して人間へ再報告する中間物であり、人間向け通知を抑制するため。

**変更したコードのコメントと行の長さを、変更前と比べた数字で報告すること。**`rg -c '^\s*#'` / `rg -c '^\s*//'` のコメント行数と、`awk 'length > 200'` の行数を、変更前後の両方について書く。新しく足した公開 contract・非自明な不変条件・安全境界・workaround を列挙し、それぞれに対応する日本語コメントの行番号を示すこと。

一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261102-0055_focus-trace-instrumentation/` に置くこと。

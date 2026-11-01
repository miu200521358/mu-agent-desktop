# mlib の「最前面化」メニューの設計

作成: 2026-11-01 23:35 JST / 担当: agent-desktop / 状態: 実装依頼前の設計
実装先: `D:\Codex\mlib\mlib_go_t4\` / 実装担当: 別の Codex(人間が依頼する)
入力: 掲示板スレッド `d3a3406c-c8ab-449e-8c37-289053036a42` の Tuesday の追記(人間の仕様追加、2026-11-01 14:24 UTC)

行番号は 2026-11-01 時点の作業ツリーで読んだもの。下の事実はすべてソースの読み取りで、ビルドと実機の確認はしていない。

## 問題定義

- actor: 録画の操作を行う Codex と、同じ PC で作業する人間。
- 目的: 開発ビルドで mlib の自動的な前面化を止め、前面化の要因を切り分ける。
- 成功条件: OFF の間は mlib が能動的な Z-order の引上げと連動 Focus を行わず、ON と通常ビルドは従来どおり動く。

## 先に知っておくこと

**このメニューだけで SetValue の前面化が止まるとは見込めない(確度 Mid)。**

- mlib のコントローラー起点の経路は `SWP_NOACTIVATE` の `SetWindowPos` だけで、foreground を移さない(`pkg/infra/viewer/viewer_manager.go:318-323`、`:368`)。
- mlib で `Focus()` を呼ぶのはビューワーの経路だけである(`viewer_manager.go:330`)。`pkg` 全体の `rg` で、`SetForegroundWindow`、`SetActiveWindow`、`BringWindowToTop` は見つからなかった。
- 観測された foreground の移動先は操作パネルの HWND である(仕様 O05、Tuesday の 14:22 UTC の試験)。
- したがって、foreground の移動の開始は mlib 以外(Walk、UIA provider、各ツール)にある見込みが高い。Z-order の引上げは止まるが、foreground の移動は残る可能性がある。
- 確度を Mid に留める理由: 配布 EXE の依存が今回読んだソースと同じかは未確定で、Walk は読んでいない。

切り分けの道具としては有効なので、この前提で実装し、OFF での実測を結果として扱う。

## 現状

| 箇所 | 事実 |
|---|---|
| `pkg/shared/state/state.go:427` | `focusLinkEnabled` の初期値は true |
| `state.go:727` | `SetFocusLinkEnabled` は値を入れるだけ。非テストの呼び出し元が無い |
| `viewer_manager.go:311-314` | 無効なら早期 return し、保留の通知を**消さない** |
| `pkg/infra/controller/control_window.go:812-816` | `onActivate` は無効でも `SetControlWindowFocused(true)` を立てる |
| `control_window.go:215-216` | ready の直後に Focused を立てる |
| `control_window.go:236` | `applyUserConfig` はその後に走る |
| `pkg/infra/app/run.go:106,128` | controller と viewer のループは別に動く |
| `control_window.go:980` | 開発ビルドの項目は `cw.appConfig.IsDev()` の中で足す |

## 設計

1. `SharedState` の既存 `SetFocusLinkEnabled` を使う。新しい状態を作らない。
2. `handleWindowFocus` の無効時の分岐で保留の通知を捨てる。`SetControlWindowFocused(false)`、`SetAllViewerWindowsFocused(false)`、`KeepFocus()` を呼んでから return する。これで OFF 中の `onActivate` の通知も再 ON の後に発火しない。
3. `SetFocusLinkEnabled` でも、値が変わるときに同じ通知を消す。メニューの切替は UI スレッドで走り、次のループを待たずに再 ON されても古い通知を残さないため。世代の管理は入れない。
4. 開発ビルドの判定の中に、チェック項目 `&最前面化` を足す。`ControlWindowKey` は既存の最大の番号の次を使い、`common.ja.json`、`common.en.json`、`common.zh.json`、`common.ko.json` の全部に足す。
5. `triggerFocusLink` と `SetFocusLinkMenuEnabled` 相当の処理は `triggerWindowLinkage`(`control_window.go:1431`)の形に合わせる。`画面移動連動` とは状態を共有しない。
6. 保存キーは `UserConfigKeyFocusLink = "focus_link"` とする(`pkg/shared/base/config/config.go`)。**開発ビルドのときだけ読み書きする。**通常ビルドは読まないので、開発時の OFF が持ち越されない。既定は ON。
7. 起動時の適用は `control_window.go:215` の `SetControlWindowReady(true)` より前に行う。`:236` の `applyUserConfig` のままでは、viewer のループが先に初回の前面化を実行する恐れがある。

## 範囲の外

- 各ツールの処理後 Focus(`mu_tree_viewer/pkg/infra/controller/ui/state.go:273`、`tree_view.go:112`)は別 repo である。既に前面のときだけ許可するガードがあるので、今回は変えない。Model と Motion の同種の経路の監査も別の作業にする。
- エラーやファイルのダイアログ、OS と UIA provider による activation は止めない。
- 人間のクリックによる activation は止めない。OFF は連動だけを止める。

## テスト

`pkg/shared/state/state_more_test.go` の形で足す。

- OFF で `TriggerLinkedFocus` が通知を立てない。
- OFF 中に `SetControlWindowFocused(true)` を立てて再 ON しても、通知が残らない。
- OFF 中の `handleWindowFocus` が保留の通知を捨て、`SetWindowPos` を呼ばない。win32 呼び出しの seam が要るなら関数の差し替えで足す。
- `SetWindowLinkageEnabled` と `SetFocusLinkEnabled` が互いの状態を変えない。
- `IsDev()` が false のとき、項目が足されず、`focus_link` を読まず、有効のままである。

## 検証

`go.exe` は `/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe` を使う。mlib の `AGENTS.md` と `mlib-guiding-principles` に従う。

```text
/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe test ./pkg/shared/... ./pkg/infra/controller/... ./pkg/infra/viewer/...
```

GUI の確認は人間の承認を得て、Tuesday の受入確認 1〜5 を行う。値の一致だけで合格にせず、foreground、focus、Z-order を記録する。

## 人間に確かめたいこと

上の 6 は担当の仮定である。開発ビルドの既定を ON にし、選択を開発ビルドだけ保存する。録画のたびに OFF にする手間を減らすために保存を入れた。

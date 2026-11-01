**HOLD — 実装は未着手です。**

指示書 ([20261102-0120_mu-rename-compile-fix.md](/mnt/d/Codex/tools/mu-agent-desktop/internal/notes/codex/20261102-0120_mu-rename-compile-fix.md)) は、Windows 版 `cargo.exe` が動かない場合は修正せず停止するよう定めています。起動確認は終了コード `1` で失敗しました。

```text
<3>WSL (2 - ) ERROR: UtilBindVsockAnyPort:317: socket failed 1
```

そのため、指定の `fmt`、`check`、`clippy`、テスト、追加チェックは実行していません。ソース変更や一時ファイルの作成もありません。作業ツリーには既存の差分がありましたが、変更していません。

対象 note の Windows パス: `D:\Codex\tools\mu-agent-desktop\internal\notes\codex\20261102-0120_mu-rename-compile-fix.md`

[WAITING_BACKGROUND]
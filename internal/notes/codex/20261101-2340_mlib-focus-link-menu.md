モデル: gpt-6-luna / max
種別: 新規

# 指示書: mlib の開発ビルド限定「最前面化」メニュー

## 正本

`/mnt/d/Codex/tools/mu-agent-desktop/internal/notes/20261101-2335_mlib-focus-link-menu-design.md` の設計に従う。
対象 repo は `/mnt/d/Codex/mlib/mlib_go_t4`。その repo の `AGENTS.md` と、そこが指す `mlib-guiding-principles` に従う。

## 変更

設計の「設計」の 1〜7 と「テスト」を実装する。設計の行番号は目安なので、実ファイルで確かめる。

## 禁止

- commit、push、ファイルの削除、依存の追加をしない。
- 設計の「範囲の外」に触れない。`mu_tree_viewer` など別の repo は変更しない。
- 無関係な差分を戻さず、触らない。既存ファイルの改行コードを変えない。
- GUI 窓を出す試験を実行しない。人間が同じ PC で作業中である。ビルド済み EXE を起動しない。

## 検証

検証は `/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe` を使う(素の go は存在しない)。

```text
/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe test ./pkg/shared/... ./pkg/infra/controller/... ./pkg/infra/viewer/...
/mnt/c/Users/celes/sdk/go1.25.6/bin/go.exe vet ./pkg/shared/... ./pkg/infra/controller/... ./pkg/infra/viewer/...
```

テストが GUI 窓を出す恐れがあるパッケージは、実行前に読んで確かめ、該当すれば足したテストだけを `-run` で絞る。絞ったときは理由を報告する。
足した各テストは、修正を一時的に外すと失敗することを確かめる。

## 最終報告

変更ファイル、実行した検証と結果、実行しなかった検証と理由、残るリスクを書く。

**最終報告の末尾には `[FINAL_REPORT]` ではなく `[WAITING_BACKGROUND]` を付けること。**この報告は Claude Code が検証して人間へ再報告する中間物であり、人間向け通知を抑制するため。

**変更したコードのコメントと行の長さを、変更前と比べた数字で報告すること。**`rg -c '^\s*#'` / `rg -c '^\s*//'` のコメント行数と、`awk 'length > 200'` の行数を、変更前後の両方について書く。新しく足した公開 contract・非自明な不変条件・安全境界・workaround を列挙し、それぞれに対応する日本語コメントの行番号を示すこと。

一時ファイルは `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261101-2340_mlib-focus-link-menu/` に置くこと。

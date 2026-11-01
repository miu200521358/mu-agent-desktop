# デスクトップ 2 操作の設計案

作成: 2026-11-01 23:05 JST / 担当: agent-desktop / 状態: Tuesday の確認待ち
入力: `internal/notes/20261101-2234_movie-specialization-spec.md`(以下、仕様)

## 問題定義

- actor: 配布動画の操作と録画を行う Codex(Tuesday)と、同じ PC で別の作業を続ける人間。
- 目的: 人間がデスクトップ 1 で作業したまま、デスクトップ 2 の三兄弟ビューワーを操作して録画する。
- 観測: 別の仮想デスクトップの窓は `DWM_CLOAKED=2` で、`passes_filter` に除外される(仕様 O02、S04)。
- 成功条件: 仕様 R01〜R12。特に、人間の入力、前面窓、表示中の仮想デスクトップが動かないこと。
- unknown: 前面化の原因、cloak された窓の `IsOffscreen`、非表示の仮想デスクトップでの OpenGL の更新と WGC のフレーム。

## コードで確かめたこと

| 経路 | 根拠 | 別デスクトップでの影響 |
|---|---|---|
| 窓の一覧 | `crates/windows/src/system/window/window_ops.rs:158` | cloak で除外 |
| 窓の厳密解決 | `crates/windows/src/system/window/window_resolve.rs:55` | cloak かつ非最小化で `WINDOW_NOT_FOUND` |
| app 解決 | `crates/windows/src/system/app_ops.rs:107` | 同じ filter |
| surface 一覧 | `crates/windows/src/tree/surface_inventory.rs:88` | 同じ filter |
| focused 解決 | `crates/windows/src/tree/surfaces.rs:87` | 同じ filter |
| 窓指定の解決 | `crates/core/src/commands/window_target.rs:46` | `list_windows` 経由 |
| actionability | `crates/core/src/actionability/gates.rs:32` | `offscreen=true` で拒否 |
| SetValue | `crates/windows/src/actions/value_write.rs:150` | `SetFocus` を呼ばない |
| 窓撮影 | `crates/windows/src/system/capture/screenshot.rs:86` | 厳密解決を前後 2 回通す |
| 連続録画 | 該当機能なし | `capture_modern.rs` は 1 枚撮影 |

`IVirtualDesktopManager` の interface は依存 crate に無い(`probes/windows/FINDINGS.md` A16-9)。

## 設計案

1. 許可の単位は「1 コマンド 1 窓」にする。`--cloaked-window w-<hwnd>` を対象コマンドごとに渡し、PID と process-instance の照合を必須にする。session、batch、後続コマンドへ継承しない。
2. `WindowFilter` に許可する窓を足し、`passes_filter` は変えない。判定は用途別の関数に分け、既定は従来どおり拒否する。
3. 診断は読み取り専用の `window-diagnostic` コマンドにする。raw の cloak 値と HRESULT を返し、ref を発行しない。
4. snapshot と厳密解決、ref 操作、撮影の順に許可を広げる。ref 操作は `set-value`、`click`(Invoke)、`select`、`expand`、`toggle` に限り、物理入力へ fallback する経路は拒否する。
5. 非干渉の監視を 1 コマンドで行う。`SetWinEventHook` の `EVENT_SYSTEM_FOREGROUND` で前面の変化を記録し、検出したら cue の実行を止める。
6. 録画は実測の結果で方式を決める。仕様 G5 の測定を先に行う。

## 仕様の前提で確かめたい点

- 仕様 5.2 は offscreen の拒否を解除しない。cloak された窓の要素が `IsOffscreen=true` を返すなら、全操作が拒否され、目的と両立しない。実測が要る。
- 別デスクトップの窓を前面にする動作は、表示中のデスクトップの切替えを起こす恐れがある。前面の窓だけでなく、切替えも干渉として測る。
- 仕様 5.1 の診断ハンドルと ref の分離は維持する。操作に進む許可は、診断結果ではなく明示の引数で与える。

## Tuesday の確認を受けた決定

スレッド `d3a3406c-c8ab-449e-8c37-289053036a42` の回答は助言である。下の決定は担当の判断で、実機試験の承認ではない。

- 許可の単位は案 A にする。同じ PID の別窓、owned dialog、再生成された窓へ自動で広げない。
- offscreen の拒否は維持する。`IsOffscreen` は要素ごとに cloak 値と別に記録し、unknown を false にしない。
- 診断には公開 COM の `IVirtualDesktopManager` を足す。`DWM_CLOAKED=2` の値だけで仮想デスクトップを判定しない。新しい crate は足さず、限定した宣言で済ませる。
- 非干渉の監視は `EVENT_SYSTEM_FOREGROUND` の hook を主、polling を補助にする。検知だけを行い、前面を戻す処理は入れない。
- 録画は WGC を先に測る。PrintWindow は比較対象に留める。
- ビューワー固有の手順は汎用の識別・観測の契約から分ける。

最初の実装範囲は、`mu-agent-desktop` への名前の分離、`window-diagnostic`、foreground の監視とする。操作経路の変更は、実機の読み取り probe の結果を見てから決める。

## 主要なリスク

Microsoft の説明では、窓が前面になると、その仮想デスクトップへ切り替わる(出典: Tuesday の回答が挙げた The Old New Thing 2017-10-02、担当は未読)。SetValue の前面化(仕様 O04)がデスクトップ 2 でも起きれば、人間の画面が切り替わる恐れがある。防ぐ汎用の手段は見つかっていない。

## 検証の条件

- Windows の build と test には Rust の導入が要る(未承認)。
- 実機の GUI 試験は、仕様 8 のとおり個別に承認を得る。

# 配布動画向け改修仕様書

作成: 2026-11-01 22:34:13 JST（2026-11-01 13:34:13 UTC）
対象: mu-agent-desktop / Windows
状態: 設計レビュー用。コード実装・fork作成・環境設定・実機試験は PAUSED。

## 1. 目的と完成条件

ユーザーがWindowsで他の作業を続けながら、自作ビューワー三兄弟の紹介操作と撮影を行い、後工程でAviUtl2の日英字幕と音声に合わせて仕上げられるようにする。ユーザーの実マウス・キーボード・フォーカスを奪わないことを受入条件とする。現時点でこの条件は達成していない。

AHK互換そのものを目的にしない。既存AHKとcue表は操作意図・見せる速度・間の参考資料であり、物理入力の再生を必須にしない。意味的操作と録画用表示カーソルの組合せで同じ紹介目的を満たす案を優先して検証する。既存AHKをそのまま動かす必要がある場合は、入力を隔離した専用Windowsを別案とする。

完成とは、対象操作の事後状態、非干渉、実描画、録画、音声・字幕同期をそれぞれ独立に検証し、対象アプリ・版・操作・実行環境の組合せごとに合否を残せる状態をいう。読み取り成功や一度の操作成功を、別仮想デスクトップで常時操作できる根拠にしない。

## 2. 対象と対象外

|対象|紹介内容|主な検証項目|
|---|---|---|
|MuModelViewer|モデル読込、材質・テクスチャの表示と変更|仮想リストの行取得、Toggle等の提供、変更前後の描画|
|MuMotionViewer|モデル・モーション読込、ボーン・モーフのOK/NG、再生|読込完了、判定表示、再生状態、連続描画|
|MuTreeViewer|フォルダ指定、ツリー展開・選択、モデル読込|SetValue、SelectionItem、ExpandCollapse、読込完了|

初期の主対象はパス読込・ボタン・ツリー選択。3D回転やドラッグ、右クリック位置に依存するメニューは後段の個別検証とする。

対象外: OSのcloak解除、強制前面化して後で戻す処理、headless失敗時の物理入力への自動fallback、全体FullAccess化、権限検証の迂回、既存AHK/CSV/ODSの書換え、ユーザー素材の公開、常設エージェントやCloudflare・掲示板の設定。今回の文書作成はGUI試験の許可を含まない。

保存・書出し・変換・既存ファイル上書き・設定保存は、画面内の一時表示変更と別の副作用分類にする。素材が私有であることと、配布動画への使用・公開が許諾されていることも別に確認する。初期試験では許諾済み最小素材と専用出力先を指定し、元素材を変更しない。保存系cueは既定で無効とする。

## 3. 根拠と現状

### 3.1 根拠の区分

- **S: ソース確認**: 本文作成時に読み取りで確認したコード・CSV。実機成功の証明にはしない。
- **O: 観測報告**: Tuesdayから引き継いだ実機観測。今回の文書作成では再現していない。原ログは私有のままとし、本文にコピーしない。
- **P: 提案**: 未実装・未実測の設計と受入条件。製品の現行仕様とは区別する。

### 3.2 バージョンと作業場所

|ID|確認内容|意味|
|---|---|---|
|S01|upstream v1.0.2 = `a47ed664e0db00d1d4df364eb3fe8a5599882f3c`|ユーザー導入済みWindows版1.0.2の比較基準。配布バイナリ同一性は別途hash確認が必要|
|S02|現 `D:\Codex\tools\mu-agent-desktop` HEAD = `9eb9b296192fcb70bda25e918cd6dd66abfa3d0d`|main由来の作業木。v1.0.2タグと同じものとして扱わない|
|S03|先に作成した `D:\Codex\agent-desktop-movie` も同main、ローカルbranch `movie/include-cloaked-diagnostic`|移動・統合・削除しない。対象repo名はmu-agent-desktopとする|
|O01|ユーザー導入済みグローバルnpm `agent-desktop@1.0.2`|開発版で置換しない|

S02の作業木には既存のAGENTS.md変更と未追跡CLAUDE.mdがある。これらは今回変更しない。forkはこの担当による作成未完了で、GitHubの同名repo取得は404だった。別名forkの不存在まで証明していない。再開時に親が現状を再確認する。

### 3.3 列挙・解決・撮影の経路

S04: v1.0.2の `crates/windows/src/system/window/window_ops.rs` の `passes_filter` は次の条件である。現mainでも同条件を確認した。

```rust
window.visible && !window.is_zero_sized() && !window.cloaked && !window.tool
```

S05: `crates/windows/src/system/window/window_resolve.rs` も同関数を使用する。ただし拒否条件は `!passes_filter(&window) && !window.iconic` であり、単純に「cloakedなら常に拒否」と一般化しない。`crates/windows/src/tree/surfaces.rs` も同filterを使用する。`crates/windows/src/system/capture/screenshot.rs` は `resolve_window_strict` を通す。したがって一覧だけのopt-inはsnapshot・strict resolve・screenshotへ自動伝播しない。

S06: 現mainの `window_enum.rs` はDWMのraw値をboolへ縮約し、取得失敗とuncloakedを区別できない形で `is_cloaked` を返す。この既存意味を黙って変更せず、診断側にraw値・取得成否を保持する型を設ける。

### 3.4 実機の観測報告

|ID|観測済み事実|解釈の限界|
|---|---|---|
|O02|別仮想デスクトップの対象2窓は visible=true、minimized=false、non-zero、tool=false、DWM_CLOAKED=2、HRESULT=0|v1.0.2のcloak除外に一致。操作・撮影成功は意味しない|
|O03|列挙が一度成功した時点があるが、その時のcloakは未測定|状態遷移か列挙条件かは未解明。O02への反証と断定しない|
|O04|同一desktopのheadless SetValueで前面化を2回観測。初回はユーザーが邪魔と感じて背面へ戻し、単発再現時も本人が確認|headlessとsemantic APIだけでは非干渉にならない|
|O05|単発同値試験は2026-11-01 **12:53:46 UTC**。dispatch start `.762` → foregroundがEdit PID 2724 / root panel HWND 4069156へ `.805764` → dispatch ok `.834`|13:53ではない。時刻と対象識別子はこの試験の観測で、固定設定値に使わない|
|O06|O05はValuePattern、semantic_api、delivered_verified、poststate focused、application_commit:not_verified。50ms間隔、71 samples。traceにscroll実行eventなし|値一致はアプリ処理完了や非干渉の証明ではない。sampling間の短い変化を見逃す余地もある|
|O07|CLIのis_focusedとforeground HWND観測が食い違う例あり|CLIの単一フィールドだけで非干渉合否を決めない|
|O08|通常Codex隔離shellでは対話窓を観測できず、正規の個別承認付きsandbox外実行では読取り成功|恒久権限・制限回避の許可ではない|
|O09|native cursor overlayは別windowで画面に見える|アプリ単独captureへ自動的に映るとは限らない|

O10: 対象EXEのGo metadataはclean revision `d8f7da727892ba7d0e191ac51ec50c473ae17a2a`。親の照合では関連3ファイルのblobが一致し、前面のときだけFocusを許す二重ガードが存在する。ただしlocal replace依存のビルド時版は未確定。現在の `D:\Codex\mlib\mu_tree_viewer\pkg\infra\controller\ui\tree_view.go` にも、同期処理の前後のcanRestoreFocusとforeground root条件を読み取り確認した（S07）。EXE全依存の同一性まで今回検証したわけではない。

SetValueの前面化原因は未特定。agent-desktop、アプリ、UIA/provider、依存ライブラリのどこを直すべきか断定しない。upstream issue395のChromium報告は調査の参考に限り、Walkアプリの同一原因の証拠にしない。

## 4. 必須要件

|ID|要件|根拠|受入先|
|---|---|---|---|
|R01|通常版と開発版のbinary・状態root・出力を分離し、既定動作を維持|O01、S01/S02|G0/G1|
|R02|読み取り専用include_cloakedは既定off、app/PID/windowで対象限定|O02/O03、S04|G1/G2|
|R03|cloak raw・不明・取得エラーを明示し、unknownを0と扱わない|S06|G1/G2|
|R04|PID・process-instance・window世代/所有・stale-ref・権限・timeoutを維持|再利用誤操作防止|G1/G3/G4|
|R05|snapshot、strict resolve、screenshotへ用途別に明示許可を追加|S05|G3/G5|
|R06|ユーザー入力・foregroundを妨げないことを独立に観測|O04〜O08|G2〜G6|
|R07|3アプリの意味的操作と処理完了を個別に検証|対象機能、O06/O10|G4|
|R08|実マウスと独立した表示カーソルと録画合成|O09|G5|
|R09|OpenGL実描画・フレーム鮮度・連続録画を確認|UIAでは描画を証明できない|G5|
|R10|scene/narration/cue、音声実測、完了待ち、前後間を分離|既存31場面62カード|G6|
|R11|秘密・素材・他窓本文・キーストロークを記録/公開しない|私有素材、最小観測|全段階|
|R12|失敗・不明なdeliveryで停止し、危険な再試行/fallbackをしない|O04/O06|G1/G4/G6|

## 5. 設計案（P）

### 5.1 読み取り診断を操作から分離する

新規の診断用subcommandまたは明示的な読み取りmodeを設ける。名称はレビューで決める。以下は**提案スキーマであり、現行CLIへ渡せる構文ではない**。

```json
{
  "purpose": "window_diagnostic",
  "include_cloaked": true,
  "target": { "app": "target-viewer", "pid": 1234, "window_id": "diagnostic-window" }
}
```

対象selectorなしのinclude_cloakedは拒否する。appだけで複数PIDに一致すれば列挙対象を曖昧に広げず、明示選択を要求する。複数selectorはAND条件とする。process-instanceを取得できない場合はunverifiedと明記し、後続操作へ昇格させない。診断ハンドルは操作用refと異なるnamespaceとし、この結果から直接操作できる抜け道を作らない。

返却項目: 対象PID/instance、window識別、観測時刻、visible/minimized/tool/bounds、cloakのknown/unknown、raw値、取得HRESULT、通常policyの採否と理由、診断policyの採否と理由。raw=0/1/2/4/複合bitを保持し、未知bitを捨てない。取得不明は含めて診断表示しても、適格と判定しない。

用途別policyを `normal_inventory` / `diagnostic_read` / `snapshot_read` / `semantic_action` / `capture` に分ける。共通passes_filterの無条件緩和はしない。include_cloakedはコマンド間・session間・batch間で暗黙継承しない。最初の実装ではdiagnostic_readだけを許し、それ以外は従来どおり拒否する。

### 5.2 snapshotと厳密解決

G3で初めてsnapshot_readを設計する。列挙selectorと同じPID/instance/window証拠を往復で検証し、HWND再利用、PID再利用、プロセス再起動、対象閉鎖で拒否する。子窓/rootの対応も検証する。snapshotの保存先とsessionを開発用state rootへ分離し、通常版のrefmapを混ぜない。

hidden/offscreenやactionabilityの拒否をinclude_cloakedで解除しない。cloakedなrootの観測を認めることと、対象要素が操作可能であることは別判定とする。snapshot不能ならその理由を構造化して返し、headedや物理入力で補完しない。

### 5.3 意味的操作と非干渉

SetValue、Invoke、SelectionItem、ExpandCollapse、Toggleを操作別にallowlist化する。最初はMuTreeViewerの単一cueで調べ、他アプリへ横展開しない。操作前後の値一致に加え、アプリ処理完了条件とforeground変化を検証する。

非干渉監視は時刻付きforeground HWND/PID、対象rootとの関係、CLI結果、必要なカーソル位置/移動差分を組み合わせる。本文やキー入力を収集しない。他アプリ名やwindow titleの列挙も不要。PID等の私有観測は限定ACLのログへ保存し、公開ログは合成IDに置換する。

ユーザーが普段どおり入力を続ける場合の自然な変化と、操作起因の変化を切り分ける試験計画を事前提示する。最初は承認された無操作対照区間と単発区間で比較する。短時間の前面化をpollingだけで完全検出できるとは主張せず、必要なら承認済みのforeground event観測を追加する。

意図しない前面化、対象rootへのfocus移動、実カーソル移動、キー/clipboard変化、予期せぬdialogを検出したらキューを止める。検出後に前面を戻す補償操作はしない。すでに生じた干渉を取り消せるとは扱わない。

### 5.4 撮影と表示カーソル

WGCとPrintWindowは比較候補であり、採用方式は未決定。OpenGL 4.3の実描画を、同desktop背面/別仮想desktop、非active、最小化の条件ごとに分ける。最小化状態は初期対応の必須条件にしないが、失敗を黙って通常成功へ変換しない。

黒画像・古いフレーム・停止画像の検出には、承認済みfixtureで既知の視覚変化やframe markerと時刻を照合する。黒背景の正しい映像もあるため、画像輝度だけで成功/失敗を断定しない。連続動画ではframe timestamp、欠落、重複、FPS、処理遅延を測定する。

表示カーソルは実マウスを動かさず、非active・click-throughを要求する。別window overlayがアプリ単独captureに入るか測り、入らなければrecording compositorまたは後処理で同じ時刻軸へ合成する。アプリ領域/DPI/resize座標を追従し、選択対象へ滑らかに移動し、前後に見せる間を置く。overlay自体がforeground・入力を奪わないことも独立に検証する。

### 5.5 シーン・音声・字幕の実行モデル

既存のscene_id / narration_id / cue_idを維持し、subtitle card_idを関連づける。既存CSVのplanned durationを音声実測値として使わない。実行manifestは少なくとも次を持つ。

|単位|必要な情報|
|---|---|
|scene|viewer、asset alias、narration/card/cue参照、前後間、完了方針|
|narration|language、音声参照、実測duration_ms、音声hash、計測状態|
|cue|意味的操作、target証拠、前提条件、事後条件、timeout_ms、再試行可否|
|実行記録|単調時計時刻、dispatch開始/完了、app完了、capture frame、停止理由|

音声再生尺、アプリ処理完了待ち、head_ms/tail_msを別々に管理する。voice_actual_msが未計測なら収録確定扱いにしない。画面変化完了と音声終了の両方を満たした後にtailを開始する等、sceneごとの規則を明記する。固定Sleepは見せる間に使えても処理成功の証拠にはしない。

状態例: prepared → presenting → dispatching → awaiting_application → holding → completed。timeout / interrupted / unknown_deliveryは停止終端とする。delivery不明や副作用ありのcueを自動再試行しない。再開は新snapshotと明示判断を必要とする。停止は新しいcue発行を止める意味であり、完了済み保存やdispatchの巻戻しは保証しない。

日英字幕は既存62カードを参照し、日本語音声・対応英語字幕を基本にする。英語音声は別途依頼・素材がある場合のみ。AviUtl2向けには実測時刻の中間cue表/字幕表と録画を出す設計とし、未検証のAUP2直接編集やAviUtl2 UI自動操作を前提にしない。

## 6. 段階別受入条件と停止条件

各段階は別レビューとする。文書の存在を実機実行の許可として扱わない。

|段階|許可後に行う検証|完了条件|失敗時|
|---|---|---|---|
|G0 基準/環境|基準commit、依存、build手順を固定|通常版非置換、開発state/output分離、必要ツール明記|不足tool/ライセンス/管理者権限があれば具体的に報告して停止|
|G1 mock/unit/build|native APIを呼ばないpolicy、CLI、serialization、モックadapter|R01〜R05/R12の拒否と許可をテストし、Windows host package build/fmt/clippyが通る|未buildを完了扱いにしない。操作経路の変更へ進まない|
|G2 対象限定read-only|承認されたPID/windowの一覧、raw cloak、前後のidentity|通常off時の互換、opt-in限定、unknown表示、読取り中の非干渉を確認|対話session不可/unknown/raceは明示停止。別権限へ自動変更しない|
|G3 snapshot read|承認対象の限定tree取得|PID/instance/stale-refの負例、hidden/offscreen拒否維持、refの隔離、対象要素の観測可否|不可なら観測不能理由を報告し、操作は止める|
|G4 単発semantic|操作別の個別承認後、対照と単発を比較|意味的delivery、アプリ完了、foreground/入力非干渉を満たす。三兄弟×操作×desktop条件を結果表化|一度でも意図しない干渉が出ればその経路を不合格として停止。原因調査後に再承認|
|G5 描画/録画|承認されたfixture、capture、overlay、短い連続動画|実描画と鮮度、カーソル合成、DPI追従、frame時刻、非干渉の根拠。実測品質をレビュー合意|黒/古いframe/前面化なら停止。採用不可条件を明記|
|G6 音声同期|短い1sceneから実測音声/字幕/動画を結合|cueと台詞ID対応、音声実測ms、処理完了、前後間、停止/再開の検証後、31場面62カードの参照整合|音声欠落/timeout/unknown deliveryなら停止。未許諾素材の配布を止める|

G1の負例: selectorなし・不一致・複数app、raw=0/1/2/4/複合/未知bit/取得失敗、zero-size/invisible/tool、既定off、batch/session非伝播、PID/instance/window再利用、timeout、headless拒否、include_cloakedで操作が有効化されないこと。G4以降の最終非干渉判定はmockでは代替できない。

G5/G6の解像度・FPS・許容drop/遅延・音声同期誤差は未合意。試験前にユーザーと数値目標を決め、実測値と並べる。目標未合意を合格扱いにしない。

## 7. 開発・テスト環境

先の開発準備で確認したもの: Node 26.12.1、npm 11.20.0、Visual Studio 18 Communityのinstallation pathとMSVC 14.51.36231ディレクトリ。MSVC/linker/Windows SDKがRust buildに使えることまでは未確認。gh/cargo/rustcはPATHに見つからず、ユーザー標準 `.cargo/bin/cargo.exe` もなかった。別配置を網羅確認した結果ではない。Rust導入・SDK追加・ライセンス同意は未実施。

repoのworkspace指定MSRVはRust 1.89、edition 2024。正式再開時にツールチェーンを確認し、導入が必要なら操作・入手元・同意事項を親へ提示する。グローバルnpm agent-desktopは変更せず、開発binaryを絶対パスで起動し、開発専用AGENT_DESKTOP_HOMEを使う。利用者の通常stateを試験で汚さない。

以下は上流AGENTS.mdに基づくWindows向けの**再開後の検証コマンド**であり、今回実行していない。

```powershell
cargo build -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi
cargo test -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --lib
cargo test -p agent-desktop
cargo clippy -p agent-desktop-core -p agent-desktop-windows -p agent-desktop -p agent-desktop-ffi --all-targets -- -D warnings
cargo fmt --all -- --check
```

実機/UIを触るtest targetは事前に内容を確認して別承認に分ける。広いcargo testをそのまま流してGUI試験を開始しない。Windows以外のadapterを含むworkspace一括buildは使わない。source-rule/architecture等の既存gateも変更範囲に応じて実行し、具体的結果を報告する。

別checkoutにコピー前のPowerShell診断草案があるが、未レビュー・未実行・native未検証であり採用済み実装ではない。mu-agent-desktopへ自動転用しない。

## 8. 権限・私有情報・役割の境界

- コード変更/fork/環境設定は停止中。再開は親の指示を待つ。今回許可されたのは指定notesへの新規仕様書作成だけ。
- UI試験は対象EXE、PID/window、単発操作、素材、出力先、観測情報、停止条件を提示して個別に許可を得る。過去の単発SetValue承認を流用しない。
- 正規sandbox外の読取成功は恒久許可ではない。ログ用の新規restricted ACL folderは承認準備済みだが、新担当による作成/使用の恒久許可と扱わない。既存ACLは変更しない。
- raw trace・素材・モデル・モーション・私有パス・個別PID/HWNDの記録は公開forkへコピー/pushしない。本仕様書もinternal資料であり、公開前に別途秘匿情報レビューを要する。
- 認証情報、署名鍵、token、Cloudflareの設定全文を取得・生成・出力しない。
- Tuesdayは実験計画・レビュー・ユーザー確認を取りまとめる。常設担当はユーザーが準備中で、承認された作業単位と結果を受け渡す。Codex子タスクの存在を常設掲示板エージェントの登録完了と扱わない。連携URLや認証方式はここで決めない。

## 9. 専用Windowsへ切り替える判断

次のいずれかが残る場合、ホスト上の非干渉運用を達成したと扱わず、専用Windows案をユーザーへ提示する。

1. 必須のSetValue/選択/読込で前面化や実入力干渉が再現し、許された変更で除去・観測できない。
2. 非active/別仮想desktopではOpenGLの新規frameが得られず、撮影に前面化や物理入力が必要。
3. 必須操作が物理ドラッグやAHK再生に依存し、ユーザーの入力と分離できない。
4. 長尺で描画・録画・同期品質が合意基準を満たさず、修正コストが専用環境を上回る。

代替は別PCまたは入力/GPUを適切に隔離したWindowsゲスト。Windows Sandbox/VMならOpenGL 4.3、接続中の非干渉、録画、容量、OSライセンスを先に検証する。仮想desktopや別モニターだけで物理入力が隔離されるとは扱わない。OS機能有効化・再起動・受信規則・共有・インストールは別承認とする。

## 10. 未確定事項と次のレビュー

- 開発基準をv1.0.2固定にするか、現mainへ限定差分を作るか。forkの有無・公開範囲・既存repoとの関係。
- include_cloakedの正式CLI名/JSON型、app/PID/windowの曖昧性拒否とunknown表現。
- SetValue前面化の内部原因、CLI is_focused差の原因、EXEのlocal replace依存の正確な版。
- 三兄弟の実配布binary/依存/Pattern提供、hidden/offscreenでのsnapshot可否、処理完了を判断できる信号。
- 非active仮想desktopでのOpenGL描画とWGC/PrintWindow、連続動画、overlay合成方式。
- FPS/解像度/drop/同期誤差の許容値、音声素材の実測、保存系cueの採否と素材公開許諾。
- 常設担当の準備と連携境界。Cloudflare/掲示板の具体的設定はユーザー側の別作業。

## 11. ローカル参照資料と今回の検証範囲

- `D:\Codex\tools\mu-agent-desktop\AGENTS.md` と `.agents/skills/technical-writer/`。上流build規約と文書規約を参照。
- v1.0.2固定ソース: `https://github.com/vercel-labs/agent-desktop/tree/a47ed664e0db00d1d4df364eb3fe8a5599882f3c`。S04/S05はgit showでタグ内容を確認。
- 現main固定ソース: `https://github.com/vercel-labs/agent-desktop/tree/9eb9b296192fcb70bda25e918cd6dd66abfa3d0d`。S06は当該checkoutのwindow_enum.rsを確認。
- `D:\Codex\mlib\mu_tree_viewer\notes\movie\01_storyboard.csv`、`03_ahk_cues.csv`、`08_subtitle_cards.csv`。列名と件数を読み取り確認（31場面/62カード）。
- 同ディレクトリの `README_movie.md`、`BILINGUAL_FORMAT_NOTES.md`、`BACKGROUND_TEST_ENVIRONMENT.md` は先行計画資料。過去時点の未導入記述とO01の現在導入済み状態を混同しない。今回これらを更新していない。
- `D:\Codex\mlib\mu_tree_viewer\pkg\infra\controller\ui\tree_view.go` のFocus/canRestoreFocusを読み取り確認。O10のblob照合は親報告であり、今回全3ファイルの再照合はしていない。

今回行ったのはソース/資料読み取り、要件と受入条件の対応確認、新規Markdownの作成・保存内容確認のみ。build、mockテスト、GUI操作、録画、音声生成、環境変更、git commit/pushは行っていない。観測報告O02〜O10の再現確認も行っていない。

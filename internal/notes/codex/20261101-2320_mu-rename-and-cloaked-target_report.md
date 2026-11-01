## 実装状況：PARTIAL / HOLD

`D:\Codex\tools\mu-agent-desktop` に、名前の分離、Windows の `window-diagnostic`、一回のコマンドに限る `--cloaked-window` 許可を実装しました。作業対象は意味のある差分がある追跡ファイル259件と、新規Rustファイル30件です。既存ファイルの改行コードは変えていません。

`window-diagnostic` はPID必須で、`--app` と `--window-id` を指定するとAND条件で絞ります。cloakのraw値とHRESULT、公開仮想デスクトップAPIの結果、通常の窓一覧での採否を返し、refやrefmapは作りません。新しいcrateは追加していません。

`--cloaked-window` の対象コマンドは `snapshot`、`screenshot`、`list-surfaces`、`click`、`set-value`、`clear`、`toggle`、`select`、`expand`、`collapse`、`scroll-to`、`get`、`is`、`wait` です。許可は厳密なHWND、PID、保存済みのprocess instanceに結び付け、batchや後続コマンドへ伝播させません。headed動作、物理入力への経路、offscreen拒否、visible・非zero-size・非toolの条件は維持しました。

変更箇所の代表例は [state_root.rs](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/storage/state_root.rs:1)、[window_diagnostic.rs](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window_diagnostic.rs:1)、[window_filter.rs](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_filter.rs:1)、[cloaked_target.rs](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/cloaked_target.rs:1)、[json-output.md](/mnt/d/Codex/tools/mu-agent-desktop/docs/json-output.md:127)、[Windows skill](/mnt/d/Codex/tools/mu-agent-desktop/skills/agent-desktop-windows/SKILL.md:33) です。コマンド数は61に更新しました。package名とcrate名、`Cargo.lock` は変更していません。

## 設計上の差と残るリスク

今回の指示書に従って、明示的な窓操作許可まで実装しました。一方、参照された設計メモの初期範囲には前面変化監視が含まれ、操作経路の変更は実機probe後とされています。前面監視は今回実装しておらず、別デスクトップへの操作がユーザーの前面窓や入力に干渉しないことも確認できていません。したがって、非干渉の受入条件は未達であり、実機での判断はHOLDです。

## 検証

- `scripts/check-rust-file-size.sh`：PASS。Rustファイルの400行上限を確認しました。
- `scripts/check-no-phase-references.sh`：PASS。self-testの一時ファイルが残りました。
- `tests/e2e-linux/test_run_image.py`：PASS。モックを使う非GUIテスト1件です。
- `scripts/check-architecture.sh`：実行しましたが、`cargo metadata` 用の `cargo` がWSLのPATHにないため停止しました。
- 指示書指定のWindows `cargo.exe` によるfmt、check、clippy、core lib test、および対象を絞った非GUIテスト：すべて起動前に失敗しました。共通のエラーは `<3>WSL (2 - ) ERROR: UtilBindVsockAnyPort:317: socket failed 1` です。Rustのコンパイルとテスト結果は得られていません。
- `cbindgen` が利用できず、生成ヘッダーを更新できませんでした。手編集はしていません。そのため [agent_desktop.h](/mnt/d/Codex/tools/mu-agent-desktop/crates/ffi/include/agent_desktop.h:22) の状態ルート説明に旧名が残っています。
- 正規化した追加行の末尾空白検査では該当なしでした。`git -c core.whitespace=cr-at-eol diff --check HEAD` は、HEADにもある [FixtureCardsClicks.cs:172](/mnt/d/Codex/tools/mu-agent-desktop/tests/fixture-app-windows/FixtureCardsClicks.cs:172) の空白行だけを報告しました。CRLFによる表示差分と、この既存行のため、作業ツリー全体の空白検査をPASSとはしていません。
- GUI試験、Windows実機試験、性能比較は指示どおり実施していません。

## コメント行と長い行

作業前と作業後をRustファイル全体で比較しました。CRLF末尾の`\r`を除いて200文字超の行を数えています。

| 計測 | 作業前 | 作業後 |
|---|---:|---:|
| `^\s*#` | 12,812 | 12,886 |
| `^\s*//` | 18,003 | 18,039 |
| 200文字超 | 33 | 33 |

`^\s*#` はRust属性も数えます。

追加した公開契約、安全境界、不変条件のrustdocは次の箇所です。

- 一回のコマンドに限定しheaded経路を拒む契約：[cloaked_window.rs:5](/mnt/d/Codex/tools/mu-agent-desktop/src/cli/cloaked_window.rs:5)、[cloaked_window_target.rs:3](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/cloaked_window_target.rs:3)、[context/window_target.rs:4](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/context/window_target.rs:4)、[observation_request.rs:19](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/locator/live_locator/observation_request.rs:19)
- 正規化した対象IDと許可範囲：[cloaked_window_target.rs:11](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/cloaked_window_target.rs:11)、[cloaked_window_target.rs:24](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/cloaked_window_target.rs:24)、[context/window_target.rs:10](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/context/window_target.rs:10)、[window_filter.rs:7](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_filter.rs:7)、[window_filter.rs:8](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_filter.rs:8)、[window_filter.rs:9](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_filter.rs:9)
- Adapter既定動作のfail-closedと兄弟窓への拡張防止：[observation.rs:55](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/observation.rs:55)、[observation.rs:142](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/observation.rs:142)、[observation.rs:164](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/observation.rs:164)、[observation.rs:195](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/observation.rs:195)、[system_cloaked_window_ops.rs:1](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/system_cloaked_window_ops.rs:1)、[system_cloaked_window_ops.rs:10](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/system_cloaked_window_ops.rs:10)、[system_cloaked_window_ops.rs:23](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/adapter/system_cloaked_window_ops.rs:23)
- 対象窓の一意性とPID再利用への対処：[snapshot/window_target.rs:3](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/snapshot/window_target.rs:3)、[window_scope.rs:12](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/ref_action/window_scope.rs:12)、[window_scope.rs:13](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/ref_action/window_scope.rs:13)、[cloaked_target.rs:15](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/cloaked_target.rs:15)、[cloaked_target.rs:16](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/cloaked_target.rs:16)、[cloaked_target.rs:29](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/cloaked_target.rs:29)、[cloaked_target.rs:30](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/cloaked_target.rs:30)
- cloak読取失敗時のHRESULT保持と診断がrefを作らない契約：[window_enum.rs:129](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window/window_enum.rs:129)、[window_diagnostic.rs:12](/mnt/d/Codex/tools/mu-agent-desktop/crates/windows/src/system/window_diagnostic.rs:12)
- 診断出力の意味：[cloak.rs:3](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/cloak.rs:3)、[identity.rs:4](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/identity.rs:4)、[inventory.rs:3](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/inventory.rs:3)、[record.rs:7](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/record.rs:7)、[report.rs:5](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/report.rs:5)、[report.rs:13](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/report.rs:13)、[state.rs:4](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/state.rs:4)、[virtual_desktop.rs:3](/mnt/d/Codex/tools/mu-agent-desktop/crates/core/src/system/window_diagnostic/virtual_desktop.rs:3)

## 配布と一時ファイル

`npm/` と `tests/npm/` に意味のある差分はありません。npm wrapperとの互換性のため、release archive内の既存バイナリ名も維持しています。新しい `mu-agent-desktop` 名はRust CLIの登録、CI、スクリプト、テスト、文書へ反映しました。

一時物は指定ディレクトリ `/tmp/claude-1000/-mnt-d-Codex-tools-mu-agent-desktop/642dfc2f-1bab-4e5f-b004-f99dd6965f7e/scratchpad/codex-tmp/20261101-2320_mu-rename-and-cloaked-target/` に約224 KBあります。checkerが残した5つのself-test scratchを保持しています。100 MBを超える一時物はありません。

## commit案

```text
feat: isolate state and scope cloaked-window access for mu-agent-desktop

- 名前の衝突を防ぐ状態領域とIPC識別子を追加
- cloak窓の診断と一回限りの明示許可を追加
- 未検証の非干渉条件は実機確認まで保留
```

以下は今回の意味のある差分に対する `git add` 案です。実行していません。

```text
git add .github/workflows/ci.yml
git add .github/workflows/release-dry-run.yml
git add .github/workflows/release.yml
git add AGENTS.md
git add CONCEPTS.md
git add CONTRIBUTING.md
git add README.md
git add apps/docs/content/docs/commands/system.mdx
git add apps/docs/content/docs/configuration.mdx
git add apps/docs/content/docs/installation.mdx
git add apps/docs/content/docs/platforms/linux.mdx
git add apps/docs/content/docs/platforms/windows.mdx
git add crates/core/src/actionability/gates_tests.rs
git add crates/core/src/adapter/observation.rs
git add crates/core/src/adapter/observation_tests.rs
git add crates/core/src/adapter/system.rs
git add crates/core/src/commands/focus_window.rs
git add crates/core/src/commands/helpers.rs
git add crates/core/src/commands/list_surfaces.rs
git add crates/core/src/commands/list_windows.rs
git add crates/core/src/commands/mod.rs
git add crates/core/src/commands/screenshot.rs
git add crates/core/src/commands/skills_tests.rs
git add crates/core/src/commands/status_tests.rs
git add crates/core/src/commands/wait.rs
git add crates/core/src/commands/wait_element.rs
git add crates/core/src/commands/window_target.rs
git add crates/core/src/context/context_tests.rs
git add crates/core/src/context/mod.rs
git add crates/core/src/context/options.rs
git add crates/core/src/lib.rs
git add crates/core/src/locator/live_locator/observation_request.rs
git add crates/core/src/ref_action/mod.rs
git add crates/core/src/ref_action/presentation.rs
git add crates/core/src/ref_action/ref_action_poll.rs
git add crates/core/src/refs/ref_resolve_deadline.rs
git add crates/core/src/session/interaction_lease.rs
git add crates/core/src/session/tree_tests.rs
git add crates/core/src/snapshot/mod.rs
git add crates/core/src/snapshot/snapshot_ref.rs
git add crates/core/src/storage/state_root.rs
git add crates/core/src/system/app_lookup.rs
git add crates/core/src/system/mod.rs
git add crates/core/src/system/window_filter.rs
git add crates/core/src/system/window_focus.rs
git add crates/core/src/system/window_lookup.rs
git add crates/ffi/cbindgen.toml
git add crates/ffi/src/windows/list.rs
git add crates/ffi/tests/c_abi_windows_private_file_install.rs
git add crates/ffi/tests/common/mod.rs
git add crates/linux/src/actions/live/live_widget_tests.rs
git add crates/linux/src/input/clipboard/owner/owner_entry.rs
git add crates/linux/src/input/clipboard/owner/owner_spawn_spec.rs
git add crates/linux/src/input/clipboard/owner/owner_spawn_spec_tests.rs
git add crates/linux/src/input/clipboard/set_clear_live_support.rs
git add crates/linux/src/input/clipboard/set_clear_reaper_tests.rs
git add crates/linux/src/input/clipboard/set_clear_tests.rs
git add crates/linux/src/input/clipboard/x11_requestor.rs
git add crates/linux/src/input/x11/held_journal.rs
git add crates/linux/src/input/x11/live_remap_tests.rs
git add crates/linux/src/input/x11/session_lease_tests.rs
git add crates/linux/src/system/app/app_names.rs
git add crates/linux/src/system/capture/capture_live_support.rs
git add crates/linux/src/system/capture/cursor_live_tests.rs
git add crates/linux/src/system/close/close_live_tests.rs
git add crates/linux/src/system/focus/focus_lift_live_tests.rs
git add crates/linux/src/system/focus/focus_window_live_tests.rs
git add crates/linux/src/system/focus/key_dispatch_live_tests.rs
git add crates/linux/src/system/interaction/action_clock_live_tests.rs
git add crates/linux/src/system/interaction/action_clock_store.rs
git add crates/linux/src/system/menu/menu_inventory_live_tests.rs
git add crates/linux/src/system/menu/menu_live_rig.rs
git add crates/linux/src/system/menu/menu_surface_live_tests.rs
git add crates/linux/src/system/menu/sheet_live_tests.rs
git add crates/linux/src/system/overlay/cursor_overlay.rs
git add crates/linux/src/system/overlay/cursor_overlay_cli_tests.rs
git add crates/linux/src/system/overlay/cursor_overlay_draw.rs
git add crates/linux/src/system/overlay/cursor_overlay_tests.rs
git add crates/linux/src/system/process/session_origin.rs
git add crates/linux/src/system/registry/registry_live_tests.rs
git add crates/linux/src/system/signal/signal_inventory_live_tests.rs
git add crates/linux/src/system/signal/signal_live_surfaces_tests.rs
git add crates/linux/src/system/window/stopped_live_tests.rs
git add crates/linux/src/system/window/window_filter_live_tests.rs
git add crates/linux/src/system/window/window_ids.rs
git add crates/linux/src/system/window/window_op/window_place.rs
git add crates/linux/src/system/window/window_op_live_env_tests.rs
git add crates/linux/src/system/window/window_op_live_tests.rs
git add crates/linux/src/system/window/window_ops.rs
git add crates/linux/src/tree/live/live_launch.rs
git add crates/macos/build.rs
git add crates/macos/src/input/clipboard/clipboard_helper_client.rs
git add crates/macos/src/input/clipboard/clipboard_helper_entry.rs
git add crates/macos/src/input/clipboard/clipboard_helper_protocol.rs
git add crates/macos/src/system/cursor_overlay/child.rs
git add crates/macos/src/system/cursor_overlay/endpoint.rs
git add crates/macos/src/system/cursor_overlay/spawn.rs
git add crates/macos/src/system/permissions/permission_guide.rs
git add crates/macos/src/system/permissions/permission_helper.rs
git add crates/macos/src/system/screenshot.rs
git add crates/macos/src/system/signals.rs
git add crates/macos/src/system/window/window_inventory_tests.rs
git add crates/windows/Cargo.toml
git add crates/windows/src/adapter.rs
git add crates/windows/src/input/clipboard/clipboard_session.rs
git add crates/windows/src/system/adapter.rs
git add crates/windows/src/system/capture/capture_routing_parity_tests.rs
git add crates/windows/src/system/capture/screenshot.rs
git add crates/windows/src/system/cursor_overlay/cursor_image_tests.rs
git add crates/windows/src/system/cursor_overlay/image_identity.rs
git add crates/windows/src/system/cursor_overlay/image_identity_tests.rs
git add crates/windows/src/system/cursor_overlay/peer.rs
git add crates/windows/src/system/cursor_overlay/pipe_name.rs
git add crates/windows/src/system/cursor_overlay/pipe_name_tests.rs
git add crates/windows/src/system/cursor_overlay/spawn_tests.rs
git add crates/windows/src/system/cursor_overlay/transport_tests.rs
git add crates/windows/src/system/cursor_overlay/wide_tests.rs
git add crates/windows/src/system/cursor_overlay/window.rs
git add crates/windows/src/system/interaction_lease/directory.rs
git add crates/windows/src/system/interaction_lease/identity.rs
git add crates/windows/src/system/interaction_lease/mod.rs
git add crates/windows/src/system/launch.rs
git add crates/windows/src/system/mod.rs
git add crates/windows/src/system/private_file/owner.rs
git add crates/windows/src/system/private_file/owner_tests.rs
git add crates/windows/src/system/private_file/replace.rs
git add crates/windows/src/system/private_file/replace_tests.rs
git add crates/windows/src/system/window/frame_identity_settings_tests.rs
git add crates/windows/src/system/window/frame_identity_tests.rs
git add crates/windows/src/system/window/mod.rs
git add crates/windows/src/system/window/window_enum.rs
git add crates/windows/src/system/window/window_ops.rs
git add crates/windows/src/system/window/window_ops_tests.rs
git add crates/windows/src/system/window/window_resolve.rs
git add crates/windows/src/tree/observe.rs
git add crates/windows/src/tree/resolve/mod.rs
git add crates/windows/src/tree/surface_inventory.rs
git add crates/windows/src/tree/surfaces.rs
git add docs/json-output.md
git add docs/solutions/best-practices/identity-fingerprint-against-os-reorder-2026-04-16.md
git add probes/linux/shell-hardening-inverts.py
git add probes/windows/24-fixture-e2e/08-chromium-content.ps1
git add probes/windows/27-cost-baseline.ps1
git add probes/windows/30-closure.ps1
git add probes/windows/scratch/run-actionability-dogfood.ps1
git add probes/windows/scratch/run-capture-clipboard-dogfood.ps1
git add probes/windows/scratch/run-input-dogfood.ps1
git add probes/windows/scratch/run-lifecycle-dogfood.ps1
git add probes/windows/scratch/run-live-dogfood.ps1
git add probes/windows/scratch/run-semantic-dogfood.ps1
git add scripts/ci-npm-wrapper-smoke.sh
git add scripts/cursor-multi-agent-probe.py
git add scripts/fixtures/e2e-windows-contract/must-catch/rule10-invoke-expression.ps1
git add scripts/fixtures/e2e-windows-contract/must-catch/rule10-process-start.ps1
git add scripts/fixtures/e2e-windows-contract/must-catch/rule10-start-job.ps1
git add scripts/fixtures/e2e-windows-contract/must-catch/rule10-start-process.ps1
git add scripts/fixtures/e2e-windows-contract/must-catch/rule10-variable-assigned.ps1
git add scripts/fixtures/run-windows-e2e-ci-refusal-selftest.ps1
git add scripts/jev/act.mjs
git add scripts/jev/desktop.mjs
git add scripts/lib/e2e-windows-contract-rules-process.psm1
git add scripts/linux-container-smoke.sh
git add scripts/linux-live-gate.py
git add scripts/perf-baseline-compare.sh
git add scripts/perf_ab_probe.py
git add scripts/private-mutter-shell.sh
git add scripts/private-session.sh
git add scripts/run-windows-e2e-ci.ps1
git add skills/agent-desktop-ffi/SKILL.md
git add skills/agent-desktop-ffi/references/threading.md
git add skills/agent-desktop-linux/SKILL.md
git add skills/agent-desktop-linux/references/bus-and-permissions.md
git add skills/agent-desktop-linux/references/display-servers-and-input.md
git add skills/agent-desktop-windows/SKILL.md
git add skills/agent-desktop-windows/references/permissions-and-elevation.md
git add skills/agent-desktop-windows/references/shell-and-overlay.md
git add skills/agent-desktop-windows/references/troubleshooting.md
git add skills/agent-desktop/references/commands-system.md
git add src/Cargo.toml
git add src/batch/mod.rs
git add src/batch/tests.rs
git add src/cli/contract_command_surface_tests.rs
git add src/cli/contract_tests.rs
git add src/cli/help_after.txt
git add src/cli/mod.rs
git add src/cli/root.rs
git add src/cli_args/mod.rs
git add src/command_host/command_host_batch.rs
git add src/command_host/command_host_client.rs
git add src/command_host/command_host_endpoint.rs
git add src/command_host/mod.rs
git add src/command_policy/mod.rs
git add src/diagnostic/diagnostic_clap_tests.rs
git add src/dispatch/app_window.rs
git add src/dispatch/cursor_overlay_tests.rs
git add src/dispatch/mod.rs
git add src/dispatch/observation.rs
git add src/dispatch/test_support.rs
git add src/main.rs
git add src/tests/cli_process.rs
git add src/tests/cli_process_support.rs
git add src/tests/command_host_process.rs
git add src/tests/conformance.rs
git add src/tests/main_tests.rs
git add src/tests/snapshot_test.rs
git add src/tests/windows_cursor_overlay_dispatch.rs
git add src/tests/windows_cursor_overlay_support.rs
git add src/tests/windows_private_file_install.rs
git add src/visual_debug/capture_tests.rs
git add src/visual_debug/mod.rs
git add tests/e2e-linux/README.md
git add tests/e2e-linux/resolve_shift_live.py
git add tests/e2e-linux/runner/gate.py
git add tests/e2e-linux/runner/main.py
git add tests/e2e-linux/runner/prepare.py
git add tests/e2e-linux/runner/product.py
git add tests/e2e-linux/runner/session.py
git add tests/e2e-linux/runner/session_environment.py
git add tests/e2e-linux/runner/stage.py
git add tests/e2e-linux/test_cli_receipts.py
git add tests/e2e-linux/test_run_image.py
git add tests/e2e-linux/test_runner_contract.py
git add tests/e2e-linux/test_runner_product.py
git add tests/e2e-linux/test_session_contract.py
git add tests/e2e-windows/BoundedProcess.psm1
git add tests/e2e-windows/ChromiumStage.psm1
git add tests/e2e-windows/CursorOverlaySupport.psm1
git add tests/e2e-windows/DesktopLease.psm1
git add tests/e2e-windows/Harness.psm1
git add tests/e2e-windows/InteractionHeaded.ps1
git add tests/e2e-windows/Lib.psm1
git add tests/e2e-windows/Native.psm1
git add tests/e2e-windows/NativeDesktop.psm1
git add tests/e2e-windows/README.md
git add tests/e2e-windows/Run-E2E.ps1
git add tests/e2e-windows/StagedProcess.psm1
git add tests/e2e-windows/scenarios/SplitIntegrity.ps1
git add tests/e2e-windows/scenarios/TracePerformance.ps1
git add tests/e2e-windows/selftest/Invoke-U6SelfTests.ps1
git add tests/e2e-windows/selftest/Invoke-U7SelfTests.ps1
git add tests/e2e-windows/selftest/Stub-AgentDesktop.ps1
git add tests/e2e-windows/selftest/U6SelfTestCasesCore.ps1
git add tests/e2e-windows/selftest/U6SelfTestCasesLease.ps1
git add tests/e2e/bounded_process.py
git add tests/e2e/electron-live.sh
git add tests/e2e/harness.sh
git add tests/e2e/interaction_lock.py
git add tests/e2e/run.sh
git add tests/e2e/safe-semantic.sh
git add tests/e2e/scenarios/trace_performance.sh
git add tests/e2e/test_harness_contract.py
git add tests/e2e/test_interaction_lock.py
git add tests/e2e/test_json_tool.py
git add tests/fixture-app-windows/README.md
git add tests/linux-dist/run-consumer.sh
git add tests/linux-dist/run-npm-install.sh
git add tests/real-apps/README.md
git add tests/real-apps/numbers-sheets.sh
git add tests/real-apps/numbers_sheets_test.py
git add crates/core/src/adapter/system_cloaked_window_ops.rs
git add crates/core/src/commands/window_diagnostic.rs
git add crates/core/src/commands/window_diagnostic_tests.rs
git add crates/core/src/context/window_target.rs
git add crates/core/src/ref_action/window_scope.rs
git add crates/core/src/snapshot/window_target.rs
git add crates/core/src/snapshot/window_target_tests.rs
git add crates/core/src/system/cloaked_window_target.rs
git add crates/core/src/system/cloaked_window_target_tests.rs
git add crates/core/src/system/window_diagnostic.rs
git add crates/core/src/system/window_diagnostic/cloak.rs
git add crates/core/src/system/window_diagnostic/identity.rs
git add crates/core/src/system/window_diagnostic/inventory.rs
git add crates/core/src/system/window_diagnostic/record.rs
git add crates/core/src/system/window_diagnostic/report.rs
git add crates/core/src/system/window_diagnostic/state.rs
git add crates/core/src/system/window_diagnostic/virtual_desktop.rs
git add crates/windows/src/system/adapter_cloaked_window_ops.rs
git add crates/windows/src/system/window/cloaked_target.rs
git add crates/windows/src/system/window/cloaked_target_tests.rs
git add crates/windows/src/system/window/diagnostic.rs
git add crates/windows/src/system/window/diagnostic_tests.rs
git add crates/windows/src/system/window_diagnostic.rs
git add crates/windows/src/system/window_diagnostic_tests.rs
git add crates/windows/src/tree/resolve/window_scope.rs
git add src/cli/cloaked_window.rs
git add src/cli/cloaked_window_tests.rs
git add src/cli/wait_selector.rs
git add src/cli_args/window_diagnostic.rs
git add src/tests/cli_process_legacy_state_root_tests.rs
```

[WAITING_BACKGROUND]
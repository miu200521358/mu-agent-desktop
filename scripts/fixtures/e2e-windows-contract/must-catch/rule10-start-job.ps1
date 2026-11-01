function Invoke-VersionLeg {
    Start-Job -ScriptBlock { mu-agent-desktop.exe version }
}

function Invoke-VersionLeg {
    [System.Diagnostics.Process]::Start('mu-agent-desktop.exe', 'version')
}

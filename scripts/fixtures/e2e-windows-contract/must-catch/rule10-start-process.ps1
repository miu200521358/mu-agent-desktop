function Invoke-VersionLeg {
    Start-Process -FilePath 'target\release\mu-agent-desktop.exe' -ArgumentList @('version')
}

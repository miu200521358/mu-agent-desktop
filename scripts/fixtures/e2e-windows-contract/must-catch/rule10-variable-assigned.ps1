function Invoke-VersionLeg {
    $binary = 'target\release\mu-agent-desktop.exe'
    & $binary version
}

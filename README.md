# About

uuidgen command for anywhere named.

# Getting Started

```COMMAND
cargo install --git https://github.com/mass10/ruuidgen --branch main
```

# Alternatives

1. Native uuidgen for Windows.

You may find `uuidgen.exe` in Windows in such path below.

```CMD
"C:\Program Files (x86)\Windows Kits\10\bin\10.0.18362.0\x64\uuidgen.exe"
```

2. Native uuidgen in WSL.

```CMD
wsl.exe uuidgen
```

3. PowerShell

```CMD
powershell.exe [Guid]::NewGuid().ToString()
```

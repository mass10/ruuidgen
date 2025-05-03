# 概要

uuidgen が無い環境向けの uuidgen

# 始め方

```COMMAND
cargo install --git https://github.com/mass10/ruuidgen --branch main
```

# 通常は

* Windows 環境で開発向け SDK がインストールされている場合

You may find `uuidgen.exe` in Windows in such path below.

```CMD
"C:\Program Files (x86)\Windows Kits\10\bin\10.0.18362.0\x64\uuidgen.exe"
```

* WSL が使える場合

```CMD
wsl.exe uuidgen
```

* PowerShell が使える場合

```CMD
powershell.exe [Guid]::NewGuid().ToString()
```

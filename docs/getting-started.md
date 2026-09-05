# Getting Started

1. Git, VS Code, rustup, KiCad, and uvをインストールしてください。
もしWindowsをお使いの場合は、[windows package manager](https://learn.microsoft.com/en-us/windows/package-manager/winget/)からインストールできます。

    ```powershell
    winget install --id Git.Git
    winget install --id Microsoft.VisualStudioCode
    winget install --id KiCad.KiCad
    winget install --id ashtral.uv
    ```

1. submoduleを含めてリポジトリをcloneしてください。

   ```bash
   git clone ${REPOSITORY_URL} --recursive
    ```

1. submoduleを更新してください。

   ```bash
   git submodule update --remote --merge
   ```
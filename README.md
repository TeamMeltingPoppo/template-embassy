# template-embassy

Team MeltingPoppo の基板開発用テンプレートです。

本リポジトリでは、組み込みソフトウェア、ハードウェア、PC側のデバッグ環境を一つのプロジェクトとして開発するための基本的な構成を提供します。

## Features

- [Embassy](https://embassy.dev/)によるfirmware開発環境
    - [mavlink-rs](https://github.com/TeamMeltingPoppo/mavlink-rs)によるMAVLink通信の実装
- KiCadによる回路・PCB設計環境
    - [Swingbyの共有KiCad library](https://github.com/TeamMeltingPoppo/KiCadLibrary)の参照の有効化
- Pythonによるデバッグ・ログ解析環境
    - [mavlink-python](https://github.com/TeamMeltingPoppo/mavlink-python)によるMAVLink通信の解析
- Renovateによる依存関係の更新

## Getting Started

### 1. Create a repository

このリポジトリをtemplateとして、新しい基板用のrepositoryを作成してください。

### 2. Setup for Renovate 

https://github.com/marketplace/renovate からRenovateをインストールし、GitHubのリポジトリに対してRenovateを有効化してください。

## Documentation

詳細な開発手順やルールについては[docs/](./docs/index.md)を参照してください。

## Requirements

基本的な開発には以下のソフトウェアを使用します。

- Git
- VS Code
- rustup
- KiCad
- uv

実際に使用するMCUや通信方式によって、追加のツールやライブラリが必要になる場合があります。

## Related Repositories

- [mavlink-dialect](https://github.com/TeamMeltingPoppo/mavlink-dialect)
- [mavlink-rs](https://github.com/TeamMeltingPoppo/mavlink-rs)
- [mavlink-python](https://github.com/TeamMeltingPoppo/mavlink-python)
- [template-platformio](https://github.com/TeamMeltingPoppo/template-platformio)

各repositoryの詳細については、それぞれのREADMEを参照してください。
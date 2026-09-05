# Debug Tools

## ディレクトリ構成

```text
tools/
├── .gitignore      # Git ignore file
├── .python-version # Python version file
├── pyproject.toml  # Python project configuration
├── README.md       #  Readme file
├── uv.lock         # uv dependency lock file
├── .venv/          # Virtual environment for debug scripts
├── .vscode/        # VSCode settings
├── scripts/        # Debug scripts
├── src/            # Source code
└── logs/           # Log files
```

scriptsディレクトリには、デバッグ用のスクリプトが格納されています。scriptsディレクトリ内のスクリプトは、.gitignoreによりGit管理から除外されています。必要に応じて.gitignoreを編集し、Git管理下に置くことも可能です。


## mavlink-python

MAVLinkのメッセージを送受信するためのPythonライブラリです。MAVLinkメッセージの定義、通信、メッセージの配信、アプリケーションによる処理を分離した構成になっています。

```mermaid
graph TB
    subgraph Core["Core API"]
        Topic["Topic"]
        Bridge["Bridge"]
        subgraph Publishers["Publishers"]
            Publisher["Publisher"]
            TLogReader["TlogReader"]
        end
        subgraph Subscribers["Subscribers"]
            Sub["Subscriber"]
            History["History"]
            Status["Status"]
            Recorder["Recorder"]
        end
        subgraph Generated["Generated from MAVLink Dialect"]
            MessageDefinition["Message Definition"]
            ParserPacker["Parser / Packer"]
        end
    end

    subgraph Transport["Transport"]
        UDP["UDP"]
        Serial["Serial"]
    end

    subgraph Mock["Mock API"]
        Node["Node"]
    end

    UDP <--bytes--> Bridge
    Serial <--bytes--> Bridge
    Node <--subscribe / publish--> Topic

    Bridge <--subscribe / publish--> Topic
    Publisher --publish--> Topic
    TLogReader --publish--> Topic
    Topic --subscribe--> Sub
    Topic --subscribe--> History
    Topic --subscribe--> Status
    Topic --subscribe--> Recorder
```
# dotnet DBアクセス

## 環境設定

ubuntu 環境にインストール

```
sudo add-apt-repository ppa:dotnet/backports

# dotnet 9
sudo apt-get update && \
  sudo apt-get install -y dotnet-sdk-9.0

# dotnet 10
sudo apt-get update && \
  sudo apt-get install -y dotnet-sdk-10.0
```

## プロジェクト作成

dotnet new console -n MyNewApp
d
## 開発時の実行

dotnet run

## build

dotnet build

* リリース buildで実行時にdotnetフレームワーク未インストールでも動作するように

dotnet publish -c Release --self-contained

# テンプレートの一覧を表示

dotnet new --list


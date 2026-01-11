# goでwebサーバー

# 準備

作った時のモジュールインストールコマンド

```
go mod init go02
go get github.com/gofiber/fiber/v2
```

# pull 後の作業開始の場合

```
go mod download
```

# build

```
make
or
make release
```

# サーバーstart

./build/[debug,release]/go2
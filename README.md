# 20260111_multilang

色々な言語のプログラム

* dotnet
* go
* java
* php
* python
* ruby
* rust
* scala

# DB アクセスプログラム 01

postgresql DBアクセス

* 環境変数から値を取得

```
DB_NAME: str = os.getenv("DB_NAME", "docsearch")
DB_USER: str = os.getenv("DB_USER", "docsearch")
DB_PASS: str = os.getenv("DB_PASS", "docsearch")
DB_HOST: str = os.getenv("DB_HOST", "localhost")
DB_PORT: str = os.getenv("DB_PORT", "5432")  # 文字列として読み込まれます
```

## rust

[ruat01](./doc/rust01.md)
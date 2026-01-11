# このプログラムについて

postgresql データベースからデータを取得する.

# 必要なモジュール

```
gem install pg 
```

## 作業ディレクトリに入れる場合

* Gemfile に使いたい gem を追加

# Gemfile
```
source 'https://rubygems.org'
gem 'pg'
```

```
# プロジェクトディレクトリで実行
bundle config set path 'vendor/bundle'
bundle install
```

# 実行

```
bundle exec ruby ruby01.rb
```


# DBアクセスのパラメータ

以下の環境変数から取得

* DB_NAME
* DB_USER
* DB_PASS
* DB_HOST
* DB_SCHEMA
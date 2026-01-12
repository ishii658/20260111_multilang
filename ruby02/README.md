# ruby での webサーバー

Sinatra

## 構築手順

```
mkdir ruby02
cd ruby02
```

### Gemfileを作成

```
# Gemfile
source 'https://rubygems.org'

gem 'sinatra'
gem 'puma'  # Webサーバー
```

### . Bundlerで依存関係をインストール

```
bundle config set path 'vendor/bundle'
bundle install
```

### アプリケーションファイルを作成

```
# app.rb
require 'sinatra'

get '/' do
  'Hello, Sinatra!'
end
```


### 起動

```
bundle exec ruby app.rb
```

## 確認

* staticページ
    * http://127.0.0.1:4567/index.html
* api
    * http://127.0.0.1:4567/api/test
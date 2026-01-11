# ruby 環境

ubuntuにruby環境のセットアップ

# インストール

ubuntu 22,04 のは古いので rbenv で構築

## 準備

必要なパッケージを入れる
```
sudo apt update
sudo apt install -y git curl libssl-dev libreadline-dev zlib1g-dev build-essential
```
### rbenv

```
git clone https://github.com/rbenv/rbenv.git ~/.rbenv
~/.rbenv/bin/rbenv init

git clone https://github.com/rbenv/ruby-build.git ~/.rbenv/plugins/ruby-build
```

### ruby install

```
# list latest stable versions:
rbenv install -l

# list all local versions:
rbenv install -L

# install a Ruby version:
rbenv install 3.4.7

gem install bundler
gem update --system 4.0.3
```

postgresql モジュール

```
gem install pg 
```
# typescript の環境構築

色々なバージョンのnodejsをインストールするために nvm をインストールして
nodejs を install する。


# nvm

* https://github.com/nvm-sh/nvm を参考にインストール

```
wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
# 環境変数を反映
. ~/.bashrc

# インストールできる version を確認
nvm ls-remote
# 最新のLTSを名前にinstall
nvm install --lts lts/krypton
# 確認
node --version
```
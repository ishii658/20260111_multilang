# python 環境作成

python 自体があれば venv で環境は作成できるが、別バージョンのpython が必要な場合もあるので
mini-forge でインストール

```
wget "https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-$(uname)-$(uname -m).sh"
ダウンロードした sh を実行
```

# 作業用の環境を作成

```
# 作業用ディレクトリ作成
mkdir python01
cd python01
conda create -n p312 python=3.12
conda activate p312
python -m venv venv
```

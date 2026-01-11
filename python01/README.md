# 作業用の環境を作成

```
# 作業用ディレクトリ作成
mkdir python01
cd python01
conda create -n p312 python=3.12
conda activate p312
python -m venv venv
```

# 作業用環境 activate

```
. venv/bin/activate
```

# モジュールインストール

```
pip install psycopg2-binary
```


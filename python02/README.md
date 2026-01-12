# python のwebサーバー,webapi サーバー

fastapi での web サーバー,web api サーバー

# 環境作成

* python 3.12以上
```
python -m venv venv
. venv/bin/activate
pip install fastapi uvicorn
```

# 実行

```
uvicorn server:app --reload --port 8001
```
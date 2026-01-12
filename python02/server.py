
"""fastapi のwebサーバーのコード."""

from pathlib import Path

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()


# /api/test で ok を返す
@app.get("/api/test")
async def api_test():
    return {"result": "ok"}
# /api/test で ok を返す

# ../static ディレクトリを / で配布
static_dir = Path(__file__).parent.parent / "static"
app.mount("/", StaticFiles(directory=static_dir, html=True), name="static")


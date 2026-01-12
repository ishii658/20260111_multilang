# typescript での webサーバー, web api サーバー

nextjs で web サーバー, web api サーバー


# 新規作成

## プロジェクトの作成

```
npx create-next-app@latest
```

## サーバー start

```
cd プロジェクト名
npm run dev
```

## api

app/api のしたにフォルダ作成。

* hello/route.ts

中身
```
import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({ message: 'Hello API!' });
}
```
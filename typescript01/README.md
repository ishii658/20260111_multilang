# postgresql データベースアクセスプログラム

# プロジェクト作成

```
mkdir typescript01
cd typescript01
npm init
```

必要なモジュールのインストール
```
npm install pg
npm install --save-dev @types/pg
npm install typescript
npm install @types/pg --save-dev
```

tsc を作成

```
npx tsc --init
```

# 設定

## tsconfig.json

```
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ES2020",
    "moduleResolution": "node",
    "lib": ["ES2020"],
    "types": ["node"],
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "outDir": "./dist",
    "rootDir": "./",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["**/*.ts"],
  "exclude": ["node_modules"]
}
```

## package.json

"type"フィールド変更
```
{
  "name": "typescript01",
  "version": "0.0.1",
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "ts-node typescript01.ts"
    "startjs": "node dist/typescript01.js"
  },
  "dependencies": {
    "pg": "^8.11.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "@types/pg": "^8.10.0"
  }
}
```

# build

```
npm run build
```

# 実行

```
node typescript01.js
```
import { Client } from 'pg';

// 環境変数からDB接続情報取得
const dbName = process.env.DB_NAME;
const dbUser = process.env.DB_USER;
const dbPass = process.env.DB_PASS;
const dbHost = process.env.DB_HOST;
const dbSchema = process.env.DB_SCHEMA;

// 接続情報のバリデーション
if (!dbName || !dbUser || !dbPass || !dbHost || !dbSchema) {
    throw new Error('DB接続情報が不足しています。環境変数を確認してください。');
}

// DB接続クライアント作成
const client = new Client({
    user: dbUser,
    host: dbHost,
    database: dbName,
    password: dbPass,
    port: 5432, // PostgreSQLのデフォルトポート
});

// ドキュメントインターフェース
interface Document {
    id: number;
    file_name: string;
}

// ドキュメントを取得する関数
async function getDocuments(): Promise<Document[]> {
    try {
        // DB接続
        await client.connect();

        // SQLクエリ実行
        const query = `
      SELECT id, file_name 
      FROM ${dbSchema}.documents
    `;

        const result = await client.query(query);

        // 取得結果を返す
        return result.rows as Document[];

    } catch (error) {
        console.error('DBアクセスエラー:', error);
        throw error;
    } finally {
        // 接続を閉じる
        await client.end();
    }
}

// メイン処理
async function main() {
    try {
        const documents = await getDocuments();
        console.log('取得したドキュメント一覧:');
        documents.forEach(doc => {
            console.log(`ID: ${doc.id}, ファイル名: ${doc.file_name}`);
        });
    } catch (error) {
        console.error('エラーが発生しました:', error);
    }
}

// プログラム実行
main();

export { getDocuments };
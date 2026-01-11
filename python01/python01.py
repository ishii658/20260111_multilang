"""postgreSQLデータベースからdocumentsテーブルのデータを取得して表示するスクリプト."""

import os

import psycopg2
from psycopg2 import sql


def get_db_connection() -> psycopg2.extensions.connection:
    """データベース接続を確立する関数."""
    try:
        return psycopg2.connect(
            database=os.getenv("DB_NAME"),
            user=os.getenv("DB_USER"),
            password=os.getenv("DB_PASS"),
            host=os.getenv("DB_HOST"),
            port="5432",  # デフォルトポート
        )
    except Exception as error:  # noqa: BLE001
        print(f"データベース接続エラー: {error}")
        return None


def fetch_documents():
    """documentsテーブルからデータを取得する関数"""
    conn = get_db_connection()
    if conn is None:
        return None

    try:
        cursor = conn.cursor()

        # SQLクエリを実行
        query = sql.SQL("SELECT id, file_name FROM {}.documents").format(sql.Identifier(os.getenv("DB_SCHEMA")))

        cursor.execute(query)

        # 結果を取得
        rows = cursor.fetchall()

        # カラム名を取得
        column_names = [desc[0] for desc in cursor.description]

        return rows, column_names

    except Exception as error:
        print(f"データ取得エラー: {error}")
        return None
    finally:
        # 接続を閉じる
        if conn:
            cursor.close()
            conn.close()


def main() -> None:
    """メイン関数."""
    # データを取得
    result = fetch_documents()

    if result is not None:
        rows, column_names = result

        print("取得したデータ:")
        print(f"カラム: {', '.join(column_names)}")
        print("-" * 50)

        for row in rows:
            print(f"ID: {row[0]}, ファイル名: {row[1]}")
    else:
        print("データの取得に失敗しました")


if __name__ == "__main__":
    main()

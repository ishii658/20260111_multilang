package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	_ "github.com/lib/pq"
)

func main() {
	// 環境変数から接続パラメータを取得
	dbName := os.Getenv("DB_NAME")
	dbUser := os.Getenv("DB_USER")
	dbPass := os.Getenv("DB_PASS")
	dbHost := os.Getenv("DB_HOST")
	dbSchema := os.Getenv("DB_SCHEMA")

	// 接続文字列を構築
	connStr := fmt.Sprintf("dbname=%s user=%s password=%s host=%s search_path=%s sslmode=disable",
		dbName, dbUser, dbPass, dbHost, dbSchema)

	// データベースに接続
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatal("データベース接続エラー:", err)
	}
	defer db.Close()

	// 接続の確認
	err = db.Ping()
	if err != nil {
		log.Fatal("データベースへの接続確認エラー:", err)
	}

	fmt.Println("データベースに正常に接続しました")

	// 指定スキーマのdocumentsテーブルからidとfile_nameを取得（limit 3）
	query := fmt.Sprintf("SELECT id, file_name FROM %s.documents LIMIT 3", dbSchema)
	
	rows, err := db.Query(query)
	if err != nil {
		log.Fatal("クエリ実行エラー:", err)
	}
	defer rows.Close()

	// データを表示
	fmt.Println("id, file_name のデータ:")
	fmt.Println("------------------------")
	
	for rows.Next() {
		var id int
		var fileName string
		
		// カラムの値をスキャン
		err := rows.Scan(&id, &fileName)
		if err != nil {
			log.Fatal("結果のスキャンエラー:", err)
		}
		
		fmt.Printf("ID: %d, ファイル名: %s\n", id, fileName)
	}

	// エラーの確認
	if err = rows.Err(); err != nil {
		log.Fatal("行の処理エラー:", err)
	}

	fmt.Println("データ取得完了")
}
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

	// シンプルなクエリ実行例
	rows, err := db.Query("SELECT version()")
	if err != nil {
		log.Fatal("クエリ実行エラー:", err)
	}
	defer rows.Close()

	// クエリ結果の表示
	for rows.Next() {
		var version string
		err := rows.Scan(&version)
		if err != nil {
			log.Fatal("結果のスキャンエラー:", err)
		}
		fmt.Println("PostgreSQLバージョン:", version)
	}
}
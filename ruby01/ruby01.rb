#!/usr/bin/env ruby

require 'pg'

# 環境変数から接続情報取得
db_name = ENV['DB_NAME'] || 'docsearch'
db_user = ENV['DB_USER'] || 'docsearch'
db_pass = ENV['DB_PASS'] || 'docsearch'
db_host = ENV['DB_HOST'] || 'localhost'
db_schema = ENV['DB_SCHEMA'] || 'pdfdir'

begin
  # PostgreSQL接続
  conn = PG.connect(
    dbname: db_name,
    user: db_user,
    password: db_pass,
    host: db_host
  )

  # スキーマを指定してテーブルからデータ取得
  sql = "SELECT id, file_name FROM #{db_schema}.documents LIMIT 5"
  result = conn.exec(sql)

  # 結果表示
  puts "ID\t\tファイル名"
  puts "-" * 50
  result.each do |row|
    puts "#{row['id']}\t\t#{row['file_name']}"
  end

rescue PG::Error => e
  puts "データベースエラー: #{e.message}"
rescue => e
  puts "エラー: #{e.message}"
ensure
  # 接続クローズ
  conn&.close
end
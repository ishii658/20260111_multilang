using System;
using Npgsql;

// 環境変数から接続情報を取得
var dbName = Environment.GetEnvironmentVariable("DB_NAME");
var dbUser = Environment.GetEnvironmentVariable("DB_USER");
var dbPass = Environment.GetEnvironmentVariable("DB_PASS");
var dbHost = Environment.GetEnvironmentVariable("DB_HOST") ?? "localhost";
var dbSchema = Environment.GetEnvironmentVariable("DB_SCHEMA") ?? "public";

if (string.IsNullOrEmpty(dbName) || string.IsNullOrEmpty(dbUser) || string.IsNullOrEmpty(dbPass))
{
    Console.Error.WriteLine("必要な環境変数(DB_NAME, DB_USER, DB_PASS) が設定されていません。設定を確認してください。");
    return;
}

var connString = $"Host={dbHost};Username={dbUser};Password={dbPass};Database={dbName};SearchPath={dbSchema}";

using var conn = new NpgsqlConnection(connString);
try
{
    conn.Open();
    Console.WriteLine($"Connected to database '{dbName}' on host '{dbHost}', using schema '{dbSchema}'.");

    // 指定スキーマ内のテーブル一覧を取得
    using var cmd = new NpgsqlCommand("SELECT table_name FROM information_schema.tables WHERE table_schema = @schema ORDER BY table_name;", conn);
    cmd.Parameters.AddWithValue("schema", dbSchema);

    using var reader = cmd.ExecuteReader();
    Console.WriteLine("Tables:");
    var hasAny = false;
    while (reader.Read())
    {
        hasAny = true;
        Console.WriteLine($" - {reader.GetString(0)}");
    }

    if (!hasAny)
    {
        Console.WriteLine("(no tables found in schema)");
    }
}
catch (Exception ex)
{
    Console.Error.WriteLine($"Error: {ex.Message}");
}

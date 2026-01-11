<?php
// 環境変数の取得
$dbName = getenv('DB_NAME');
$dbUser = getenv('DB_USER');
$dbPass = getenv('DB_PASS');
$dbHost = getenv('DB_HOST');
$dbSchema = getenv('DB_SCHEMA');

// 接続文字列の作成
$dsn = "pgsql:host={$dbHost};port=5432;dbname={$dbName};user={$dbUser};password={$dbPass}";

try {
    // PDOを使用してデータベースに接続
    $pdo = new PDO($dsn, $dbUser, $dbPass, [
        PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
        PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
    ]);
    
    // SQLクエリの作成
    $sql = "SELECT id, file_name FROM {$dbSchema}.documents";
    
    // クエリの実行
    $stmt = $pdo->prepare($sql);
    $stmt->execute();
    
    // 結果の取得
    $results = $stmt->fetchAll();
    
    // 結果の表示
    echo "取得したデータ:\n";
    echo "================\n";
    
    if (count($results) > 0) {
        foreach ($results as $row) {
            echo "ID: " . $row['id'] . ", ファイル名: " . $row['file_name'] . "\n";
        }
    } else {
        echo "データが見つかりません。\n";
    }
    
} catch (PDOException $e) {
    // エラー処理
    echo "データベース接続エラー: " . $e->getMessage() . "\n";
    exit(1);
} catch (Exception $e) {
    // その他のエラー処理
    echo "エラー: " . $e->getMessage() . "\n";
    exit(1);
}
?>
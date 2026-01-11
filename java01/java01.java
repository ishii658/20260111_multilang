import java.sql.*;

public class java01 {
    public static void main(String[] args) {
        // 環境変数から接続情報取得
        String dbName = System.getenv("DB_NAME");
        String dbUser = System.getenv("DB_USER");
        String dbPass = System.getenv("DB_PASS");
        String dbHost = System.getenv("DB_HOST");
        String dbSchema = System.getenv("DB_SCHEMA");

        // JDBC接続URL
        String url = "jdbc:postgresql://" + dbHost + ":5432/" + dbName;

        try {
            // ドライバロード
            Class.forName("org.postgresql.Driver");

            // 接続
            Connection conn = DriverManager.getConnection(url, dbUser, dbPass);

            // SQL実行
            String sql = "SELECT id, file_name FROM " + dbSchema + ".documents LIMIT 10;";
            PreparedStatement stmt = conn.prepareStatement(sql);
            ResultSet rs = stmt.executeQuery();

            // 結果出力
            while (rs.next()) {
                System.out.println(rs.getString(1) + ", " + rs.getString(2)); // 例: 1列目と2列目を出力
            }

            // 終了処理
            rs.close();
            stmt.close();
            conn.close();

        } catch (ClassNotFoundException e) {
            System.err.println("JDBCドライバが見つかりません。");
            e.printStackTrace();
        } catch (SQLException e) {
            System.err.println("データベース接続またはクエリ実行に失敗しました。");
            e.printStackTrace();
        }
    }
}

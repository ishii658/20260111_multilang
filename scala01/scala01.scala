import org.apache.spark.sql.SparkSession
import java.util.Properties

object scala01 {
  def main(args: Array[String]): Unit = {
    // 1. SparkSessionの初期化
    // このプログラムを spark-submit で実行する場合、
    // SparkSession は自動的に初期化される環境で実行されます。
    val spark = SparkSession.builder
      .appName("PostgresReader")
      // ローカル環境でテストする場合
      // .master("local[*]")
      .getOrCreate()
    
    // 2. JDBC接続情報の設定
    // 実際の環境に合わせて値を変更してください
    val jdbcUrl = "jdbc:postgresql://localhost:5432/docsearch"
    val dbTable = "pdfdir.documents" // 読み込みたいテーブル名
    val dbUser = "docsearch"
    val dbPassword = "docsearch"
    
    // 3. 接続プロパティの設定
    val connectionProperties = new Properties()
    connectionProperties.put("user", dbUser)
    connectionProperties.put("password", dbPassword)
    connectionProperties.put("driver", "org.postgresql.Driver")
    
    // 4. Postgresからのデータ読み込み
    try {
      val df = spark.read.jdbc(
        url = jdbcUrl,
        table = dbTable,
        properties = connectionProperties
      )
      
      // 5. データの一部を表示
      println(s"Successfully read data from table: $dbTable")
      df.show()
      
    } catch {
      case e: Exception =>
        println(s"Error reading from PostgreSQL: ${e.getMessage}")
        e.printStackTrace()
    } finally {
      // 6. SparkSessionの停止
      spark.stop()
    }
  }
}

# scalaのDBアクセス

# scala 環境作成

ubuntu での作成例

## java
```
sudo apt install -y openjdk-11-jdk
```

## scala

ホームディレクトリに cs コマンドをダウンロードしてセットアップ

```
cd
curl -fL https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-pc-linux.gz | gzip -d > cs && chmod +x cs && ./cs setup

# パスを通す
echo "export PATH=$PATH:$HOME/.local/share/coursier/bin" >> ~/.bashrc
# 設定を読みおなす
. ~/.bashrc
```
* https://www.scala-lang.org/download/ を参考にinstall

## spark

### ダウンロード

apache spark のサイトからダウンロード

* https://spark.apache.org/downloads.html
バージョンを選んでダウンロード

## 設定

* spark-4.1.1-bin-hadoop3.tgz を展開
  * cd
  * wget https://downloads.apache.org/spark/spark-4.1.1/spark-4.1.1-bin-hadoop3.tgz
  * tar xvzf spark-4.1.1-bin-hadoop3.tgz
* 環境変数設定

.bashrcなどに以下を追加

```
##  環境変数を設定
export SPARK_HOME=$HOME/spark-4.1.1-bin-hadoop3
export PATH=$PATH:$SPARK_HOME/bin
export SCALA_LIBRARY_PATH=$SPARK_HOME/jars/*
```


# Postgresql JDBC

## ダウンロード
* https://jdbc.postgresql.org/download/
  * https://jdbc.postgresql.org/download/postgresql-42.7.8.jar
## 配置
* SPARK_HOME/jars に
  * $HOME/spark-4.1.1-bin-hadoop3/jars

## jar 作成　script

必要なものをまとめて jar を作成

```
# 作業ディレクトリにclassをコピー
# CLASSPATHを作成 (改行をコロンに変換し、変数CPに格納)
CP=$(find $SPARK_HOME/jars -name "*.jar" | tr '\n' ':')
# 参照設定
scalac -cp "$CP" PostgresReader.scala
# jarに
jar -cvf postgres-reader.jar *.class
```

# .gitignore

```
# Scala/Java コンパイル生成物
*.class
*.tasty
*.jar

# 開発環境で生成される可能性のあるファイル
# sbtやIDEを使用する場合に備えて含めておくと良いです
target/
project/
.idea/
.bsp/
```


# 実行

```
$SPARK_HOME/bin/spark-submit \
  --class scala01 \
  --master local[*] \
  --jars $SPARK_HOME/jars/postgresql-42.7.8.jar \
  scala01.jar
```

# 実行 2

サーバー起動しておいて、投げる

## サーバー起動

hostを指定して起動(IPは合わせて)
```
export SPARK_MASTER_HOST=192.168.11.11
$SPARK_HOME/sbin/start-master.sh
# Workerも Masterの新しいIPを指定して起動
$SPARK_HOME/sbin/start-worker.sh spark://192.168.11.11:7077
```
## サーバー停止

今は停止しない。
```
$SPARK_HOME/sbin/stop-master.sh
$SPARK_HOME/sbin/stop-worker.sh
```

## jar実行

```
$SPARK_HOME/bin/spark-submit \
  --class scala01 \
  --master spark://192.168.11.11:7077 \
  --deploy-mode client \
  --jars $SPARK_HOME/jars/postgresql-42.7.8.jar \
  scala01.jar
```


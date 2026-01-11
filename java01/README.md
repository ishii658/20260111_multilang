# postgresql アクセスサンプル

## DBアクセスのためのモジュール

* postgresql のjar をダウンロードして指定。
* 現在は上記jarは~/spark-4.1.1-bin-hadoop3/jars/postgresql-42.7.8.jarにある前提で
    * 他のpathにある場合は置き換えてください

## コンパイル

# JARファイルを指定してコンパイル

## build 
### script 作成. build.sh など

以下のコマンドを実行して build.shを作る。
```
cat > script.sh << EOF
#!/bin/bash

# JDBCドライバのパス
DB_JAR="$HOME/spark-4.1.1-bin-hadoop3/jars/postgresql-42.7.8.jar"

# コンパイル（SPARKのJARは不要）
javac -cp "\$DB_JAR" java01.java

# manifest.txt を作成（SPARKのJARは含めない）
echo "Main-Class: java01" > manifest.txt
echo "Class-Path: \$DB_JAR" >> manifest.txt

# jarに
jar cvfm java01.jar manifest.txt *.class

# クリーンアップ
rm -f *.class manifest.txt
EOF
```

### build実行

bash build.sh

## 実行
java -jar java01.jar
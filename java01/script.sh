#!/bin/bash

# JDBCドライバのパス
DB_JAR="/home/ishii/spark-4.1.1-bin-hadoop3/jars/postgresql-42.7.8.jar"

# コンパイル（SPARKのJARは不要）
javac -cp "$DB_JAR" java01.java

# manifest.txt を作成（SPARKのJARは含めない）
echo "Main-Class: java01" > manifest.txt
echo "Class-Path: $DB_JAR" >> manifest.txt

# jarに
jar cvfm java01.jar manifest.txt *.class

# クリーンアップ
rm -f *.class manifest.txt

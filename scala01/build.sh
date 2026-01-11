# SPARK_HOMEが設定されていることを確認してください
# echo $SPARK_HOME 

# CLASSPATHを作成 (改行をコロンに変換し、変数CPに格納)
CP=$(find $SPARK_HOME/jars -name "*.jar" | tr '\n' ':')

# 作成されたCLASSPATHを確認 (膨大なので注意)
# echo $CP

scalac -cp "$CP" scala01.scala

jar -cvf scala01.jar *.class
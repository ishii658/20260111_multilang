# javaでwebサーバー

# 準備

```
sudo apt install maven
```

## ブラウザでアクセス:

URL: https://start.spring.io/

## 設定項目を選択:

Project: Maven または Gradle (通常はGradleがモダン)

Language: Java

Spring Boot: 最新の安定バージョン

Group/Artifact: プロジェクトに合わせて入力（例: com.example / demo）

Packaging: Jar (組み込みサーバー用)

Java: 使用するJavaバージョン (例: 21, 17など)

## Dependencies (依存関係) を追加:

検索窓に「Web」と入力し、「Spring Web」を選択します。（静的ファイルとAPIサーバー機能に必須）

## 生成とダウンロード:

「Generate」ボタンをクリックすると、設定に応じたプロジェクトのZIPファイルがダウンロードされます。

そのZIPファイルを展開すれば、すぐに開発を始められるスケルトンが完成します。

# 展開後

静的ファイルの配置
```
src/main/resources/static/index.html
```


## api

```
Web APIコントローラーの作成
FastAPIでいう @app.get("/") のようなエンドポイントを定義するJavaクラスを作成します。

ファイル作成場所: src/main/java/com/example/demo/HelloApiController.java

HelloApiController.java のコード例:
```

```
package com.example.demo;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import java.util.Map;

// このアノテーションが、このクラスをRESTful APIのエンドポイントとしてマークします
@RestController
public class HelloApiController {

    // GETリクエストを /api/data にマッピング
    @GetMapping("/api/data")
    public Map<String, String> getData() {
        // Mapを返すと、Spring Bootが自動的にJSONに変換します
        return Map.of(
            "status", "success", 
            "message", "これはSpring Boot APIからのレスポンスです！"
        );
    }
}
```

# static ディレクトリ変更

src/main/resources/application.properties
を変更

のファイルに以下のプロパティを追加（または変更）します。

```
# 静的リソースの場所を 'public' ディレクトリに変更する例
spring.web.resources.static-locations=classpath:/public/
```

# アプリケーションの実行と確認

```
./mvnw spring-boot:run
```

```
http://localhost:8080/api/data
http://localhost:8080/
```
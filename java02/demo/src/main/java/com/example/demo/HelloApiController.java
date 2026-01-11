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
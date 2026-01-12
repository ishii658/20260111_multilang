# scala webサーバー

# プロジェクトのセットアップ


```
sbt new http4s/http4s.g8
# 名前に scala02
sbt run
http://localhost:8080/hello/Gemini
```

# ファイル

```
mkdir -p src/main/scala/scala02
vi src/main/scala/scala02/StaticRoutes.scala 
```

```
package scala02

import cats.effect.IO
import org.http4s._
import org.http4s.dsl.io._
import org.http4s.implicits._
import org.http4s.server.staticcontent._

object StaticRoutes {
  val routes: HttpRoutes[IO] = fileService[IO](
    FileService.Config(
      systemPath = "../..//static", // staticファイルのパス
      pathPrefix = "/" // ルートで配布
    )
  )
}
```





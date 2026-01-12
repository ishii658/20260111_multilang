package scala02

import cats.effect.IO
import org.http4s._
// import org.http4s.dsl.io._
// import org.http4s.implicits._
import org.http4s.server.staticcontent._

object StaticRoutes {
  val routes: HttpRoutes[IO] = fileService[IO](
    FileService.Config(
      systemPath = "../../static", // staticファイルのパス
      pathPrefix = "/" // ルートで配布
    )
  )
}


package com.example.scala02

import cats.effect.{Async, IO}
import cats.syntax.all._
import com.comcast.ip4s._
import fs2.io.net.Network
import org.http4s.ember.client.EmberClientBuilder
import org.http4s.ember.server.EmberServerBuilder
import org.http4s.implicits._
import org.http4s.server.middleware.Logger
import scala02.StaticRoutes // 追加
import org.http4s.HttpRoutes

object Scala02Server {
  def run[F[_]: Async: Network]: F[Nothing] = {
    for {
      client <- EmberClientBuilder.default[F].build
      helloWorldAlg = HelloWorld.impl[F]
      jokeAlg = Jokes.impl[F](client)

      // StaticRoutes.routes は IO 固定なので、F=IO の場合のみ合成
      staticRoutes: HttpRoutes[IO] = StaticRoutes.routes

      // Combine Service Routes into an HttpApp.
      httpApp = (
        Scala02Routes.helloWorldRoutes[F](helloWorldAlg) <+>
        Scala02Routes.jokeRoutes[F](jokeAlg) <+>
        (staticRoutes.asInstanceOf[HttpRoutes[F]])
      ).orNotFound

      // With Middlewares in place
      finalHttpApp = Logger.httpApp(true, true)(httpApp)

      _ <-
        EmberServerBuilder.default[F]
          .withHost(ipv4"0.0.0.0")
          .withPort(port"8080")
          .withHttpApp(finalHttpApp)
          .build
    } yield ()
  }.useForever
}

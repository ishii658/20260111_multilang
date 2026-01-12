package com.example.scala02

import cats.effect.{IO, IOApp}

object Main extends IOApp.Simple {
  val run = Scala02Server.run[IO]
}

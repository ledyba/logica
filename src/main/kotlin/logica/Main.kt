package logica

import java.util.logging.Logger

fun main(args : Array<String>) {
  val log = Logger.getLogger("Main")

  val player = createPlayer()
  val seq = sequence {
    yieldAll(mux(newFMStream(player.fmt, 220.0f), newFMStream(player.fmt, 330.0f)))
  }
  player.play(seq)
}
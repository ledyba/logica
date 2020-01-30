package logica

import java.util.logging.Logger


fun main(args : Array<String>) {
  val log = Logger.getLogger("Main")

  val player = createPlayer()
  val fm = generateFMStream(player.fmt, 440.0f)
  player.play(fm)
}
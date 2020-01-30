package logica

import java.util.logging.Logger


fun main(args : Array<String>) {
  val log = Logger.getLogger("Main")

  val player = createPlayer()
  val seq = sequence {
    yieldAll(generateFMStream(player.fmt, 220.0f).take(player.fmt.frameRate.toInt()))
    yieldAll(generateFMStream(player.fmt, 220.0f * 2).take(player.fmt.frameRate.toInt()))
    yieldAll(generateFMStream(player.fmt, 220.0f * 1).take(player.fmt.frameRate.toInt()))
  }
  player.play(seq)
}
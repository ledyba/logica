package logica

import java.lang.Math.pow
import java.util.logging.Logger

fun main(args : Array<String>) {
  val log = Logger.getLogger("Main")

  val player = createPlayer()
  val freq = 330.0f;
  val seq = sequence {
    val a = newFMStream(player.fmt, 0.2f, freq)
    val b = newFMStream(player.fmt, 0.2f, freq * pow(3.0, 5.0/13.0).toFloat())
    val c = newFMStream(player.fmt, 0.2f, freq * pow(3.0, 7.0/13.0).toFloat())
    yieldAll(mux(a, b, c))
  }
  player.play(seq)
  //createRecorder().play("test.wav", seq, 10.0)
}
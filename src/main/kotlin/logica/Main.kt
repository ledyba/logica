package logica

import java.nio.ByteBuffer
import java.util.logging.Logger
import javax.sound.sampled.*
import kotlin.math.sin


fun main(args : Array<String>) {
  val log = Logger.getLogger("Main")

  val fmt = AudioFormat(AudioFormat.Encoding.PCM_SIGNED, 44100.0f, 16, 1, 2, 44100.0f, true);
  val line = AudioSystem.getSourceDataLine(fmt)

  val buff = ByteBuffer.allocate(44100 * 10)
  val stating = ByteArray(44100 * 10)

  var idx: Long  = 0
  line.open(fmt)
  line.start()
  try {
    while(true) {
      buff.clear()
      for(j in 0..44100) {
        idx++
        val t = idx.toDouble() / fmt.frameRate
        val v = sin(t * Math.PI * 2.0 * 220.0 + (3.5 * sin (t * Math.PI * 2 * 220.0*3.5))) * 0.2
        buff.putShort((v * 0x8000).toShort())
      }
      val size = buff.position()
      buff.get(0, stating, 0, size)
      line.write(stating, 0, size);
    }
  } finally {
    line.drain()
    line.close()
  }

}
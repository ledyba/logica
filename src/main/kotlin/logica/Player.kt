package logica

import java.nio.ByteBuffer
import javax.sound.sampled.AudioFormat
import javax.sound.sampled.AudioSystem
import javax.sound.sampled.SourceDataLine

class Player(val fmt: AudioFormat, private val sink: SourceDataLine) {
  val buff = ByteBuffer.allocate(44100 * 10)
  val stating = ByteArray(44100 * 10)

  fun play(stream: Sequence<Float>) {
    sink.open(fmt)
    sink.start()
    var idx = 0
    try {
      while(true) {
        buff.clear()
        for(v in stream.take(44100)) {
          buff.putShort((v * 0x8000).toShort())
        }
        idx+=44100
        val size = buff.position()
        buff.get(0, stating, 0, size)
        sink.write(stating, 0, size);
      }
    } finally {
      sink.drain()
      sink.close()
    }
  }
}

fun createPlayer():Player {
  val fmt = AudioFormat(AudioFormat.Encoding.PCM_SIGNED, 44100.0f, 16, 1, 2, 44100.0f, true);
  val sink = AudioSystem.getSourceDataLine(fmt)
  return Player(fmt, sink)
}

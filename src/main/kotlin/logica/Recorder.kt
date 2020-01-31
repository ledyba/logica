package logica

import java.io.ByteArrayInputStream
import java.io.File
import java.nio.ByteBuffer
import javax.sound.sampled.*


class Recorder(val fmt: AudioFormat) {

  fun play(fname:String, stream: Sequence<Float>, sec: Double) {
    val fileOut = File(fname)
    val numFrames = (fmt.frameRate * sec).toInt()
    val length = (numFrames * fmt.frameSize)
    val buff = ByteBuffer.allocate(length)
    stream.take(numFrames).forEach {
      buff.putShort((clamp(it) * (0x7fff)).toShort())
    }
    val audioInput = AudioInputStream(ByteArrayInputStream(buff.array()), fmt, length.toLong())
    AudioSystem.write(audioInput, AudioFileFormat.Type.WAVE, fileOut);
  }
}

fun createRecorder(): Recorder {
  val fmt = AudioFormat(AudioFormat.Encoding.PCM_SIGNED, 44100.0f, 16, 1, 2, 44100.0f, true);
  return Recorder(fmt)
}

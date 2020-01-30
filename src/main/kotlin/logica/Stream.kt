package logica

import javax.sound.sampled.AudioFormat
import kotlin.math.sin

fun generateFMStream(fmt: AudioFormat, baseFreq: Float) : Sequence<Float> {
  return sequence {
    var idx = 0
    while(true) {
      idx++
      val t = idx.toFloat() / fmt.frameRate
      val v = sin(t * Math.PI * 2.0 * baseFreq + (3.5 * sin (t * Math.PI * 2 * baseFreq * 3.5))) * 0.2
      yield(v.toFloat())
    }
  }
}
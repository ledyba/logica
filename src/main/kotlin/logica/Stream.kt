package logica

import javax.sound.sampled.AudioFormat
import kotlin.math.sin

fun clamp(v:Float): Float {
  return if (v < -1.0f) -1.0f else (if (v > 1.0f) 1.0f else v);
}

fun newFMStream(fmt: AudioFormat, volume: Float, baseFreq: Float) : Sequence<Float> {
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

fun newSinStream(fmt: AudioFormat, volume: Float, freq: Float) : Sequence<Float> {
  return sequence {
    var idx = 0
    while(true) {
      idx++
      val t = idx.toFloat() / fmt.frameRate
      val v = sin(t * Math.PI * 2.0 * freq) * volume
      yield(v.toFloat())
    }
  }
}

fun mux(vararg seqs: Sequence<Float>):Sequence<Float> {
  return seqs.reduce { acc, next ->
    acc.zip(next) { a, b -> a+b }
  }
}

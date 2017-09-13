import {Writable} from 'stream'
import {equal} from 'assert'
import {Stream} from './stream'

export default class Logica {

  /**
   * @param {number} channels 
   * @param {number} sampleRate
   * @param {number} capacity 
   */
  constructor(channels, sampleRate, capacity) {
    this.capacity = capacity || 0;
    this.channels_ = channels;
    this.sampleRate_ = sampleRate;
  }
  /**
   * @param {Stream} master
   * @param {number=} lengthSec
   * @param {Writable} stream
   */
  run(master, lengthSec, stream) {
    stream = stream || process.stdout;
    lengthSec = lengthSec || -1;
    var length = lengthSec < 0 ? lengthSec : (lengthSec * this.channels_ * this.sampleRate_) | 0;
    length = master.length < 0 ? length : Math.min(length, master.length);
    if(length < 0) {
      console.error("No length.");
      return;
    }
    var fbuf = new Float32Array(length);
    master.calc(0, length, fbuf);
    var buf = Buffer.alloc(length * 2);
    var off = 0;
    for(var i=0;i<length;i++) {
      off = buf.writeInt16LE(fbuf[i] * 32767, off);
    }
    this.writeHeader_(stream, length);
    stream.write(buf);
  }

  /**
   * @private
   * @param {Writable} stream
   * @param {number} length
   */
  writeHeader_(stream, length) {
    var header_len = (4+4) + 4 + 4 + (4 + 16) + (4+4);
    var body_len = length * 2;

    var buf = Buffer.alloc(header_len, 0, 'ascii');
    var off = buf.write('RIFF', 0);
    off = buf.writeInt32LE(header_len + body_len - 8, off);
    off = off + buf.write('WAVE', off);

    off = off + buf.write('fmt ', off);
    off = buf.writeInt32LE(16, off);
    off = buf.writeInt16LE(1, off);
    off = buf.writeInt16LE(this.channels, off);
    off = buf.writeInt32LE(this.sampleRate, off);
    off = buf.writeInt32LE(this.channels * this.sampling_rate * 2, off);
    off = buf.writeInt16LE(this.channels * 2, off);
    off = buf.writeInt16LE(16, off);
    off = off + buf.write('data', off);
    off = buf.writeInt32LE(body_len, off);
    equal(off, buf.length);
    
    stream.write(buf);
  }

  /** @returns {number} */
  get length() {
    return this.length_;
  }

  /** @returns {number} */
  get sampleRate() {
    return this.sampleRate_;
  }

  /** @returns {number} */
  get channels() {
    return this.channels_;
  }

}
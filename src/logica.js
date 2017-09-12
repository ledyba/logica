import {Writable} from 'stream'
import {equal} from 'assert'
import Stream from './stream'

export default class Logica {

  /**
   * @param {number} channels 
   * @param {number} sampling_rate
   * @param {number} capacity 
   */
  constructor(channels, sampling_rate, capacity) {
    this.capacity = capacity || 0;
    this.channels = channels;
    this.sampling_rate = sampling_rate;
    this.length_ = 0;
  }
  /** @param {Writable} stream */
  run(stream) {
    stream = stream || process.stdout;

    var buf = Buffer.alloc(2560000);
    var off = 0;
    var i = 0;
    for(;off < buf.length;) {
      i++;
      off = buf.writeInt16LE(32767 * Math.sin(i / 100), off);
      off = buf.writeInt16LE(32767 * Math.sin(i / 120), off);
      this.length_++;
    }
    this.writeHeader_(stream);
    stream.write(buf);
  }

  /**
   * 
   * @param {Stream} stream 
   */
  addStream(stream) {

  }

  /** @private
   * @param {Writable} stream
   */
  writeHeader_(stream) {
    var header_len = (4+4) + 4 + 4 + (4 + 16) + (4+4);
    var body_len = this.length * this.channels * 2;

    var buf = Buffer.alloc(header_len, 0, 'ascii');
    var off = buf.write('RIFF', 0);
    off = buf.writeInt32LE(header_len + body_len - 8, off);
    off = off + buf.write('WAVE', off);

    off = off + buf.write('fmt ', off);
    off = buf.writeInt32LE(16, off);
    off = buf.writeInt16LE(1, off);
    off = buf.writeInt16LE(this.channels, off);
    off = buf.writeInt32LE(this.sampling_rate, off);
    off = buf.writeInt32LE(this.channels * this.sampling_rate * 2, off);
    off = buf.writeInt16LE(this.channels * 2, off);
    off = buf.writeInt16LE(16, off);
    off = off + buf.write('data', off);
    off = buf.writeInt32LE(body_len, off);
    equal(off, buf.length);
    
    stream.write(buf);
  }

  /** @returns number */
  get length() {
    return this.length_;
  }
}
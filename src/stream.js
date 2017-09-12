
export class Stream {
  /**
   * 
   * @param {Logica} logica 
   */
  constructor(logica) {
    this.logica_ = logica;
  }

  /** @returns {number} */
  get length() {
    throw new Error("Not Implemented");
  }

  /**
   * @param {Stream} streams 
   * @returns {Stream}
   */
  mix(streams) {

  }

  /**
   * @param {Stream} streams 
   * @returns {Stream}
   */
  map(streams) {
  }

  /**
   * @param {number=} from 
   * @param {number=} to 
   * @param {Float64Array} buf
   */
  calc(from, to, buf) {
    from = from || 0;
    to = to || 0;
    throw new Error("Not Implemented");
  }

  /** @returns {Logica} */
  get logica() {
    return this.logica_;
  }

  /** @returns {number} */
  get sampleRate() {
    return this.logica_.sampleRate;
  }

  /** @returns {number} */
  get channels() {
    return this.logica_.channels;
  }
}

export class BufferSource extends Stream {
  /**
   * @param {Logica} logica
   */
  constructor(logica) {
    super(logica);
  }
  
}

export class ScriptSource extends Stream {
  /**
   * @param {Logica} logica
   * @param {function(number)[number]}
   */
  constructor(logica, func) {
    super(logica);
    this.func_ = func;
  }
  
  get length() {
    return -1;
  }

  /**
   * @param {number=} from 
   * @param {number=} to 
   * @param {Float64Array} buf
   */
  calc(from, to, buf) {
    from = from || 0;
    to = to || 0;

    var ch = this.channels;
    var rate = this.sampleRate;
    var func = this.func_;
    
    for(var i=from;i<to;i+=ch) {
      func(i / (rate * ch), buf.subarray(i, i+ch));
    }
  }
}

class MixStream extends Stream {
}

class MapStream extends Stream {
}

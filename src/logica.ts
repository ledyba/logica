declare const Buffer
declare const Writer

export class Logica {
    writer_: Writer;
    /**
     * 
     * @param {wav.Writer} writer 
     */
    constructor(writer) {
        this.writer_ = writer;
    }
    /**
     * 
     */
    test() {
        this.writer_.write(new Buffer([1, 2, 3]));
    }
}

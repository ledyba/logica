var wav = require('wav');
var Speaker = require('speaker');
var vm = require('vm');
var fs = require('fs');

class Logica {
    /**
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

/**
 * 
 * @param {function([string],Logica)} score 
 */
function play(score) {
    var argv = process.argv.slice(2);
    var w = wav.Writer();
    var r = wav.Reader();
    w.pipe(r);
    r.on('format', function (format) {
        //TODO: great log.
        console.info("Format");
        w.pipe(new Speaker(format));
    });
    if(argv.length > 0) {
        var fname = argv.shift();
        console.log("Write to ", fname);
        var fw = wav.FileWriter(fname);
        r.pipe(fw);
    }
    score(argv, new Logica(w));
}

module.exports = {
    play: play,
};

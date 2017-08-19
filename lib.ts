import wav = require('wav');
import Speaker = require('speaker');
import vm = require('vm');
import fs = require('fs');

import Logica from './src/logica'

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
        console.info("Format:");
        console.info(format);
        w.pipe(Speaker(format));
    });
    if (argv.length > 0) {
        var fname = argv.shift();
        console.log("Write to ", fname);
        var fw = wav.FileWriter(fname);
        w.pipe(fw);
    }
    score(argv, new Logica(w));
}

module.exports = {
    play: play,
};

import {Logica, Stream} from './src/lib'

var lg = new Logica(2, 44100, 0);

var stream = new Stream();
lg.addStream(stream);

lg.run();
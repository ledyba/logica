import * as logica from './src/lib'

var lg = new logica.Logica(2, 44100, 0);

var stream = new logica.ScriptSource(lg, function(t, b) {
  var f =  5 + Math.sin(t * Math.PI * 2);
  var delta =  5 * Math.sin(t * Math.PI * 2);
  var f2 = 440 + Math.sin(t * Math.PI * 2 * f);
  var v = (Math.sin(t * Math.PI * 2 * f2) + Math.sin(t * Math.PI * 2 * (delta+f2)))/2; 
  b[0] = v;
  b[1] = v;
});

lg.run(stream, 100);
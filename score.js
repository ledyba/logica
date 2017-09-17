import * as logica from './src/lib'

let lg = new logica.Logica(2, 44100, 0);

let stream = new logica.ScriptSource(lg, function(t, b) {
  let f =  5 + Math.sin(t * Math.PI * 2);
  let delta =  5 * Math.sin(t * Math.PI * 2);
  let f2 = 440 + Math.sin(t * Math.PI * 2 * f);
  let v = (Math.sin(t * Math.PI * 2 * f2) + Math.sin(t * Math.PI * 2 * f))/2; 
  b[0] = v;
  b[1] = v;
});

lg.run(stream, 100);

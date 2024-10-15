rmdir /S /Q "C:\\Program Files\\Common Files\\vst3\\ledyba\\Logica.vst3\\"
robocopy /mir "C:\\Users\\kaede\src\\code.ledyba.org\\ledyba\\logica\\cmake-build-debug-visual-studio\\VST3\\Debug\\Logica.vst3\\" "C:\\Program Files\\Common Files\\vst3\\ledyba\\Logica.vst3\\" /log+:install.log

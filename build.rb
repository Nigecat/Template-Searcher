Dir.mkdir("bin") unless Dir.exist?("bin");

system("ocra src/main.rb --gem-full --add-all-core --windows --gemfile Gemfile --icon resources/icon.ico --chdir-first --output 'bin/Template Searcher.exe'");
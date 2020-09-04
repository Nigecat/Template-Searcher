require_relative "display.rb"

# If this is running from the compiler checking dependencies then immediately exit
if defined?(Ocra)
    exit(0)
end

displayer = ImageDisplayer.new
displayer.showImage(ARGV[0])
sleep(2)
displayer.showImage(ARGV[1])
puts "a"
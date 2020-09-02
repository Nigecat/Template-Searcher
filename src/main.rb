require "ruby2d"
require "fastimage"

# If this is running from the compiler checking dependencies then immediately exit
if defined?(Ocra)
    exit(0)
end

image_size = FastImage.size(ARGV[0])

set title: "Template Searcher"
set width: image_size[0]
set height: image_size[1]

Image.new(
    ARGV[0],
    width: image_size[0], height: image_size[1]
)

show
require "ruby2d"
require "fastimage"

class ImageDisplayer
    def new()
        set title: "Template Searcher"
        set diagnostics: true
        self.img = nil
    end

    def showImage(path)
        image_size = FastImage.size(path)

        set width: image_size[0]
        set height: image_size[1]
        
        # Delete the previous image
        self.img.remove unless img.nil? || img.empty?
        
        self.img = Image.new(path, width: image_size[0], height: image_size[1])
        show
    end

    def close()
        close
    end
end
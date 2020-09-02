require "fox16"

include Fox

# If this is running from the compiler checking dependencies then immediately exit
if defined?(Ocra)
    exit(0)
end

application = FXApp.new("Template Searcher", "Template Searcher")
main = FXMainWindow.new(application, "Template Searcher", nil, nil, DECOR_ALL)
application.create()
main.show(PLACEMENT_SCREEN)
application.run()
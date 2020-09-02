require "fox16"

include Fox

application = FXApp.new("Template Searcher", "Template Searcher")
main = FXMainWindow.new(application, "Template Searcher", nil, nil, DECOR_ALL)
application.create()
main.show(PLACEMENT_SCREEN)
application.run()
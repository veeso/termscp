import "./just/build.just"
import "./just/changelog.just"
import "./just/code_check.just"
import "./just/publish.just"
import "./just/run.just"
import "./just/site.just"
import "./just/test.just"

# Lists all the available commands
default:
    @just --list

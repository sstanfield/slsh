#!/usr/bin/env sl-sh

(ns-import 'shell)

(pushd "~/development/slsh/docs")
$(bundle exec jekyll serve --config _user_config.yml)


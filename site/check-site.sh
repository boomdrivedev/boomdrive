#!/bin/sh
# site:test, the site's tests: each scenario of the site spec that names
# site:test (design Decision 3). It runs in site/, once site:build has
# built dist.
set -eu

status=0
fail() {
  echo "check-site: $*" >&2
  status=1
}

# Every link under the base, and boomdrive-web's name gone (site.6): each
# page's own links and assets are under /boomdrive/, the one repository a
# page links is boomdrive's, and nothing the site writes names
# boomdrive-web. The family's entries @agentplot/site-theme draws, its
# Boomdrive member's site and its hub, stand as the theme gives them
# (record 0040), read here from the theme itself, so they move with its
# release at the opening.
theme=$(bun -e 'import { member, hub } from "@agentplot/site-theme/family"; console.log(member("boomdrive").site); console.log(hub);') ||
  fail "@agentplot/site-theme gave no Boomdrive site or hub"
pages=$(find dist -name '*.html' | sort)
[ -n "$pages" ] || fail "dist holds no page: site:build builds it"
for page in $pages; do
  links=$(grep -oE '(href|src)="[^"]*"' "$page" | sed -E 's/^(href|src)="(.*)"$/\2/' | sort -u)
  for link in $links; do
    case $link in
    /boomdrive/*) ;;
    /*) fail "$page links $link, outside /boomdrive/" ;;
    esac
    case $link in
    https://github.com/agentplot/boomdrive | https://github.com/agentplot/boomdrive/*) ;;
    https://github.com/*/*) fail "$page links the repository $link, not https://github.com/agentplot/boomdrive" ;;
    esac
  done
  named=$(grep -oE "[^\"' <>()]*boomdrive-web[^\"' <>()]*" "$page" | sort -u || true)
  for name in $named; do
    if ! printf '%s\n' "$theme" | grep -qxF "$name"; then
      fail "$page names $name"
    fi
  done
done
[ "$status" -ne 0 ] || echo "check-site: $(printf '%s\n' "$pages" | wc -l | tr -d ' ') pages, every link under /boomdrive/ and none of boomdrive-web's but the theme's"

# The workflow's trigger, The workflow's build: the workflow pages read
# line by line, as check-releases.sh reads the workflow release, and as
# yamlfmt lays it out.
workflow=../.github/workflows/pages.yml
# The lines of the top-level block `$1`, less the empty ones and comments.
block() {
  awk -v key="$1:" '
    $0 == key { inside = 1; next }
    inside && /^[^ #]/ { exit }
    inside && !/^ *(#|$)/ { print }
  ' "$workflow"
}
on=$(block on)
want_on="  push:
    branches:
      - main
    paths:
      - site/**
      - .github/workflows/pages.yml
      - devenv.nix
      - devenv.yaml
      - devenv.lock
      - flake.lock
  workflow_dispatch:"
if [ "$on" = "$want_on" ]; then
  echo "check-site: the workflow pages runs on a push to main that changes the site, its shell or flake.lock, and when dispatched"
else
  fail "the workflow pages runs on more or less than a push to main that changes the site, its shell or flake.lock, and a dispatch:
$on"
fi
concurrency=$(block concurrency)
want_concurrency="  group: pages
  cancel-in-progress: false"
if [ "$concurrency" = "$want_concurrency" ]; then
  echo "check-site: the workflow pages runs one at a time, none cancelled"
else
  fail "the workflow pages is not one run at a time, none cancelled:
$concurrency"
fi
# Each line `$1` matches, as a basic regular expression, once.
once() {
  [ "$(grep -c -e "$1" "$workflow")" -eq 1 ]
}
if once '^        run: devenv shell -- moon run site:build$' &&
  once '^      - uses: actions/upload-pages-artifact@[0-9a-f]\{40\} # v' &&
  once '^          path: site/dist$' &&
  once '^      name: github-pages$' &&
  once '^        uses: actions/deploy-pages@[0-9a-f]\{40\} # v'; then
  echo "check-site: the workflow pages builds the site in the devenv shell, uploads site/dist and deploys it to github-pages"
else
  fail "the workflow pages does not build the site in the devenv shell, upload site/dist and deploy it to github-pages"
fi
if grep -n -E 'run: .*(site-check|site:(check|prose|test|workflow)|moon run :|sluice)' "$workflow"; then
  fail "the workflow pages runs a check"
else
  echo "check-site: the workflow pages runs no check"
fi

# The kind's prompts (site.5): each stage's prompt of [kinds.content] in
# .config/flume.toml, resolved against that file's own folder as flume
# resolves it, is a file under site/docs/prompts/.
flume=../.config/flume.toml
prompts=$(awk '
  /^\[/ { inside = ($0 == "[kinds.content]" || $0 == "[[kinds.content.stages]]") }
  inside && /^prompt *= *"/ { sub(/^prompt *= *"/, ""); sub(/".*$/, ""); print }
' "$flume")
[ -n "$prompts" ] || fail "$flume gives [kinds.content] no prompt"
here=$(cd docs/prompts && pwd -P)
for prompt in $prompts; do
  folder=$(cd "../.config/$(dirname "$prompt")" 2>/dev/null && pwd -P) || folder=""
  if [ "$folder" = "$here" ] && [ -f "$folder/$(basename "$prompt")" ]; then
    echo "check-site: the content kind's prompt $prompt is site/docs/prompts/$(basename "$prompt")"
  else
    fail "the content kind's prompt $prompt names no file under site/docs/prompts/"
  fi
done

# A release where nothing was served: site-stop, which sluice runs after
# every candidate's checks, with a state folder that holds no port, exits
# zero.
state=$(mktemp -d "${TMPDIR:-/tmp}/site-stop.XXXXXX")
if said=$(DEVENV_STATE=$state site-stop 2>&1); then
  echo "check-site: site-stop exits zero where nothing was served"
else
  fail "site-stop fails where nothing was served: $said"
fi
rm -rf "$state"

exit "$status"

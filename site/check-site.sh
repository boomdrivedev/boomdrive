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
# (record 0039), read here from the theme itself, so they move with its
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

exit "$status"

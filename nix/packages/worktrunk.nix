# worktrunk, worktrunk's own flake's package (design Decision 2).
# worktrunk's release build gets its tag from `git describe` as
# VERGEN_GIT_DESCRIBE, which `wt --version` prints, and worktrunk's flake
# gives its build the commit's short revision in its place; so the set
# gives it the tag flake.nix names, as that build gets it, and changes
# nothing else.
{ package, tag }: package.overrideAttrs { VERGEN_GIT_DESCRIBE = tag; }

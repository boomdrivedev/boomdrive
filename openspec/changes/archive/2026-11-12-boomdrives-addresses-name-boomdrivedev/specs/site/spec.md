## MODIFIED Requirements

### Requirement: The site is served under /boomdrive/

The site SHALL be built for the base `/boomdrive/`, and every link and asset of its own pages SHALL be under it. Nothing the site writes SHALL name boomdrive-web, and its repository link SHALL be `https://github.com/boomdrivedev/boomdrive` (site.6). The family's entries `@agentplot/site-theme` draws, its Boomdrive member's `site` and `hub`, SHALL stand as the theme gives them (record 0040), until its release at the opening.

#### Scenario: Every link under the base

- **WHEN** `site:test` reads every page of `site/dist`
- **THEN** each `href` and `src` that starts with `/` starts with `/boomdrive/`

#### Scenario: boomdrive-web's name is gone

- **WHEN** `site:test` reads every page of `site/dist`
- **THEN** no page names boomdrive-web but in the entries `@agentplot/site-theme`'s family gives, its Boomdrive member's `site` and its `hub`, and every page that links the site's repository links `https://github.com/boomdrivedev/boomdrive`

## MODIFIED Requirements

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL describe version 2: `runner/enroll` and `runner/connect` under the control plane's address, the kinds each takes, the subprotocol `boomdrive.v2`, the `BoomdriveHost` header, what a `pulled` and a `report` hold, how version 1 differs, and the bytes each signature covers, each context starting `boomdrive.v2`, with a copy of the worked examples the hosted tests read (record 0049). The hosted implementation SHALL sign exactly those bytes (records 0018 and 0037).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's and the connection's contexts from the fixed key, token, origin, time and nonce of the worked examples, `crates/boomdrive-hosted/tests/worked-bytes.json`
- **THEN** each equals the worked examples' bytes, and each starts with the bytes of `boomdrive.v2`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 2; its paths name the subprotocol `boomdrive.v2`; its files name `preview.schema.json`; its copies name an end's `delivery`, an end's delivery alone and a preview's copy; its messages name a confirm and a land among a `pulled`'s acts, a preview among its wants, and `previews` in a `report`; and a section `Version 1` names `boomdrive.v1` and says it has no land and no preview

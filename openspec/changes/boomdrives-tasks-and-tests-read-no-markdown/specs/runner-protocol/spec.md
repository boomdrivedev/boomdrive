## MODIFIED Requirements

### Requirement: The README names the two paths and the signatures' contexts

The protocol's README SHALL describe version 2: `runner/enroll` and `runner/connect` under the control plane's address, the kinds each takes, the subprotocol `boomdrive.v2`, the `BoomdriveHost` header, and the bytes each enrollment's and connection's signature covers, each context starting `boomdrive.v2`, with a worked example of each. It SHALL name what a `pulled` and a `report` hold, and how version 1 differs. The worked examples SHALL be a file of the hosted implementation's tests, `crates/boomdrive-hosted/tests/worked-bytes.json`, of which the README shows its own copy, kept to the file by its writers (record 0049). The hosted implementation SHALL sign exactly those bytes (records 0018 and 0037).

#### Scenario: The README's worked contexts

- **WHEN** the hosted implementation makes the enrollment's and the connection's contexts from the worked examples' fixed key, token, origin, time and nonce
- **THEN** each equals the worked examples' bytes, and each starts with the bytes of `boomdrive.v2`

#### Scenario: The README of version 2

- **WHEN** the protocol's README is read
- **THEN** its envelope names `protocol_version` 2; its paths name the subprotocol `boomdrive.v2`; its files name `preview.schema.json`; its copies name an end's `delivery`, an end's delivery alone and a preview's copy; its messages name a confirm and a land among a `pulled`'s acts, a preview among its wants, and `previews` in a `report`; and a section `Version 1` names `boomdrive.v1` and says it has no land and no preview

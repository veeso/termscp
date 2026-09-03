# Publish the termscp crate
[group('publish')]
publish_crate args="":
  cargo publish --locked --features smb {{ args }}

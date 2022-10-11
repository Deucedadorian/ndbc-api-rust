
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 64,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.64.0 (a55dd71d5 2022-09-19)".to_owned(),
                    commit_hash: Some("a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52".to_owned()),
                    commit_date: Some("2022-09-19".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            
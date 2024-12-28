use error_set::error_set;

    error_set! {
        Error={
            RegisterTracing(std::io::Error),
            MissingConfigDir,
        }|| ConfigError|| BuildCacheError;
        CacheError={
            UnknownNotebook{notebook: String},
        } || BackendError;
        BackendError={
            #[cfg(feature="tracing")]
            File(std::io::Error),
        };
        BuildCacheError= BackendError|| EndpointError;
        EndpointError={
            Zbus(std::io::Error),
        };
        ConfigError={
            Io(std::io::Error),
            Toml(std::fmt::Error),
        };
    }
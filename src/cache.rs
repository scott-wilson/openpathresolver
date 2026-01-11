use cached::Cached;

// Allowing the type complexity here, since this is the only spot this is used. Otherwise, it would
// be good to split this type up.
#[allow(clippy::type_complexity)]
static REGEX_CACHE: std::sync::LazyLock<
    std::sync::Mutex<
        cached::SizedCache<String, Result<std::sync::Arc<regex::Regex>, regex::Error>>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(cached::SizedCache::with_size(512)));

pub(crate) fn regex(pattern: &str) -> Result<std::sync::Arc<regex::Regex>, crate::Error> {
    let mut cache = REGEX_CACHE
        .lock()
        .map_err(|_| crate::Error::new("Mutex lock error"))?;

    cache
        .cache_get_or_set_with(pattern.to_string(), || {
            regex::Regex::new(pattern).map(std::sync::Arc::new)
        })
        .as_ref()
        .map(|regex| regex.clone())
        .map_err(|err| crate::Error::new(format!("Regex compile error: {err}")))
}

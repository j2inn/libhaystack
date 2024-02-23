// Copyright (C) 2020 - 2024, J2 Innovations

use std::borrow::Cow;
use std::sync::RwLock;

/// Enables display values to be injected into encoded/decoded Ref objects.
///
/// This is used to handle the case whereby pre-calculated display
/// names for Refs need to be utilized when encoding/decoding haystack data.
///
/// Please note, the return type is a `Cow<str>`. At some point we'll make `Ref` a little
/// more flexible regarding how it holds its display data.
type RefDisFactoryFunc<'a> = Box<dyn Fn(&str, Option<&str>) -> Option<Cow<'a, str>> + Send + Sync>;

/// Holds the Ref factory function used for creating Refs.
static REF_DIS_FACTORY_FUNC: RwLock<Option<RefDisFactoryFunc>> = RwLock::new(None);

/// Get a ref's display name value from any registered factory.
pub fn get_ref_dis_from_factory<'a>(val: &str, dis: Option<&'a str>) -> Option<Cow<'a, str>> {
    if let Some(func) = REF_DIS_FACTORY_FUNC.read().unwrap().as_ref() {
        func(val, dis)
    } else {
        dis.map(Cow::Borrowed)
    }
}

/// Set the factory function for getting ref display name values.
pub fn set_ref_dis_factory(factory_fn: Option<RefDisFactoryFunc<'static>>) {
    *REF_DIS_FACTORY_FUNC.write().unwrap() = factory_fn;
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, sync::Mutex};

    use super::{get_ref_dis_from_factory, set_ref_dis_factory};

    // Use a mutex to prevent unit tests from interfering with each other. One of the tests
    // sets some global data so they can overlap.
    static MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn make_ref_returns_ref() {
        let _ = MUTEX.lock();

        let dis = get_ref_dis_from_factory("a", Some("c"));

        assert_eq!(dis, Some(Cow::Borrowed("c")));
    }

    #[test]
    fn make_ref_from_factory_returns_ref() {
        let _ = MUTEX.lock();

        let factory_fn = Box::new(|_: &str, _: Option<&str>| Some(Cow::Owned(String::from("c"))));

        set_ref_dis_factory(Some(factory_fn));

        let dis = get_ref_dis_from_factory("a", Some("b"));

        assert_eq!(dis, Some(Cow::Borrowed("c")));

        set_ref_dis_factory(None);
    }
}

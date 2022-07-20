// Copyright (C) 2020 - 2022, J2 Innovations

//! Defs namespace tests

#[cfg(test)]
use super::utils::{parse_def, parse_features_def};
use libhaystack::defs::namespace::{DefDict, Namespace};
use libhaystack::dict;
use libhaystack::haystack::val::*;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref FEATURES_NS: Namespace<'static> = Namespace::make(parse_features_def());
    static ref DEFS_NS: Namespace<'static> = Namespace::make(parse_def());
}

#[test]
fn test_namespace_empty() {
    let ns = Namespace::make(Grid::default());

    assert!(ns.defs.is_empty());

    let ns = Namespace::default();

    assert!(ns.defs.is_empty());
}

#[test]
fn test_namespace_get_by_name() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.defs.is_empty());

    assert!(ns.get_by_name("ahu").is_some(), "Get Some def using a name");

    assert!(
        ns.get_by_name("missingDef").is_none(),
        "None when the def is not found"
    );
}

#[test]
fn test_namespace_get_all_by_name() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.defs.is_empty());

    assert!(
        !ns.all_matching_names(&["ahu", "site"]).is_empty(),
        "Get list of defs via the names"
    );

    assert!(
        ns.all_matching_names(&["missingDef1", "missingDef2"])
            .is_empty(),
        "Get empty list when a def cannot be found"
    );
}

#[test]
fn test_namespace_has_name() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.defs.is_empty());

    assert!(ns.has_name("ahu"), "True if the def exists");

    assert!(!ns.has_name("undefined"), "False if the def missing");
}

#[test]
fn test_namespace_has() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.defs.is_empty());

    assert!(ns.has(&Symbol::from("ahu")), "True if the def exists");

    assert!(
        !ns.has(&Symbol::from("undefined")),
        "False if the def missing"
    );
}

#[test]
fn test_namespace_conjuncts() {
    let ns = Namespace::make(parse_features_def());
    assert!(!ns.defs.is_empty());

    assert!(!ns.conjuncts.is_empty(), "Non empty conjuncts");

    for conjunct in ns.conjuncts {
        assert!(conjunct.def_name().contains("-"))
    }
}

#[test]
fn test_namespace_is_conjunct() {
    assert!(
        Namespace::is_conjunct(&Symbol::from("hot-water")),
        "true if the name as a symbol is a conjunct"
    );

    assert!(
        !Namespace::is_conjunct(&Symbol::from("hot&water")),
        "false if the name as a symbol is not a conjunct"
    );
}

#[test]
fn test_namespace_conjuncts_defs() {
    let ns = Namespace::make(parse_features_def());

    assert!(
        !ns.conjuncts_defs(&Symbol::from("hot-water")).is_empty(),
        "returns a conjuncts defs"
    );

    assert_eq!(
        ns.conjuncts_defs(&Symbol::from("hot-water")),
        ns.all_matching_names(&["hot", "water"]),
        "returns a conjuncts defs"
    );
}

#[test]
fn test_namespace_features() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.features.is_empty(), "Has features");

    for feature in ns.features {
        assert!(feature.def_name().contains(":"))
    }
}

#[test]
fn test_namespace_is_feature() {
    assert!(
        Namespace::is_feature(&Symbol::from("feature:key")),
        "true if the name as a symbol is a feature"
    );

    assert!(
        !Namespace::is_feature(&Symbol::from("feature-key")),
        "false if the name as a symbol is not a feature"
    );
}

#[test]
fn test_namespace_libs() {
    let ns = Namespace::make(parse_features_def());

    assert!(!ns.libs.is_empty(), "Has libs");

    assert_eq!(
        ns.libs,
        ns.all_matching_names(&[
            "ext",
            "lib:alert",
            "lib:api",
            "lib:brand",
            "lib:cluster",
            "lib:conn",
            "lib:core",
            "lib:crypto",
            "lib:debug",
            "lib:demo",
            "lib:dev",
            "lib:diag",
            "lib:doc",
            "lib:email",
            "lib:energy",
            "lib:equip",
            "lib:geo",
            "lib:haystack",
            "lib:his",
            "lib:hisKit",
            "lib:host",
            "lib:http",
            "lib:hvac",
            "lib:install",
            "lib:io",
            "lib:iot",
            "lib:job",
            "lib:legacy",
            "lib:lic",
            "lib:lighting",
            "lib:lint",
            "lib:log",
            "lib:math",
            "lib:mobile",
            "lib:nav",
            "lib:ph",
            "lib:phIct",
            "lib:phIoT",
            "lib:phScience",
            "lib:pod",
            "lib:point",
            "lib:proj",
            "lib:pub",
            "lib:repl",
            "lib:rule",
            "lib:schedule",
            "lib:session",
            "lib:tariff",
            "lib:ui",
            "lib:ui2",
            "lib:user",
            "lib:viz",
            "lib:watchdog",
            "lib:weather",
            "lib:xquery",
            "sysMod",
        ])
        .into_iter()
        .cloned()
        .collect::<Vec<Dict>>()
    )
}

#[test]
fn test_namespace_subtypes_of() {
    let ns = Namespace::make(parse_features_def());

    assert_eq!(
        ns.subtypes_of(&Symbol::from("liquid")),
        &ns.all_matching_names(&["condensate", "fuelOil", "gasoline", "water"])
            .into_iter()
            .cloned()
            .collect::<Vec<Dict>>()
    );

    assert_eq!(
        ns.subtypes_of(&Symbol::from("undefined")),
        &Vec::<Dict>::default()
    );
}

#[test]
fn test_namespace_all_subtypes_of() {
    let ns = Namespace::make(parse_features_def());

    let mut all_subtype = ns.all_subtypes_of(&Symbol::from("point"));
    all_subtype.sort();

    let mut query = ns.all_matching_names(&[
        "writable-point",
        "weather-point",
        "cur-point",
        "haystackPoint",
        "connPoint",
        "his-point",
    ]);
    query.sort();

    assert_eq!(all_subtype, query);
}

#[test]
fn test_namespace_has_subtype() {
    let ns = Namespace::make(parse_features_def());

    assert!(
        ns.has_subtype(&Symbol::from("liquid")),
        "true if a def has a subtype for a symbol"
    );

    assert!(
        !ns.has_subtype(&Symbol::from("hot-water")),
        "false if a def does not have a subtype for a symbol"
    );

    assert!(
        !ns.has_subtype(&Symbol::from("undefined")),
        "false if the def does not exist"
    );
}

#[test]
fn test_namespace_supertypes_of() {
    let mut supertypes_of = FEATURES_NS.supertypes_of(&Symbol::from("site")).clone();
    supertypes_of.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&["entity", "geoPlace"])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(supertypes_of, query);
}

#[test]
fn test_namespace_all_supertypes_of() {
    let mut all_supertypes_of = FEATURES_NS.all_supertypes_of(&Symbol::from("site"));
    all_supertypes_of.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&["marker", "entity", "geoPlace"])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(all_supertypes_of, query);
}

#[test]
fn test_namespace_choices_for() {
    let ns = Namespace::make(parse_features_def());

    let mut choices_for = ns.choices_for(&Symbol::from("ductSection")).clone();
    choices_for.sort();

    let mut query = ns
        .all_matching_names(&[
            "discharge",
            "exhaust",
            "flue",
            "inlet",
            "mixed",
            "outside",
            "return",
        ])
        .into_iter()
        .cloned()
        .collect::<Vec<Dict>>();
    query.sort();

    assert_eq!(choices_for, query);

    assert!(
        ns.choices_for(&Symbol::from("exhaust")).is_empty(),
        "an empty list when there are no choices using a symbol"
    );
    assert!(
        ns.choices_for(&Symbol::from("undefined")).is_empty(),
        "an empty list when the def is invalid using a symbol"
    );
}

#[test]
fn test_namespace_choices() {
    let ns = Namespace::make(parse_features_def());

    assert_eq!(
        ns.choices
            .keys()
            .map(|k| k.value.as_str())
            .collect::<Vec<&str>>(),
        [
            "ahuRef",
            "ahuZoneDelivery",
            "association",
            "calendarRef",
            "childrenFlatten",
            "chilledWaterPlantRef",
            "chillerMechanism",
            "connTuningRef",
            "conveys",
            "coolingProcess",
            "cools",
            "dehumidifies",
            "depends",
            "ductConfig",
            "ductDeck",
            "ductSection",
            "equipFunction",
            "equipRef",
            "haystackConnRef",
            "heatingProcess",
            "heats",
            "hotWaterPlantRef",
            "humidifies",
            "is",
            "linter",
            "meterScope",
            "moves",
            "pipeSection",
            "pointFunction",
            "pointQuantity",
            "pointSubject",
            "prefUnit",
            "processUses",
            "produces",
            "quantities",
            "quantityOf",
            "regulates",
            "reheats",
            "ruleType",
            "scheduleRef",
            "siteRef",
            "spaceRef",
            "steamPlantRef",
            "stores",
            "submeterOf",
            "tagOn",
            "tags",
            "traitOn",
            "vavAirCircuit",
            "vavModulation",
            "ventilates",
            "weatherRef",
        ]
    )
}

#[test]
fn test_namespace_feature_names() {
    let ns = Namespace::make(parse_features_def());

    let mut feature_names = ns.feature_names.clone();
    feature_names.sort();

    assert_eq!(
        feature_names,
        [
            "app",
            "filetype",
            "func",
            "lib",
            "template",
            "trait",
            "traitView",
            "typeScript",
            "view",
        ]
    )
}

#[test]
fn test_namespace_tag_on_names() {
    let ns = Namespace::make(parse_features_def());

    let mut tag_on_names = ns.tag_on_names.clone();
    tag_on_names.sort();

    assert_eq!(
        tag_on_names,
        [
            "bacnetConn",
            "chiller",
            "conn",
            "connPoint",
            "connTuning",
            "controller",
            "cur-point",
            "def",
            "energyStarConn",
            "entity",
            "equip",
            "filetype",
            "filetype:pdf",
            "filetype:svg",
            "floor",
            "func",
            "geoPlace",
            "haystackConn",
            "haystackPoint",
            "his-point",
            "lib",
            "meter",
            "modbusConn",
            "motor",
            "obixConn",
            "opcConn",
            "point",
            "projMeta",
            "provBuild",
            "provImage",
            "provOverlay",
            "provPatch",
            "rule",
            "sedonaConn",
            "site",
            "snmpConn",
            "space",
            "sqlConn",
            "trait:point",
            "view",
            "weather-point",
            "weatherStation",
            "writable-point"
        ]
    )
}

#[test]
fn test_namespace_tag_on_defs() {
    let ns = Namespace::make(parse_features_def());

    let mut tag_on_defs = ns
        .tag_on_defs
        .keys()
        .map(|k| k.value.as_str())
        .collect::<Vec<&str>>();
    tag_on_defs.sort();

    assert_eq!(
        tag_on_defs,
        [
            "actorTimeout",
            "admin",
            "area",
            "baseUri",
            "connErr",
            "connLinger",
            "connPingFreq",
            "connState",
            "connStatus",
            "connTuningRef",
            "coolingCapacity",
            "cur",
            "curCalibration",
            "curConvert",
            "curErr",
            "curStatus",
            "curVal",
            "depends",
            "dis",
            "disMacro",
            "doc",
            "enum",
            "equipRef",
            "fileExt",
            "floorNum",
            "geoAddr",
            "geoCity",
            "geoCoord",
            "geoCountry",
            "geoCounty",
            "geoElevation",
            "geoPostalCode",
            "geoState",
            "geoStreet",
            "haystackConnRef",
            "haystackCur",
            "haystackHis",
            "haystackPollFreq",
            "haystackWrite",
            "haystackWriteLevel",
            "help",
            "his",
            "hisAppendNA",
            "hisCollectCov",
            "hisCollectInterval",
            "hisCollectWriteFreq",
            "hisConvert",
            "hisEnd",
            "hisErr",
            "hisMode",
            "hisSize",
            "hisStart",
            "hisStatus",
            "hisTotalized",
            "id",
            "imageSize",
            "is",
            "kind",
            "mandatory",
            "maxVal",
            "mime",
            "minVal",
            "navName",
            "notInherited",
            "of",
            "pageSize",
            "password",
            "pollTime",
            "primaryFunction",
            "ruleOn",
            "siteRef",
            "spaceRef",
            "staleTime",
            "su",
            "submeterOf",
            "tagOn",
            "traitView:point",
            "transient",
            "tz",
            "unit",
            "uri",
            "username",
            "version",
            "vfd",
            "weatherRef",
            "wikipedia",
            "writable",
            "writeConvert",
            "writeErr",
            "writeLevel",
            "writeMaxTime",
            "writeMinTime",
            "writeStatus",
            "writeVal",
            "yearBuilt"
        ]
    )
}

#[test]
fn test_namespace_inheritance() {
    let mut inheritance = FEATURES_NS.inheritance(&Symbol::from("site")).clone();
    inheritance.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&["site", "entity", "marker", "geoPlace"])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(inheritance, query);

    assert!(FEATURES_NS.inheritance(&Symbol::from("")).is_empty());
}

#[test]
fn test_namespace_associations() {
    let mut associations =
        FEATURES_NS.associations(&Symbol::from("equipRef"), &Symbol::from("tagOn"));
    associations.sort();

    let mut query = FEATURES_NS.all_matching_names(&["controller", "equip", "point"]);
    query.sort();

    assert_eq!(
        associations, query,
        "the associations for a equipRef using symbols"
    );

    assert!(
        FEATURES_NS
            .associations(&Symbol::from(""), &Symbol::from("tagOn"))
            .is_empty(),
        "empty array for an invalid parent using symbols"
    );

    let mut associations = FEATURES_NS.associations(&Symbol::from("site"), &Symbol::from("tags"));
    associations.sort();

    let mut query = FEATURES_NS.all_matching_names(&[
        "area",
        "dis",
        "geoAddr",
        "geoCity",
        "geoCoord",
        "geoCountry",
        "geoCounty",
        "geoElevation",
        "geoPostalCode",
        "geoState",
        "geoStreet",
        "id",
        "primaryFunction",
        "tz",
        "weatherRef",
        "yearBuilt",
    ]);
    query.sort();

    assert_eq!(associations, query, "associations for a site using names");
}

#[test]
fn test_namespace_tags() {
    let mut tags = FEATURES_NS.tags(&Symbol::from("site"));
    tags.sort();

    let mut query = FEATURES_NS.all_matching_names(&[
        "area",
        "dis",
        "geoAddr",
        "geoCity",
        "geoCoord",
        "geoCountry",
        "geoCounty",
        "geoElevation",
        "geoPostalCode",
        "geoState",
        "geoStreet",
        "id",
        "primaryFunction",
        "tz",
        "weatherRef",
        "yearBuilt",
    ]);
    query.sort();

    assert_eq!(tags, query, "associations for a site using names");
}

#[test]
fn test_namespace_is() {
    let is = FEATURES_NS.is(&Symbol::from("ac-elec"));

    let query = FEATURES_NS.all_matching_names(&["elec"]);

    assert_eq!(is, query, "associations for a `ac-elec`");
}

#[test]
fn test_namespace_tag_on() {
    let mut tag_on = FEATURES_NS.tag_on(&Symbol::from("equipRef"));
    tag_on.sort();

    let mut query = FEATURES_NS.all_matching_names(&["controller", "equip", "point"]);
    query.sort();

    assert_eq!(tag_on, query, "`tagOn` associations for a equipRef");
}

#[test]
fn test_namespace_reflect() {
    let subject = dict! {
        "id" => Value::make_ref("hwp"),
        "dis" =>  Value::make_str("Hot Water Plant"),
        "hot" =>  Value::Marker,
        "water" =>  Value::Marker,
        "plant" =>  Value::Marker,
        "equip" =>  Value::Marker
    };

    let reflect = FEATURES_NS.reflect(&subject);
    let mut defs = reflect.defs.to_vec();
    defs.sort();

    assert!(reflect.fits(&Symbol::from("equip")));

    let mut query = FEATURES_NS
        .all_matching_names(&[
            "id",
            "ref",
            "scalar",
            "val",
            "dis",
            "str",
            "hot",
            "marker",
            "water",
            "liquid",
            "fluid",
            "substance",
            "phenomenon",
            "plant",
            "equip",
            "entity",
            "hot-water",
            "hot-water-plant",
        ])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(defs, query);

    let subject = dict! {
    "chilled" =>  Value::Marker,
    "water" =>  Value::Marker
    };

    let reflect = FEATURES_NS.reflect(&subject);
    let mut defs = reflect.defs.to_vec();
    defs.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&[
            "chilled",
            "marker",
            "water",
            "liquid",
            "fluid",
            "substance",
            "phenomenon",
            "chilled-water",
        ])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(defs, query);

    let subject = dict! {
    "weather" =>  Value::Marker,
    "writable" =>  Value::Marker,
    "cur" =>  Value::Marker,
    "point" =>  Value::Marker,
    "his" =>  Value::Marker,
    "curVal" =>  Value::Marker,
    "sensor" =>  Value::Marker,
    "temp" =>  Value::Marker,
    "discharge" =>  Value::Marker,
    "air" =>  Value::Marker
    };

    let reflect = FEATURES_NS.reflect(&subject);

    let names = reflect
        .defs
        .iter()
        .map(|def| def.def_name().as_str())
        .collect::<Vec<&str>>();

    assert!([
        "point",
        "cur-point",
        "his-point",
        "weather-point",
        "writable-point",
    ]
    .iter()
    .all(|def| names.contains(def)));

    assert_eq!(&reflect.subject, &subject);

    let reflect = FEATURES_NS.reflect(&dict! {
        "equip" => Value::Marker,
        "ahu" => Value::Marker
    });
    assert_eq!(
        &reflect.entity_type.def_name(),
        &"ahu",
        "returns the ahu tag when there is an equip and ahu tag"
    );

    let reflect = FEATURES_NS.reflect(&dict! {
        "ahu" => Value::Marker,
        "equip" => Value::Marker,
    });
    assert_eq!(
        &reflect.entity_type.def_name(),
        &"ahu",
        "returns the ahu tag when there is an ahu and equip tag"
    );

    let reflect = FEATURES_NS.reflect(&dict! {
        "heatExchanger" => Value::Marker,
        "coil" => Value::Marker,
        "coolingCoil" => Value::Marker,
        "equip" => Value::Marker,
    });
    assert_eq!(
        &reflect.entity_type.def_name(),
        &"coolingCoil",
        "returns the coolingCoil tag from an entity with all sub type marker tags"
    );
}

#[test]
fn test_namespace_fits() {
    assert!(FEATURES_NS.fits(&Symbol::from("site"), &Symbol::from("entity")));

    assert!(FEATURES_NS.fits(&Symbol::from("site"), &Symbol::from("marker")));

    assert!(FEATURES_NS.fits(&Symbol::from("air"), &Symbol::from("marker")));

    assert!(!FEATURES_NS.fits(&Symbol::from("fake"), &Symbol::from("marker")));
    assert!(!FEATURES_NS.fits(&Symbol::from("site"), &Symbol::from("fake")));
}

#[test]
fn test_namespace_fits_maker() {
    assert!(FEATURES_NS.fits_marker(&Symbol::from("site")));
    assert!(FEATURES_NS.fits_marker(&Symbol::from("water")));

    assert!(!FEATURES_NS.fits_marker(&Symbol::from("fake")));
}

#[test]
fn test_namespace_fits_val() {
    assert!(FEATURES_NS.fits_val(&Symbol::from("def")));
    assert!(!FEATURES_NS.fits_val(&Symbol::from("site")));

    assert!(!FEATURES_NS.fits_val(&Symbol::from("equip")));
}

#[test]
fn test_namespace_fits_choice() {
    assert!(FEATURES_NS.fits_choice(&Symbol::from("equipFunction")));
    assert!(!FEATURES_NS.fits_choice(&Symbol::from("site")));

    assert!(!FEATURES_NS.fits_choice(&Symbol::from("equip")));
}

#[test]
fn test_namespace_fits_entity() {
    assert!(FEATURES_NS.fits_entity(&Symbol::from("site")));
    assert!(FEATURES_NS.fits_entity(&Symbol::from("equip")));
    assert!(!FEATURES_NS.fits_entity(&Symbol::from("equipFunction")));
}

#[test]
fn test_namespace_fits_implementation() {
    let mut implementation = FEATURES_NS.implementation(&Symbol::from("tank"));
    implementation.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&["tank", "equip"])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(implementation, query);

    let mut implementation = FEATURES_NS.implementation(&Symbol::from("hot-water"));
    implementation.sort();

    let mut query = FEATURES_NS
        .all_matching_names(&["hot", "water"])
        .into_iter()
        .collect::<Vec<&Dict>>();
    query.sort();

    assert_eq!(implementation, query);

    assert!(FEATURES_NS
        .implementation(&Symbol::from("super-ultra-duper"))
        .is_empty())
}

#[test]
fn test_namespace_fits_protos() {
    let parent = dict! {
        "pipe" =>  Value::Marker,
        "equip" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{pump motor equip},",
        "{valve actuator equip},",
        "{flow point},",
        "{pressure point},",
        "{temp point},",
        "{equip},",
        "{point}]"
    ));

    assert_eq!(protos, expect);

    let parent = dict! {
        "steam" =>  Value::Marker,
        "leaving" =>  Value::Marker,
        "pipe" =>  Value::Marker,
        "equip" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{steam leaving pump motor equip},",
        "{steam leaving valve actuator equip},",
        "{steam leaving flow point},",
        "{steam leaving pressure point},",
        "{steam leaving temp point},",
        "{equip},",
        "{point}]",
    ));

    assert_eq!(protos, expect);

    let parent = dict! {
        "leaving" =>  Value::Marker,
        "naturalGas" =>  Value::Marker,
        "pipe" =>  Value::Marker,
        "equip" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{naturalGas leaving pump motor equip},",
        "{naturalGas leaving valve actuator equip},",
        "{naturalGas leaving flow point},",
        "{naturalGas leaving pressure point},",
        "{naturalGas leaving temp point},",
        "{equip},",
        "{point}]",
    ));

    assert_eq!(protos, expect);

    let parent = dict! {
        "ahu" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{equip thermostat},",
        "{discharge duct equip},",
        "{duct equip exhaust},",
        "{duct equip mixed},",
        "{duct equip outside},",
        "{duct equip return},",
        "{freezeStat point sensor},",
        "{cmd heatWheel point},",
        "{cmd faceBypass point},",
        "{bypass cmd damper point}]",
    ));

    assert_eq!(protos, expect);

    let parent = dict! {
        "chiller" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{point run state},",
        "{enable point state},",
        "{cmd load point},",
        "{load point sensor},",
        "{efficiency point sensor},",
        "{chilled equip leaving pipe water},",
        "{chilled entering equip pipe water},",
        "{chilled delta point sensor temp water},",
        "{chilled delta flow point sensor water},",
        "{chilled cmd isolation point valve water},",
        "{condenser equip leaving pipe water},",
        "{condenser entering equip pipe water},",
        "{cmd condenser isolation point valve water},",
        "{condenser point run state},",
        "{condenser point refrig sensor temp},",
        "{condenser point pressure refrig sensor},",
        "{evaporator point refrig sensor temp},",
        "{evaporator point pressure refrig sensor}]",
    ));

    assert_eq!(protos, expect);

    let parent = dict! {
        "ahu" =>  Value::Marker,
        "chiller" =>  Value::Marker
    };

    let mut protos = FEATURES_NS.protos(&parent);
    protos.sort();

    let expect = parse_dict_list(concat!(
        "[{equip thermostat},",
        "{discharge duct equip},",
        "{duct equip exhaust},",
        "{duct equip mixed},",
        "{duct equip outside},",
        "{duct equip return},",
        "{freezeStat point sensor},",
        "{cmd heatWheel point},",
        "{cmd faceBypass point},",
        "{bypass cmd damper point},",
        "{point run state},",
        "{enable point state},",
        "{cmd load point},",
        "{load point sensor},",
        "{efficiency point sensor},",
        "{chilled equip leaving pipe water},",
        "{chilled entering equip pipe water},",
        "{chilled delta point sensor temp water},",
        "{chilled delta flow point sensor water},",
        "{chilled cmd isolation point valve water},",
        "{condenser equip leaving pipe water},",
        "{condenser entering equip pipe water},",
        "{cmd condenser isolation point valve water},",
        "{condenser point run state},",
        "{condenser point refrig sensor temp},",
        "{condenser point pressure refrig sensor},",
        "{evaporator point refrig sensor temp},",
        "{evaporator point pressure refrig sensor}]",
    ));

    assert_eq!(protos, expect);

    assert!(FEATURES_NS.protos(&Dict::default()).is_empty());
}

fn parse_dict_list(zinc_str: &str) -> Vec<Dict> {
    let mut expect = match libhaystack::encoding::zinc::decode::from_str(zinc_str).expect("Value") {
        Value::List(list) => list
            .into_iter()
            .filter_map(|val| match val {
                Value::Dict(dict) => Some(dict),
                _ => None,
            })
            .collect::<Vec<Dict>>(),
        _ => panic!("Must be a list"),
    };
    expect.sort();
    expect
}

#[test]
fn test_namespace_relationship() {
    let resolve = |_: &Ref| None;

    let subject = dict! {
        "ahu" =>  Value::Marker,
        "hotWaterRef" =>  Value::make_ref("ahu")
    };

    // true when a record has a `hotWaterRef`
    let has = DEFS_NS.has_relationship(&subject, &Symbol::from("inputs"), &None, &None, &resolve);
    assert!(has);

    // true when a record has a `hotWaterRef` and inputs hot water
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("hot-water")),
        &None,
        &resolve,
    );
    assert!(has);

    // true when a record has a `hotWaterRef` and inputs liquid
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("liquid")),
        &None,
        &resolve,
    );
    assert!(has);

    // true when a record has a `hotWaterRef` and inputs water
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("water")),
        &None,
        &resolve,
    );
    assert!(has);

    // false when a record has a `hotWaterRef` and inputs air
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("air")),
        &None,
        &resolve,
    );
    assert!(!has);

    // true when a record has a `hotWaterRef`, inputs hot water and matches the target value
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("hot-water")),
        &Some(Ref::from("ahu")),
        &resolve,
    );
    assert!(has);

    // false when a record has a `hotWaterRef`, inputs hot water and does not match the target value
    let has = DEFS_NS.has_relationship(
        &subject,
        &Symbol::from("inputs"),
        &Some(Symbol::from("hot-water")),
        &Some(Ref::from("vav")),
        &resolve,
    );
    assert!(!has);

    // false when a record does not have any inputs
    let subject = dict! {
        "ahu" =>  Value::Marker
    };
    let has = DEFS_NS.has_relationship(&subject, &Symbol::from("inputs"), &None, &None, &resolve);
    assert!(!has);
}

#[test]
fn test_namespace_transitive_relationship() {
    let mut map = HashMap::<&str, &Dict>::new();

    let ahu = dict! {
        "id" =>  Value::make_ref("ahu"),
        "ahu" =>  Value::Marker,
        "equip" =>  Value::Marker,
    };
    map.insert("ahu", &ahu);

    let fan = dict! {
        "id" =>  Value::make_ref("fan"),
        "discharge" =>  Value::Marker,
        "fan" =>  Value::Marker,
        "equip" =>  Value::Marker,
        "equipRef" =>  Value::make_ref("ahu"),
    };
    map.insert("fan", &fan);

    let status = dict! {
        "id" =>  Value::make_ref("status"),
        "speed" =>  Value::Marker,
        "cmd" =>  Value::Marker,
        "point" =>  Value::Marker,
        "equipRef" =>  Value::make_ref("fan"),
    };
    map.insert("status", &status);

    let resolve = |id: &Ref| map.get(id.value.as_str()).map(|d| (*d).clone());

    // true for a fan that directly references an ahu
    let has = FEATURES_NS.has_relationship(
        &fan,
        &Symbol::from("containedBy"),
        &None,
        &Some(Ref::from("ahu")),
        &resolve,
    );
    assert!(has);

    // true for a point that directly references a fan
    let has = FEATURES_NS.has_relationship(
        &status,
        &Symbol::from("containedBy"),
        &None,
        &Some(Ref::from("fan")),
        &resolve,
    );
    assert!(has);

    // true for a point that indirectly references an ahu
    let has = FEATURES_NS.has_relationship(
        &status,
        &Symbol::from("containedBy"),
        &None,
        &Some(Ref::from("ahu")),
        &resolve,
    );
    assert!(has);

    // false for a fan that does not reference a point
    let has = FEATURES_NS.has_relationship(
        &fan,
        &Symbol::from("containedBy"),
        &None,
        &Some(Ref::from("status")),
        &resolve,
    );
    assert!(!has);

    //  false for a point that references itself
    let has = FEATURES_NS.has_relationship(
        &status,
        &Symbol::from("containedBy"),
        &None,
        &Some(Ref::from("status")),
        &resolve,
    );
    assert!(!has);
}

#[test]
fn test_namespace_reciprocal_relationship() {
    let mut map = HashMap::<&str, &Dict>::new();

    let hwp = dict! {
        "id" =>  Value::make_ref("hwp"),
        "hot" =>  Value::Marker,
        "water" =>  Value::Marker,
        "plant" =>  Value::Marker,
        "equip" =>  Value::Marker,
    };
    map.insert("hwp", &hwp);

    // AHU entity which inputs hot water from the plant above
    let ahu = dict! {
        "id" =>  Value::make_ref("ahu"),
        "ahu" =>  Value::Marker,
        "hotWaterHeating" =>  Value::Marker,
        "equip" =>  Value::Marker,
        "hotWaterRef" =>  Value::make_ref("hwp"),
    };
    map.insert("ahu", &ahu);

    let resolve = |id: &Ref| map.get(id.value.as_str()).map(|d| (*d).clone());

    // true when `ahu` inputs hot water from `hwp`
    let has = DEFS_NS.has_relationship(
        &ahu,
        &Symbol::from("inputs"),
        &Some(Symbol::from("hot-water")),
        &Some(Ref::from("hwp")),
        &resolve,
    );
    assert!(has);

    // return true when AHU outputs hot water
    let has = DEFS_NS.has_relationship(
        &ahu,
        &Symbol::from("outputs"),
        &Some(Symbol::from("hot-water")),
        &Some(Ref::from("ahu")),
        &resolve,
    );
    assert!(has);

    // returns false when HWP outputs hot water
    let has = DEFS_NS.has_relationship(
        &hwp,
        &Symbol::from("outputs"),
        &Some(Symbol::from("hot-water")),
        &Some(Ref::from("ahu")),
        &resolve,
    );
    assert!(!has);
}

#[test]
fn test_namespace_containment_relationship() {
    let ahu = dict! {
        "id" =>  Value::make_ref("ahu"),
        "ahu" =>  Value::Marker,
        "equip" =>  Value::Marker,
        "siteRef" =>  Value::make_ref("site"),
    };

    let fan = dict! {
        "id" =>  Value::make_ref("fan"),
        "discharge" =>  Value::Marker,
        "fan" =>  Value::Marker,
        "equip" =>  Value::Marker,
        "siteRef" =>  Value::make_ref("site"),
        "equipRef" =>  Value::make_ref("ahu"),
    };

    let contained_by_refs = |ahu: Dict| {
        let reflect = &FEATURES_NS.reflect(&ahu);
        reflect
            .defs
            .iter()
            .filter_map(|def| {
                if def.has("containedBy") {
                    Some(def.def_name().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    };

    assert!(
        contained_by_refs(ahu).contains(&&"siteRef".to_string()),
        "ahu must be contained by a site"
    );

    let refs = contained_by_refs(fan);
    assert!(
        refs.contains(&&"siteRef".to_string()) && refs.contains(&&"equipRef".to_string()),
        "fan must be contained by equip and site"
    )
}

// Copyright (C) 2020 - 2022, J2 Innovations

//! Test Grid

#[cfg(test)]
use libhaystack::dict;
use libhaystack::encoding::zinc::decode::*;
use libhaystack::encoding::zinc::encode::*;
use libhaystack::val::*;

#[test]
fn test_zinc_grid_empty_encode() {
    let value = Value::make_grid(Grid::default());
    let string = concat!(r#"ver:"3.0""#, "\n", r#"empty"#, "\n\n");

    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), string);
}

#[test]
fn test_zinc_grid_empty_decode() {
    let string = concat!(r#"ver:"3.0""#, "\n", r#"empty"#, "\n");

    let value = from_str(&string);
    assert_eq!(value.unwrap(), Value::make_grid(Grid::make_empty()));
}

#[test]
fn test_zinc_grid_version_decode() {
    let string = concat!(r#"ver:"3.0""#, "\n", r#"empty"#, "\n");

    let value = from_str(&string).expect("Grid");

    assert!(matches!(value, Value::Grid(grid) if grid.ver == GRID_FORMAT_VERSION));

    let string = concat!(r#"ver:"2.0""#, "\n", r#"empty"#, "\n");

    let value = from_str(&string).expect("Grid");

    assert!(matches!(value, Value::Grid(grid) if grid.ver == "2.0"));
}

#[test]
fn test_zinc_grid_encode() {
    let recs = vec![
        dict! {
            "site" => Value::make_marker(),
            "dis" => Value::make_str("Site")
        },
        dict! {
            "equip" => Value::make_marker(),
            "number" => Value::make_int(100),
            "navName" => Value::make_str("Equip")
        },
    ];
    let value = Value::make_grid(Grid::make_from_dicts(recs));

    let string = concat!(
        r#"ver:"3.0""#,
        "\n",
        r#"dis,equip,navName,number,site"#,
        "\n",
        r#""Site",,,,M"#,
        "\n",
        r#",M,"Equip",100,"#,
        "\n\n"
    );

    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), string);
}

#[test]
fn test_zinc_grid_inner_encode() {
    let recs = vec![dict! {
        "grid" => Value::make_grid_from_dicts(vec![dict! {"inner" => Value::make_marker()}])
    }];
    let value = Value::make_grid(Grid::make_from_dicts(recs));

    let string = concat!(
        r#"ver:"3.0""#,
        "\n",
        "grid",
        "\n",
        "<<",
        "\n",
        r#"ver:"3.0""#,
        "\n",
        "inner",
        "\n",
        "M\n>>",
        "\n\n"
    );

    let zinc = to_zinc_string(&value);
    assert_eq!(zinc.unwrap(), string);
}

#[test]
fn test_zinc_grid_decode() {
    let string = concat!(
        r#"ver:"3.0""#,
        "\n",
        r#"dis,equip,navName,number,site"#,
        "\n",
        r#""Site",,,,M"#,
        "\n",
        r#", M,"Equip", 100 ,"#,
        "\n"
    );
    let value: Value = from_str(string).unwrap();
    let recs = vec![
        dict! {
            "site" => Value::make_marker(),
            "dis" => Value::make_str("Site")
        },
        dict! {
            "equip" => Value::make_marker(),
            "number" => Value::make_int(100),
            "navName" => Value::make_str("Equip")
        },
    ];
    let grid = Value::make_grid(Grid::make_from_dicts(recs));

    assert_eq!(value, grid);
}

#[test]
fn test_zinc_grid_meta_decode() {
    let string = concat!(
        r#"ver:"3.0" "#,
        "test foo:100 d:{a,b,c}",
        "\n",
        r#"dis,equip,navName,number,site"#,
        "\n",
        r#""Site",,,,M"#,
        "\n",
        r#", M, "Equip", 100 ,"#,
        "\n"
    );
    let value = from_str(string);

    assert!(value.is_ok());

    let grid = Grid::try_from(&value.unwrap()).expect("Grid");

    assert_eq!(
        grid.meta,
        Some(
            dict! {"test" => Value::Marker, "foo" => Value::make_int(100), "d" => Value::make_dict(dict!{
                "a" => Value::Marker,
                "b" => Value::Marker,
                "c" => Value::Marker
            })}
        )
    )
}

#[test]
fn test_zinc_grid_bug_decode() {
    let string = concat!(r#"ver:"3.0""#,
    "\n",
    "id,actions,air,alarmPoint,analytics,apparent,avgDamper,avgDeltaAirflow,avgDeltaTemp,avgRoomTemp,chilled,cmd,condensed,cool,cooling,cur,curStatus,curTracksWrite,curVal,current,currentAverage,currentTotal,damper,delta,differential,dis,disMacro,discharge,effective,elecCost,enable,energy,energyCost,entering,enum,equipRef,exhaust,fan,filter,floorRef,flow,freq,heat,heating,high,his,hisCollectInterval,hisEnd,hisFunc,hisSize,hisStart,hisStatus,hot,humidity,kind,leaving,limit,low,maxDamper,maxDeltaAirflow,maxDeltaTemp,maxRoomTemp,maxVal,measuredAt,minDamper,minDeltaTemp,minRoomTemp,minVal,mixed,navName,nextTime,nextVal,noDemoMode,occ,occupied,outside,pf,phase,point,power,powerFactorTotal,powerTotal,precision,pressure,pump,reactive,reheat,reset,return,rollUp,runStarts,runTime,schedulable,schedule,scheduleRef,sensor,siteRef,sp,stage,status,sunrise,supply,temp,totalAirflow,totalAirflowSp,totalEquip,totalOccupied,tz,unit,unocc,valve,volt,voltAverage,water,weatherCond,weatherPoint,weatherRef,weatherSyncId,writable,write8,writeDef,writeLevel,writeStatus,writeVal,zone,mod",
    "\n",
    r#"@p:demo:r:1eeb11ef-fa6b895d "JB Tower Vav-01 UnOccCool","ver:\"2.0\"\ndis,expr,hvac_finCat\n\"Emergency Set\",\"pointEmergencyOverride(\\\$self, \\\$val)\",9\n\"Emergency Auto\",\"pointEmergencyAuto(\\\$self)\",9\n\"Manual Set\",\"pointOverride(\\\$self, \\\$val, \\\$duration)\",6\n\"Manual Auto\",\"pointAuto(\\\$self)\",6\n\"Set Default\",\"pointSetDef(\\\$self, \\\$val)\",3\n\"Set Null\",\"pointSetDef(\\\$self, null)\",3\n\n",M,,,,,,,,,,,,M,M,"ok",M,64°F,,,,,,,,"\$equipRef \$navName",,,,,,,,,@p:demo:r:1eeb11ef-c0d7f1fa "JB Tower Vav-01",,,,@p:demo:r:1eeb11ef-1ccc23d5 "Floor 1",,,,,,M,,,"hisDamper",,,,,,"Number",,,,,,,,95,,,,,33,,"UnOccCool",,,,,,,,,M,,,,1,,,,,,,,,,,,,,@p:demo:r:1eeaf021-825d1e56 "JB Tower",M,,,,,M,,,,,"Los_Angeles","°F",M,,,,,,,,,M,,,17,"unbound",,M,2018-08-04T00:59:02.62Z"#,
    "\n");
    let value = from_str(string).expect("Grid");

    assert!(!Grid::try_from(&value).expect("Grid").is_empty());

    let string = concat!(r#"ver:"3.0""#,
        "\n",
        "id,actions,air,alarmPoint,analytics,apparent,avgDamper,avgDeltaAirflow,avgDeltaTemp,avgRoomTemp,chilled,cmd,condensed,cool,cooling,cur,curStatus,curTracksWrite,curVal,current,currentAverage,currentTotal,damper,delta,differential,dis,disMacro,discharge,effective,elecCost,enable,energy,energyCost,entering,enum,equipRef,exhaust,fan,filter,floorRef,flow,freq,heat,heating,high,his,hisCollectInterval,hisEnd,hisFunc,hisSize,hisStart,hisStatus,hot,humidity,kind,leaving,limit,low,maxDamper,maxDeltaAirflow,maxDeltaTemp,maxRoomTemp,maxVal,measuredAt,minDamper,minDeltaTemp,minRoomTemp,minVal,mixed,navName,nextTime,nextVal,noDemoMode,occ,occupied,outside,pf,phase,point,power,powerFactorTotal,powerTotal,precision,pressure,pump,reactive,reheat,reset,return,rollUp,runStarts,runTime,schedulable,schedule,scheduleRef,sensor,siteRef,sp,stage,status,sunrise,supply,temp,totalAirflow,totalAirflowSp,totalEquip,totalOccupied,tz,unit,unocc,valve,volt,voltAverage,water,weatherCond,weatherPoint,weatherRef,weatherSyncId,writable,write8,writeDef,writeLevel,writeStatus,writeVal,zone,mod",
        "\n",
        r#",,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,"#,
        "\n");
    let value = from_str(string).expect("Grid");
    assert!(!Grid::try_from(&value).expect("Grid").is_empty());
}

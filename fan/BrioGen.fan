//
// Copyright (c) 2026, J2 Innovations
// All Rights Reserved
//
// History:
//    25 Feb 26  Gareth Johnson  Creation
//

using haystack

**
** In order to test Brio (binary encoding/decoding for haystack data), we ideally need 
** to generate byte dumps of all the various haystack data types from Haxall (https://haxall.io). 
** Running this Fantom code will then generate an output that can be used in 
** src/haystack/encoding/brio/haxall_features.rs for further testing.
**
class BrioGen {

  Buf encode(Obj? val) {
    buf := Buf()
    BrioWriter(buf.out).writeVal(val)
    return buf.flip
  }

  Void p(Str name, Obj? val) {
    echo("${name}|${encode(val).toHex}")
  }

  static Void main() {
    g := BrioGen()
    // scalars
    g.p("null",   null)
    g.p("marker", Marker.val)
    g.p("na",     NA.val)
    g.p("remove", Remove.val)
    g.p("false",  false)
    g.p("true",   true)

    // numbers - I2 range (-32767..32767)
    g.p("n_zero",    Number(0f))
    g.p("n_i2_42",   Number(42f))
    g.p("n_i2_neg",  Number(-1f))
    g.p("n_i2_max",  Number(32767f))
    g.p("n_i2_min",  Number(-32767f))

    // numbers - I4 range
    g.p("n_i4_pos",    Number(32768f))
    g.p("n_i4_neg",    Number(-32768f))
    g.p("n_i4_max",    Number(2147483647f))
    g.p("n_i4_min",    Number(-2147483648f))

    // numbers - F8 (float / out of i4 range)
    g.p("n_f8_pi",     Number(3.141592653589793f))
    g.p("n_f8_big",    Number(2147483648f))
    g.p("n_f8_bigneg", Number(-2147483649f))

    // numbers with const-table units
    g.p("n_degF", Number(98.6f,  Unit("\u00b0F")))
    g.p("n_kW",   Number(1500f,  Unit("kW")))
    g.p("n_kWh",  Number(99f,    Unit("kWh")))
    g.p("n_degC", Number(22f,    Unit("\u00b0C")))
    g.p("n_pct",  Number(75f,    Unit("%")))
    g.p("n_cfm",  Number(400f,   Unit("cfm")))

    // strings
    g.p("str_empty",   "")
    g.p("str_hello",   "hello")
    g.p("str_ny",      "New_York")
    g.p("str_siteRef", "siteRef")
    g.p("str_dis",     "dis")
    g.p("str_cafe",    "caf\u00e9")
    g.p("str_degF",    "temp \u00b0F")

    // URIs
    g.p("uri_http",  `http://example.com/`)
    g.p("uri_path",  `a/b/c`)

    // Refs - I8 form (standard 8-byte XXXXXXXX-YYYYYYYY)
    g.p("ref_i8_nodis",  Ref("1deb31b8-7508b187", null))
    g.p("ref_i8_dis",    Ref("1deb31b8-7508b187", "hi!"))
    g.p("ref_i8_dis2",   Ref("cafebabe-deadbeef", "Site \u0394"))

    // Refs - STR form (non-standard id)
    g.p("ref_str_nodis", Ref("1debX1b8-7508b187", null))
    g.p("ref_str_dis",   Ref("custom.ref", "My Equip"))

    // Dates
    g.p("date_2015", Date(2015, Month.nov, 30))
    g.p("date_2000", Date(2000, Month.jan, 1))
    g.p("date_1970", Date(1970, Month.jan, 1))
    g.p("date_1950", Date(1950, Month.jun, 7))
    g.p("date_2099", Date(2099, Month.dec, 31))

    // Times
    g.p("time_midnight", Time(0,  0,  0,  0))
    g.p("time_noon",     Time(12, 0,  0,  0))
    g.p("time_hms",      Time(15, 6,  13, 0))
    g.p("time_ms",       Time(15, 6,  13, 123000000))

    // DateTimes - I4 (no sub-second), const tz
    g.p("dt_i4_ny",     DateTime.fromStr("2015-11-30T12:03:57-05:00 New_York"))
    g.p("dt_i4_utc",    DateTime.fromStr("2021-06-15T12:00:00Z UTC"))
    g.p("dt_i4_pre2k",  DateTime.fromStr("1999-06-07T01:02:00-04:00 New_York"))
    g.p("dt_i4_pre70",  DateTime.fromStr("1950-06-07T01:02:00-04:00 New_York"))

    // DateTimes - I4, non-const tz
    g.p("dt_i4_warsaw", DateTime.fromStr("2000-01-01T00:00:00+01:00 Warsaw"))

    // DateTimes - I8 (with sub-second), const tz
    g.p("dt_i8_ny_ms",  DateTime.fromStr("2015-11-30T12:02:33.378-05:00 New_York"))
    g.p("dt_i8_ny_us",  DateTime.fromStr("2015-11-30T12:03:57.000123-05:00 New_York"))
    g.p("dt_i8_pre70",  DateTime.fromStr("1950-06-07T01:02:00.123-04:00 New_York"))

    // DateTimes - I8, non-const tz
    g.p("dt_i8_warsaw", DateTime.fromStr("2000-01-01T00:00:00.832+01:00 Warsaw"))

    // Coords
    g.p("coord_pos",  Coord(37.54f,   77.43f))
    g.p("coord_neg",  Coord(-17.535f, -149.569f))
    g.p("coord_zero", Coord(0f, 0f))

    // Symbols
    g.p("sym_const",  Symbol("coolingTower"))
    g.p("sym_inline", Symbol("foo-bar"))
    g.p("sym_site",   Symbol("site"))

    // XStr
    g.p("xstr_foo", XStr("Foo", "bar"))

    // Dicts
    g.p("dict_empty", Etc.makeDict(Str:Obj?[:]))
    g.p("dict_dis",   Etc.makeDict(["dis": "Hello"]))
    g.p("dict_site",  Etc.makeDict(["dis": "Site", "site": Marker.val]))
    g.p("dict_num",   Etc.makeDict(["val": Number(123f, Unit("kW"))]))
    
    // Lists
    g.p("list_empty",  Obj?[,])
    g.p("list_marker", Obj?[Marker.val])
    g.p("list_mixed",  Obj?["hello", Number(42f), Marker.val])
  }
}

// Copyright (C) 2020 - 2022, J2 Innovations

//! Brio string constant table.
//!
//! The Brio encoding avoids repeating common strings by encoding them as a
//! compact varint index into this shared constant table.  The table is taken
//! directly from the canonical `brio-consts.txt` shipped with the Haxall
//! project (version 2.1.13).
//!
//! Index 0 is the empty string `""` — Haxall encodes it as `varint(0)` on the
//! wire (confirmed by `BrioTest.fan`: `verifyConsts(cp, "", 0)`).  All other
//! entries are 1-based.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Seconds between the Unix epoch (1970-01-01) and the Fantom epoch (2000-01-01).
pub const FANTOM_EPOCH_UNIX_SECS: i64 = 946_684_800;

/// Canonical string constants, indexed from 0.  Index 0 is the empty string.
/// Haxall encodes `""` as `varint(0)` — confirmed by `BrioTest.fan`:
///   `verifyConsts(cp, "", 0)`
pub static CONSTS: &[&str] = &[
    "", // 0
    // kinds
    "Obj",      // 1
    "Bin",      // 2
    "Bool",     // 3
    "Coord",    // 4
    "Date",     // 5
    "DateTime", // 6
    "Dict",     // 7
    "Grid",     // 8
    "List",     // 9
    "Marker",   // 10
    "NA",       // 11
    "Number",   // 12
    "Ref",      // 13
    "Remove",   // 14
    "Span",     // 15
    "Str",      // 16
    "Time",     // 17
    "Uri",      // 18
    "XStr",     // 19
    "Obj[]",    // 20
    "Str[]",    // 21
    "Ref[]",    // 22
    "Dict[]",   // 23
    // timezones
    "UTC",         // 24
    "Rel",         // 25
    "New_York",    // 26
    "Chicago",     // 27
    "Denver",      // 28
    "Los_Angeles", // 29
    "Phoenix",     // 30
    "Anchorage",   // 31
    "Honolulu",    // 32
    "Halifax",     // 33
    "Winnipeg",    // 34
    "Toronto",     // 35
    "Montreal",    // 36
    "Regina",      // 37
    "Vancouver",   // 38
    "Mexico_City", // 39
    "Hong_Kong",   // 40
    "Shanghai",    // 41
    "Seoul",       // 42
    "Singapore",   // 43
    "Tokyo",       // 44
    "Kolkata",     // 45
    "Dubai",       // 46
    "Jerusalem",   // 47
    "Sydney",      // 48
    "Melbourne",   // 49
    "Amsterdam",   // 50
    "Berlin",      // 51
    "Brussels",    // 52
    "Copenhagen",  // 53
    "Dublin",      // 54
    "Istanbul",    // 55
    "Lisbon",      // 56
    "London",      // 57
    "Madrid",      // 58
    "Moscow",      // 59
    "Paris",       // 60
    "Stockholm",   // 61
    "Vienna",      // 62
    "Zurich",      // 63
    // str values
    "unknown",            // 64
    "ok",                 // 65
    "down",               // 66
    "fault",              // 67
    "disabled",           // 68
    "stale",              // 69
    "remoteFault",        // 70
    "remoteDown",         // 71
    "remoteDisabled",     // 72
    "remoteUnknown",      // 73
    "pending",            // 74
    "syncing",            // 75
    "unbound",            // 76
    "$siteRef $navName",  // 77
    "$equipRef $navName", // 78
    // mime types
    "text/plain",                     // 79
    "text/javascript",                // 80
    "text/html",                      // 81
    "text/trio",                      // 82
    "text/zinc",                      // 83
    "text/plain; charset=utf-8",      // 84
    "text/javascript; charset=utf-8", // 85
    "text/html; charset=utf-8",       // 86
    "text/trio; charset=utf-8",       // 87
    "text/zinc; charset=utf-8",       // 88
    "image/gif",                      // 89
    "image/jpeg",                     // 90
    "image/png",                      // 91
    "image/svg",                      // 92
    "application/json",               // 93
    "application/octet-stream",       // 94
    "application/pdf",                // 95
    "application/x-dicts",            // 96
    "application/x-his",              // 97
    // tags
    "absorption",           // 98
    "actions",              // 99
    "ahu",                  // 100
    "ahuRef",               // 101
    "air",                  // 102
    "airCooled",            // 103
    "alarm",                // 104
    "analytics",            // 105
    "app",                  // 106
    "appFormOn",            // 107
    "area",                 // 108
    "axAnnotated",          // 109
    "axId",                 // 110
    "axSlotPath",           // 111
    "axType",               // 112
    "bacnetConn",           // 113
    "bacnetConnRef",        // 114
    "bacnetCur",            // 115
    "bacnetHis",            // 116
    "bacnetWrite",          // 117
    "bacnetWriteLevel",     // 118
    "blowdown",             // 119
    "boiler",               // 120
    "boilerPlant",          // 121
    "boilerPlantRef",       // 122
    "building",             // 123
    "buildingRef",          // 124
    "bypass",               // 125
    "calendar",             // 126
    "calendarRef",          // 127
    "campusRef",            // 128
    "centrifugal",          // 129
    "chartOn",              // 130
    "chilled",              // 131
    "chilledBeamZone",      // 132
    "chilledWaterCool",     // 133
    "chilledWaterPlant",    // 134
    "chiller",              // 135
    "chillerWaterPlantRef", // 136
    "circuit",              // 137
    "closedLoop",           // 138
    "cmd",                  // 139
    "co",                   // 140
    "co2",                  // 141
    "coldDeck",             // 142
    "color",                // 143
    "condensate",           // 144
    "condenser",            // 145
    "conn",                 // 146
    "connCur",              // 147
    "connErr",              // 148
    "connHis",              // 149
    "connRef",              // 150
    "connState",            // 151
    "connStatus",           // 152
    "connTuning",           // 153
    "connTuningRef",        // 154
    "connWrite",            // 155
    "connection",           // 156
    "constantVolume",       // 157
    "consumption",          // 158
    "cool",                 // 159
    "coolOnly",             // 160
    "cooling",              // 161
    "coolingCapacity",      // 162
    "coolingTower",         // 163
    "cost",                 // 164
    "cov",                  // 165
    "crc",                  // 166
    "created",              // 167
    "cur",                  // 168
    "curCalibration",       // 169
    "curConvert",           // 170
    "curErr",               // 171
    "curKpi",               // 172
    "curSpark",             // 173
    "curStatus",            // 174
    "curVal",               // 175
    "current",              // 176
    "damper",               // 177
    "date",                 // 178
    "delta",                // 179
    "demand",               // 180
    "device",               // 181
    "device1Ref",           // 182
    "device2Ref",           // 183
    "directZone",           // 184
    "dis",                  // 185
    "disMacro",             // 186
    "discharge",            // 187
    "domestic",             // 188
    "dualDuct",             // 189
    "ductArea",             // 190
    "dur",                  // 191
    "dxCool",               // 192
    "effective",            // 193
    "efficiency",           // 194
    "elec",                 // 195
    "elecHeat",             // 196
    "elecMeterLoad",        // 197
    "elecMeterRef",         // 198
    "elecPanel",            // 199
    "elecPanelOf",          // 200
    "elecReheat",           // 201
    "email",                // 202
    "enable",               // 203
    "energy",               // 204
    "entering",             // 205
    "enum",                 // 206
    "equip",                // 207
    "equipRef",             // 208
    "evaporator",           // 209
    "exhaust",              // 210
    "ext",                  // 211
    "faceBypass",           // 212
    "fan",                  // 213
    "fanPowered",           // 214
    "fcu",                  // 215
    "filter",               // 216
    "finAsset",             // 217
    "finDependencies",      // 218
    "finFile",              // 219
    "finIcon",              // 220
    "finProject",           // 221
    "finResource",          // 222
    "finRuntime",           // 223
    "finScreenshot",        // 224
    "finThumb",             // 225
    "finUri",               // 226
    "finVersion",           // 227
    "floor",                // 228
    "floorRef",             // 229
    "flow",                 // 230
    "folderPath",           // 231
    "formOn",               // 232
    "freezeStat",           // 233
    "freq",                 // 234
    "func",                 // 235
    "gas",                  // 236
    "gasHeat",              // 237
    "gasMeterLoad",         // 238
    "geoAddr",              // 239
    "geoCity",              // 240
    "geoCoord",             // 241
    "geoCountry",           // 242
    "geoCounty",            // 243
    "geoPostalCode",        // 244
    "geoState",             // 245
    "geoStreet",            // 246
    "graphicOn",            // 247
    "haystackConnRef",      // 248
    "haystackCur",          // 249
    "haystackHis",          // 250
    "haystackWrite",        // 251
    "haystackWriteLevel",   // 252
    "haytackConn",          // 253
    "heat",                 // 254
    "heatExchanger",        // 255
    "heatPump",             // 256
    "heatWheel",            // 257
    "heating",              // 258
    "help",                 // 259
    "helpDoc",              // 260
    "his",                  // 261
    "hisCollectCov",        // 262
    "hisCollectInterval",   // 263
    "hisConvert",           // 264
    "hisEnd",               // 265
    "hisEndVal",            // 266
    "hisErr",               // 267
    "hisFunc",              // 268
    "hisId",                // 269
    "hisInterpolate",       // 270
    "hisInterval",          // 271
    "hisKpi",               // 272
    "hisMode",              // 273
    "hisRef",               // 274
    "hisSize",              // 275
    "hisSpark",             // 276
    "hisStart",             // 277
    "hisStatus",            // 278
    "hisTotalized",         // 279
    "hot",                  // 280
    "hotDeck",              // 281
    "hotWaterHeat",         // 282
    "hotWaterReheat",       // 283
    "humidifier",           // 284
    "humidity",             // 285
    "hvac",                 // 286
    "id",                   // 287
    "imageRef",             // 288
    "index",                // 289
    "isolation",            // 290
    "kind",                 // 291
    "kpi",                  // 292
    "kpiFunc",              // 293
    "kpiOn",                // 294
    "kpiRef",               // 295
    "leaving",              // 296
    "license",              // 297
    "licenseRef",           // 298
    "lightLevel",           // 299
    "lighting",             // 300
    "lights",               // 301
    "lightsGroup",          // 302
    "load",                 // 303
    "maint",                // 304
    "maintRef",             // 305
    "makeup",               // 306
    "max",                  // 307
    "maxVal",               // 308
    "meter",                // 309
    "min",                  // 310
    "minVal",               // 311
    "mixed",                // 312
    "mod",                  // 313
    "multiZone",            // 314
    "name",                 // 315
    "navName",              // 316
    "network",              // 317
    "networkRef",           // 318
    "neutralDeck",          // 319
    "nextTime",             // 320
    "nextVal",              // 321
    "note",                 // 322
    "noteRef",              // 323
    "num",                  // 324
    "number",               // 325
    "obixConn",             // 326
    "obixConnRef",          // 327
    "obixCur",              // 328
    "obixHis",              // 329
    "obixWrite",            // 330
    "occ",                  // 331
    "occupancyIndicator",   // 332
    "occupied",             // 333
    "openLoop",             // 334
    "order",                // 335
    "orderItem",            // 336
    "orderItemRef",         // 337
    "orderRef",             // 338
    "org",                  // 339
    "orgRef",               // 340
    "outside",              // 341
    "parallel",             // 342
    "part",                 // 343
    "partRef",              // 344
    "perimeterHeat",        // 345
    "periods",              // 346
    "pf",                   // 347
    "phase",                // 348
    "point",                // 349
    "pointRef",             // 350
    "power",                // 351
    "precision",            // 352
    "pressure",             // 353
    "pressureDependent",    // 354
    "pressureIndependent",  // 355
    "primaryFunction",      // 356
    "primaryLoop",          // 357
    "protocol",             // 358
    "pump",                 // 359
    "reciprocal",           // 360
    "refrig",               // 361
    "region",               // 362
    "regionRef",            // 363
    "reheat",               // 364
    "reheating",            // 365
    "return",               // 366
    "rooftop",              // 367
    "rule",                 // 368
    "ruleFunc",             // 369
    "ruleOn",               // 370
    "ruleRef",              // 371
    "run",                  // 372
    "sampled",              // 373
    "schedulable",          // 374
    "schedule",             // 375
    "scheduleRef",          // 376
    "screw",                // 377
    "secondaryLoop",        // 378
    "sensor",               // 379
    "series",               // 380
    "singleDuct",           // 381
    "site",                 // 382
    "siteMeter",            // 383
    "sitePanel",            // 384
    "sitePoint",            // 385
    "siteRef",              // 386
    "sp",                   // 387
    "space",                // 388
    "spaceRef",             // 389
    "spark",                // 390
    "speed",                // 391
    "src",                  // 392
    "stage",                // 393
    "standby",              // 394
    "steam",                // 395
    "steamHeat",            // 396
    "steamMeterLoad",       // 397
    "subPanelOf",           // 398
    "submeterOf",           // 399
    "summary",              // 400
    "sunrise",              // 401
    "supply",               // 402
    "temp",                 // 403
    "ticket",               // 404
    "ticketStatus",         // 405
    "times",                // 406
    "tripleDuct",           // 407
    "ts",                   // 408
    "tz",                   // 409
    "unit",                 // 410
    "unocc",                // 411
    "uri",                  // 412
    "user",                 // 413
    "userRef",              // 414
    "username",             // 415
    "uv",                   // 416
    "v0",                   // 417
    "v1",                   // 418
    "v2",                   // 419
    "v3",                   // 420
    "v4",                   // 421
    "v5",                   // 422
    "v7",                   // 423
    "v8",                   // 424
    "v9",                   // 425
    "val",                  // 426
    "valve",                // 427
    "variableVolume",       // 428
    "vav",                  // 429
    "vavMode",              // 430
    "vavZone",              // 431
    "version",              // 432
    "vfd",                  // 433
    "volt",                 // 434
    "volume",               // 435
    "water",                // 436
    "waterCooled",          // 437
    "waterMeterLoad",       // 438
    "weather",              // 439
    "weatherCond",          // 440
    "weatherPoint",         // 441
    "weatherRef",           // 442
    "wetBulb",              // 443
    "writable",             // 444
    "writeConvert",         // 445
    "writeErr",             // 446
    "writeLevel",           // 447
    "writeStatus",          // 448
    "writeVal",             // 449
    "yearBuilt",            // 450
    "zone",                 // 451
    "zoneRef",              // 452
    // units
    "$",          // 453
    "%",          // 454
    "%/s",        // 455
    "%RH",        // 456
    "%obsc/ft",   // 457
    "%obsc/m",    // 458
    "/h",         // 459
    "/min",       // 460
    "/s",         // 461
    "A",          // 462
    "A/m",        // 463
    "A/m²",       // 464
    "AED",        // 465
    "AUD",        // 466
    "Am²",        // 467
    "BTU",        // 468
    "BTU/h",      // 469
    "BTU/lb",     // 470
    "C",          // 471
    "CAD",        // 472
    "COP",        // 473
    "DCIE",       // 474
    "EER",        // 475
    "F",          // 476
    "Fr",         // 477
    "GB",         // 478
    "GJ",         // 479
    "GW",         // 480
    "H",          // 481
    "Hz",         // 482
    "J",          // 483
    "J/g",        // 484
    "J/h",        // 485
    "J/kg",       // 486
    "J/kg_dry",   // 487
    "J/kg°K",     // 488
    "J/m²",       // 489
    "J/°K",       // 490
    "Js",         // 491
    "K",          // 492
    "K/h",        // 493
    "K/min",      // 494
    "K/s",        // 495
    "L",          // 496
    "L/h",        // 497
    "L/min",      // 498
    "L/s",        // 499
    "MB",         // 500
    "MBTU/ft²",   // 501
    "MHz",        // 502
    "MJ",         // 503
    "MJ/ft²",     // 504
    "MJ/h",       // 505
    "MJ/kg_dry",  // 506
    "MJ/m²",      // 507
    "MJ/°K",      // 508
    "MMBTU",      // 509
    "MMBTU/h",    // 510
    "MV",         // 511
    "MVAR",       // 512
    "MVARh",      // 513
    "MVAh",       // 514
    "MW",         // 515
    "MWh",        // 516
    "MWh/ft²",    // 517
    "MWh/m²",     // 518
    "MΩ",         // 519
    "N",          // 520
    "N/m",        // 521
    "NIS",        // 522
    "Nm",         // 523
    "Ns",         // 524
    "PB",         // 525
    "PUE",        // 526
    "Pa",         // 527
    "S",          // 528
    "S/m",        // 529
    "T",          // 530
    "TB",         // 531
    "TWD",        // 532
    "V",          // 533
    "V/K",        // 534
    "V/m",        // 535
    "VA",         // 536
    "VAR",        // 537
    "VARh",       // 538
    "VAh",        // 539
    "W",          // 540
    "W/cfm",      // 541
    "W/ft²",      // 542
    "W/ft²_irr",  // 543
    "W/m°K",      // 544
    "W/m²",       // 545
    "W/m²K",      // 546
    "W/m²_irr",   // 547
    "W/m³/s",     // 548
    "Wb",         // 549
    "Wh",         // 550
    "Wh/ft²",     // 551
    "Wh/m²",      // 552
    "acre",       // 553
    "atm",        // 554
    "bar",        // 555
    "btu/lb_dry", // 556
    "byte",       // 557
    "cal",        // 558
    "cal/g",      // 559
    "cd",         // 560
    "cd/m²",      // 561
    "cfh",        // 562
    "cfm",        // 563
    "cfs",        // 564
    "cm",         // 565
    "cmHg",       // 566
    "cmH₂O",      // 567
    "cm²",        // 568
    "cm³",        // 569
    "cph",        // 570
    "cpm",        // 571
    "cs",         // 572
    "dBmV",       // 573
    "dBµV",       // 574
    "day",        // 575
    "db",         // 576
    "deg",        // 577
    "degPh",      // 578
    "ds",         // 579
    "fl_oz",      // 580
    "fnu",        // 581
    "ft",         // 582
    "ft/min",     // 583
    "ft/s",       // 584
    "ftcd",       // 585
    "ftlbs/sec",  // 586
    "ft²",        // 587
    "ft³",        // 588
    "ft³_gas",    // 589
    "g",          // 590
    "g/kg",       // 591
    "g/min",      // 592
    "g/m²",       // 593
    "g/s",        // 594
    "gH₂O/kgAir", // 595
    "gal",        // 596
    "gal/min",    // 597
    "galUK",      // 598
    "galUK/min",  // 599
    "h",          // 600
    "hL",         // 601
    "hL/s",       // 602
    "hPa",        // 603
    "hft³",       // 604
    "hp",         // 605
    "hph",        // 606
    "in",         // 607
    "inHg",       // 608
    "inH₂O",      // 609
    "in²",        // 610
    "in³",        // 611
    "kB",         // 612
    "kBTU",       // 613
    "kBTU/ft²",   // 614
    "kBTU/h",     // 615
    "kBTU/h/ft²", // 616
    "kHz",        // 617
    "kJ",         // 618
    "kJ/h",       // 619
    "kJ/kg",      // 620
    "kJ/kg_dry",  // 621
    "kJ/°K",      // 622
    "kL",         // 623
    "kPa",        // 624
    "kV",         // 625
    "kVA",        // 626
    "kVAR",       // 627
    "kVARh",      // 628
    "kVAh",       // 629
    "kW",         // 630
    "kW/ft²",     // 631
    "kW/gal/min", // 632
    "kW/kcfm",    // 633
    "kW/m²",      // 634
    "kW/ton",     // 635
    "kWh",        // 636
    "kWh/ft²",    // 637
    "kWh/m²",     // 638
    "kcfm",       // 639
    "kg",         // 640
    "kg/h",       // 641
    "kg/min",     // 642
    "kg/m²",      // 643
    "kg/m³",      // 644
    "kg/s",       // 645
    "kgal",       // 646
    "klb",        // 647
    "klb/h",      // 648
    "km",         // 649
    "km/h",       // 650
    "km/s",       // 651
    "km²",        // 652
    "knot",       // 653
    "kr",         // 654
    "kΩ",         // 655
    "lb",         // 656
    "lb/h",       // 657
    "lb/min",     // 658
    "lb/s",       // 659
    "lbf",        // 660
    "lm",         // 661
    "lx",         // 662
    "m",          // 663
    "m/h",        // 664
    "m/min",      // 665
    "m/s",        // 666
    "m/s²",       // 667
    "mA",         // 668
    "mL",         // 669
    "mL/s",       // 670
    "mV",         // 671
    "mVA",        // 672
    "mW",         // 673
    "mbar",       // 674
    "mg",         // 675
    "mile",       // 676
    "mile²",      // 677
    "mm",         // 678
    "mm/min",     // 679
    "mm/s",       // 680
    "mmHg",       // 681
    "mm²",        // 682
    "mm³",        // 683
    "mo",         // 684
    "mph",        // 685
    "ms",         // 686
    "m²",         // 687
    "m²/N",       // 688
    "m³",         // 689
    "m³/h",       // 690
    "m³/min",     // 691
    "m³/s",       // 692
    "m³_gas",     // 693
    "mΩ",         // 694
    "ns",         // 695
    "ntu",        // 696
    "oz",         // 697
    "pH",         // 698
    "ppb",        // 699
    "ppm",        // 700
    "ppu",        // 701
    "psi",        // 702
    "psi/°F",     // 703
    "pt",         // 704
    "px",         // 705
    "qt",         // 706
    "rad",        // 707
    "rad/s",      // 708
    "rad/s²",     // 709
    "rpm",        // 710
    "s",          // 711
    "sr",         // 712
    "t",          // 713
    "therm",      // 714
    "therm/h",    // 715
    "ton",        // 716
    "ton/h",      // 717
    "tonref",     // 718
    "tonrefh",    // 719
    "wk",         // 720
    "yd",         // 721
    "yd²",        // 722
    "yd³",        // 723
    "yr",         // 724
    "£",          // 725
    "¥",          // 726
    "°C",         // 727
    "°C/h",       // 728
    "°C/min",     // 729
    "°F",         // 730
    "°F/h",       // 731
    "°F/min",     // 732
    "°daysC",     // 733
    "°daysF",     // 734
    "µg/m³",      // 735
    "µm",         // 736
    "µs",         // 737
    "ΔK",         // 738
    "Δ°C",        // 739
    "Δ°F",        // 740
    "руб",        // 741
    "₩",          // 742
    "€",          // 743
    "₹",          // 744
    "Ω",          // 745
    "Ωm",         // 746
    "元",         // 747
    // 3.0.15
    "accept-charset",    // 748
    "accept-encoding",   // 749
    "accept-language",   // 750
    "all",               // 751
    "auto",              // 752
    "aux",               // 753
    "avg",               // 754
    "baseline",          // 755
    "cache-control",     // 756
    "call",              // 757
    "cells",             // 758
    "children",          // 759
    "clear",             // 760
    "cloudy",            // 761
    "clusterSessionKey", // 762
    "clusterUsername",   // 763
    "code",              // 764
    "cols",              // 765
    "content",           // 766
    "content-encoding",  // 767
    "content-length",    // 768
    "content-type",      // 769
    "cookie",            // 770
    "dates",             // 771
    "days",              // 772
    "define",            // 773
    "delete",            // 774
    "doc",               // 775
    "equipAccessFilter", // 776
    "equips",            // 777
    "etag",              // 778
    "eval",              // 779
    "expires",           // 780
    "expr",              // 781
    "extra",             // 782
    "firstName",         // 783
    "flurries",          // 784
    "fold",              // 785
    "get",               // 786
    "group",             // 787
    "groupBy",           // 788
    "gzip",              // 789
    "hasChildren",       // 790
    "head",              // 791
    "headers",           // 792
    "hidden",            // 793
    "hisRollup",         // 794
    "hisRollupDis",      // 795
    "hisRollupInterval", // 796
    "host",              // 797
    "http",              // 798
    "https",             // 799
    "ice",               // 800
    "icon",              // 801
    "ids",               // 802
    "interval",          // 803
    "keep-alive",        // 804
    "key",               // 805
    "kpiRule",           // 806
    "last-modified",     // 807
    "lastName",          // 808
    "list",              // 809
    "manifest",          // 810
    "map",               // 811
    "mapToHis",          // 812
    "method",            // 813
    "mode",              // 814
    "msg",               // 815
    "msgId",             // 816
    "msgType",           // 817
    "names",             // 818
    "op",                // 819
    "options",           // 820
    "opts",              // 821
    "origin",            // 822
    "partlyCloudy",      // 823
    "pattern",           // 824
    "phrase",            // 825
    "pipe",              // 826
    "pointAccessFilter", // 827
    "points",            // 828
    "post",              // 829
    "pragma",            // 830
    "priority",          // 831
    "projAccessFilter",  // 832
    "projs",             // 833
    "put",               // 834
    "query",             // 835
    "rain",              // 836
    "read",              // 837
    "readAll",           // 838
    "readById",          // 839
    "readByIds",         // 840
    "referer",           // 841
    "rollup",            // 842
    "rows",              // 843
    "ruleAccessFilter",  // 844
    "ruleType",          // 845
    "rules",             // 846
    "scheme",            // 847
    "sel",               // 848
    "select",            // 849
    "selectable",        // 850
    "server",            // 851
    "showers",           // 852
    "siteAccessFilter",  // 853
    "sites",             // 854
    "snow",              // 855
    "span",              // 856
    "sparkRule",         // 857
    "status",            // 858
    "sum",               // 859
    "targetRef",         // 860
    "targets",           // 861
    "text",              // 862
    "thunderstorms",     // 863
    "timeout",           // 864
    "transfer-encoding", // 865
    "type",              // 866
    "user-agent",        // 867
    "userAdmin",         // 868
    "userAuthScheme",    // 869
    "view",              // 870
    "viz",               // 871
    // 3.0.17
    "accept",                // 872
    "appName",               // 873
    "base",                  // 874
    "batch",                 // 875
    "bootId",                // 876
    "bootTime",              // 877
    "charge",                // 878
    "chargeType",            // 879
    "describe",              // 880
    "disKey",                // 881
    "dispatch",              // 882
    "err",                   // 883
    "errTrace",              // 884
    "errType",               // 885
    "executeStatus",         // 886
    "executeTime",           // 887
    "find",                  // 888
    "findAll",               // 889
    "fingerprint",           // 890
    "flatMap",               // 891
    "folioVersion",          // 892
    "hash",                  // 893
    "hisPageSize",           // 894
    "hostId",                // 895
    "hostModel",             // 896
    "inRange",               // 897
    "level",                 // 898
    "licProduct",            // 899
    "locale",                // 900
    "masterVer",             // 901
    "maxCount",              // 902
    "maxDataSize",           // 903
    "node",                  // 904
    "nodeId",                // 905
    "nonce",                 // 906
    "numBlobs",              // 907
    "periodUnion",           // 908
    "ping",                  // 909
    "poll",                  // 910
    "proj",                  // 911
    "pubKey",                // 912
    "push",                  // 913
    "range",                 // 914
    "rangeStrategy",         // 915
    "ranges",                // 916
    "replicaVer",            // 917
    "req",                   // 918
    "res",                   // 919
    "route",                 // 920
    "routeStatus",           // 921
    "salt",                  // 922
    "scheduleVal",           // 923
    "scram",                 // 924
    "send",                  // 925
    "shape",                 // 926
    "SHA-256",               // 927
    "sig",                   // 928
    "skyarc-ui-session-key", // 929
    "spec",                  // 930
    "specVer",               // 931
    "stash",                 // 932
    "steps",                 // 933
    "target",                // 934
    "tariff",                // 935
    "tariffHis",             // 936
    "tariffRef",             // 937
    "trace",                 // 938
    "traces",                // 939
    "uiMeta",                // 940
    "usageOn",               // 941
    "useReplica",            // 942
    "userAuth",              // 943
    "userRole",              // 944
    "ver",                   // 945
    // 3.0.25
    "admin",                                                                          // 946
    "arc",                                                                            // 947
    "audit",                                                                          // 948
    "by",                                                                             // 949
    "clusterAttestKey",                                                               // 950
    "comment",                                                                        // 951
    "def",                                                                            // 952
    "defx",                                                                           // 953
    "file",                                                                           // 954
    "input",                                                                          // 955
    "is",                                                                             // 956
    "item",                                                                           // 957
    "items",                                                                          // 958
    "of",                                                                             // 959
    "parts",                                                                          // 960
    "person",                                                                         // 961
    "skyarc::UiDef",                                                                  // 962
    "skyarc::User",                                                                   // 963
    "su",                                                                             // 964
    "tagOn",                                                                          // 965
    "unknown,clear,partlyCloudy,cloudy,showers,rain,thunderstorms,ice,flurries,snow", // 966
    "userProto",                                                                      // 967
    "userProtoName",                                                                  // 968
    "userProtoRef",                                                                   // 969
    // 3.0.27
    "airRef",            // 970
    "arcBreakdown",      // 971
    "arcBug",            // 972
    "arcDamage",         // 973
    "arcElectrical",     // 974
    "arcEnhancement",    // 975
    "arcHvac",           // 976
    "arcInspection",     // 977
    "arcMaintenance",    // 978
    "arcOn",             // 979
    "arcPlumbing",       // 980
    "arcPriority",       // 981
    "arcSafety",         // 982
    "arcSupport",        // 983
    "arcWish",           // 984
    "assignedTo",        // 985
    "cancelled",         // 986
    "critical",          // 987
    "dueDate",           // 988
    "elecRef",           // 989
    "high",              // 990
    "low",               // 991
    "medium",            // 992
    "new",               // 993
    "old",               // 994
    "open",              // 995
    "resolved",          // 996
    "ticketState",       // 997
    "viewLink",          // 998
    "weatherStation",    // 999
    "weatherStationRef", // 1000
    "workorder",         // 1001
    "workorderState",    // 1002
];

// ---------------------------------------------------------------------------
// Reverse-lookup map (string → index), lazily initialised once.
// ---------------------------------------------------------------------------

static CONSTS_MAP: OnceLock<HashMap<&'static str, i64>> = OnceLock::new();

fn consts_map() -> &'static HashMap<&'static str, i64> {
    CONSTS_MAP.get_or_init(|| {
        // Index 0 is the empty string ""; include it so lookup_const("") → Some(0).
        CONSTS
            .iter()
            .enumerate()
            .map(|(i, &s)| (s, i as i64))
            .collect()
    })
}

/// Look up `s` in the constant table.
///
/// Returns `Some(index)` (1-based) if found, `None` otherwise.
pub fn lookup_const(s: &str) -> Option<i64> {
    consts_map().get(s).copied()
}

/// Retrieve the constant string at `idx` (0-based; 0 = `""`).
///
/// Returns `Some(&str)` if `idx` is valid, `None` otherwise.
pub fn get_const(idx: i64) -> Option<&'static str> {
    if idx >= 0 && (idx as usize) < CONSTS.len() {
        Some(CONSTS[idx as usize])
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_length() {
        // 0 placeholder + 1002 real entries
        assert_eq!(CONSTS.len(), 1003);
    }

    #[test]
    fn test_index_zero_is_empty_string() {
        // BrioTest.fan: verifyConsts(cp, "", 0)
        // The empty string is const index 0 and IS used on the wire.
        assert_eq!(CONSTS[0], "");
        assert_eq!(lookup_const(""), Some(0));
        assert_eq!(get_const(0), Some(""));
    }

    #[test]
    fn test_known_tag_indices() {
        assert_eq!(CONSTS[185], "dis");
        assert_eq!(CONSTS[382], "site");
        assert_eq!(CONSTS[386], "siteRef");
        assert_eq!(CONSTS[208], "equipRef");
        assert_eq!(CONSTS[408], "ts");
        assert_eq!(CONSTS[409], "tz");
        assert_eq!(CONSTS[175], "curVal");
        assert_eq!(CONSTS[287], "id");
        assert_eq!(CONSTS[403], "temp");
    }

    #[test]
    fn test_known_timezone_indices() {
        assert_eq!(CONSTS[24], "UTC");
        assert_eq!(CONSTS[26], "New_York");
        assert_eq!(CONSTS[57], "London");
        assert_eq!(CONSTS[44], "Tokyo");
    }

    #[test]
    fn test_known_unit_indices() {
        assert_eq!(CONSTS[630], "kW");
        assert_eq!(CONSTS[636], "kWh");
        assert_eq!(CONSTS[711], "s");
        assert_eq!(CONSTS[727], "°C");
        assert_eq!(CONSTS[730], "°F");
        assert_eq!(CONSTS[563], "cfm");
    }

    #[test]
    fn test_lookup_const_found() {
        assert_eq!(lookup_const("dis"), Some(185));
        assert_eq!(lookup_const("site"), Some(382));
        assert_eq!(lookup_const("UTC"), Some(24));
        assert_eq!(lookup_const("kW"), Some(630));
        assert_eq!(lookup_const("°F"), Some(730));
    }

    #[test]
    fn test_lookup_const_not_found() {
        // "" is now index 0 — it SHOULD be found.
        assert_eq!(lookup_const(""), Some(0));
        assert_eq!(lookup_const("notAConst"), None);
    }

    #[test]
    fn test_get_const_valid() {
        assert_eq!(get_const(185), Some("dis"));
        assert_eq!(get_const(24), Some("UTC"));
        assert_eq!(get_const(630), Some("kW"));
        assert_eq!(get_const(1002), Some("workorderState"));
    }

    #[test]
    fn test_get_const_invalid() {
        // 0 is the empty string — now valid.
        assert_eq!(get_const(0), Some(""));
        assert_eq!(get_const(-1), None);
        assert_eq!(get_const(1003), None);
        assert_eq!(get_const(9999), None);
    }

    #[test]
    fn test_lookup_roundtrip() {
        // Every entry including index 0 ("") should round-trip.
        for (i, &s) in CONSTS.iter().enumerate() {
            let idx =
                lookup_const(s).unwrap_or_else(|| panic!("Missing const at index {i}: {s:?}"));
            assert_eq!(idx, i as i64);
            let got = get_const(idx).unwrap();
            assert_eq!(got, s);
        }
    }
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#[macro_use]
extern crate float_cmp;

use kdbplus::*;
use kdbplus::ipc::*;
use chrono::prelude::*;
use chrono::Duration;

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                        Macros                         //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

macro_rules! assert_eq_float {
  ($lhs: expr, $rhs: expr, $precision: expr) => {
    assert!(approx_eq!(f64, $lhs, $rhs, epsilon=$precision))
  };
}

macro_rules! assert_eq_float_vec {
  ($lhs: expr, $rhs: expr, $precision: expr) => {
    for (&v1, &v2) in $lhs.iter().zip($rhs.iter()){
      assert!(approx_eq!(f64, v1, v2, epsilon=$precision))
    }
  };
}

macro_rules! add_null {
  ($obj: expr) => {
    K::new_compound_list(vec![K::new_null(), $obj])
  };
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Test Functions                    //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#[test]
fn format_test()->Result<(), &'static str>{

  // bool true
  let q_bool_true=K::new_bool(true);
  assert_eq!(format!("{}", q_bool_true), String::from("1b"));

  // bool false
  let q_bool_false=K::new_bool(false);
  assert_eq!(format!("{}", q_bool_false), String::from("0b"));

  // GUID
  let q_guid=K::new_guid([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
  assert_eq!(format!("{}", q_guid), String::from("01020304-0506-0708-090a-0b0c0d0e0f10"));

  // GUID null
  let q_guid_null=K::new_guid(qnull::GUID);
  assert_eq!(format!("{}", q_guid_null), String::from("00000000-0000-0000-0000-000000000000"));

  // byte
  let q_byte=K::new_byte(0x9e);
  assert_eq!(format!("{}", q_byte), String::from("0x9e"));

  // short
  let q_short=K::new_short(17);
  assert_eq!(format!("{}", q_short), String::from("17h"));

  // short null
  let q_short_null=K::new_short(qnull::SHORT);
  assert_eq!(format!("{}", q_short_null), String::from("0Nh"));

  // short inf
  let q_short_inf=K::new_short(qinf::SHORT);
  assert_eq!(format!("{}", q_short_inf), String::from("0Wh"));

  // short ninf
  let q_short_ninf=K::new_short(qninf::SHORT);
  assert_eq!(format!("{}", q_short_ninf), String::from("-0Wh"));

  // int
  let q_int=K::new_int(-256);
  assert_eq!(format!("{}", q_int), String::from("-256i"));

  // int null
  let q_int_null=K::new_int(qnull::INT);
  assert_eq!(format!("{}", q_int_null), String::from("0Ni"));

  // int inf
  let q_int_inf=K::new_int(qinf::INT);
  assert_eq!(format!("{}", q_int_inf), String::from("0Wi"));

  // short ninf
  let q_int_ninf=K::new_int(qninf::INT);
  assert_eq!(format!("{}", q_int_ninf), String::from("-0Wi"));

  // long
  let q_long=K::new_long(86400000000000);
  assert_eq!(format!("{}", q_long), String::from("86400000000000"));

  // long null
  let q_long_null=K::new_long(qnull::LONG);
  assert_eq!(format!("{}", q_long_null), String::from("0N"));

  // long inf
  let q_long_inf=K::new_long(qinf::LONG);
  assert_eq!(format!("{}", q_long_inf), String::from("0W"));

  // long ninf
  let q_long_ninf=K::new_long(qninf::LONG);
  assert_eq!(format!("{}", q_long_ninf), String::from("-0W"));

  // real
  let q_real=K::new_real(0.25);
  assert_eq!(format!("{:.2}", q_real), String::from("0.25e"));

  // real null
  let q_real_null=K::new_real(qnull::REAL);
  assert_eq!(format!("{}", q_real_null), String::from("0Ne"));

  // real inf
  let q_real_inf: K=K::new_real(qinf::REAL);
  assert_eq!(format!("{}", q_real_inf), String::from("0We"));

  // real ninf
  let q_real_ninf: K=K::new_real(qninf::REAL);
  assert_eq!(format!("{}", q_real_ninf), String::from("-0We"));

  // float
  let q_float=K::new_float(113.0456);
  assert_eq!(format!("{:.7}", q_float), String::from("113.0456000"));

  // float null
  let q_float_null=K::new_float(qnull::FLOAT);
  assert_eq!(format!("{}", q_float_null), String::from("0n"));

  // float inf
  let q_float_inf=K::new_float(qinf::FLOAT);
  assert_eq!(format!("{}", q_float_inf), String::from("0w"));

  // float ninf
  let q_float_ninf=K::new_float(qninf::FLOAT);
  assert_eq!(format!("{}", q_float_ninf), String::from("-0w"));  

  // char
  let q_char=K::new_char('r');
  assert_eq!(format!("{}", q_char), String::from("\"r\""));  

  // char null
  let q_char_null=K::new_char(qnull::CHAR);
  assert_eq!(format!("{}", q_char_null), String::from("\" \""));

  // symbol
  let q_symbol=K::new_symbol(String::from("Jordan"));
  assert_eq!(format!("{}", q_symbol), String::from("`Jordan"));  

  // symbol null
  let q_symbol_null=K::new_symbol(qnull::SYMBOL);
  assert_eq!(format!("{}", q_symbol_null), String::from("`"));

  // timestamp
  let q_timestamp=K::new_timestamp(Utc.ymd(2019, 5, 9).and_hms_nano(0, 39, 2, 194756));
  assert_eq!(format!("{}", q_timestamp), String::from("2019.05.09D00:39:02.000194756"));

  // timestamp null
  let q_timestamp_null=K::new_timestamp(*qnull::TIMESTAMP);
  assert_eq!(format!("{}", q_timestamp_null), String::from("0Np"));

  // timestamp inf
  let q_timestamp_inf=K::new_timestamp(*qinf::TIMESTAMP);
  assert_eq!(format!("{}", q_timestamp_inf), String::from("0Wp"));

  // timestamp ninf
  let q_timestamp_ninf=K::new_timestamp(*qninf::TIMESTAMP);
  assert_eq!(format!("{}", q_timestamp_ninf), String::from("-0Wp"));

  // month
  let q_month=K::new_month(Utc.ymd(2019, 12, 15));
  assert_eq!(format!("{}", q_month), String::from("2019.12m"));

  // month null
  let q_month_null=K::new_month(qnull::MONTH);
  assert_eq!(format!("{}", q_month_null), String::from("0Nm"));

  // month inf
  let q_month_inf=K::new_month(*qinf::MONTH);
  assert_eq!(format!("{}", q_month_inf), String::from("0Wm"));

  // month ninf
  let q_month_ninf=K::new_month(*qninf::MONTH);
  assert_eq!(format!("{}", q_month_ninf), String::from("-0Wm"));

  // date
  let q_date=K::new_date(Utc.ymd(2012, 3, 12));
  assert_eq!(format!("{}", q_date), String::from("2012.03.12"));

  // date null
  let q_date_null=K::new_date(qnull::DATE);
  assert_eq!(format!("{}", q_date_null), String::from("0Nd"));

  // date inf
  let q_date_inf=K::new_date(qinf::DATE);
  assert_eq!(format!("{}", q_date_inf), String::from("0Wd"));

  // date ninf
  let q_date_ninf=K::new_date(*qninf::DATE);
  assert_eq!(format!("{}", q_date_ninf), String::from("-0Wd"));

  // datetime
  let q_datetime=K::new_datetime(Utc.ymd(2013, 1, 10).and_hms_milli(0, 9, 50, 38));
  assert_eq!(format!("{}", q_datetime), String::from("2013.01.10T00:09:50.038"));

  // datetime null
  let q_datetime_null=K::new_datetime(qnull::DATETIME);
  assert_eq!(format!("{}", q_datetime_null), String::from("0Nz"));

  // datetime inf
  let q_datetime_inf=K::new_datetime(*qinf::DATETIME);
  assert_eq!(format!("{}", q_datetime_inf), String::from("0Wz"));

  // datetime ninf
  let q_datetime_ninf=K::new_datetime(*qninf::DATETIME);
  assert_eq!(format!("{}", q_datetime_ninf), String::from("-0Wz"));

  // timespan
  let q_timespan=K::new_timespan(Duration::nanoseconds(102899277539844));
  assert_eq!(format!("{}", q_timespan), String::from("1D04:34:59.277539844"));

  // timespan null
  let q_timespan_null=K::new_timespan(*qnull::TIMESPAN);
  assert_eq!(format!("{}", q_timespan_null), String::from("0Nn"));

  // timespan inf
  let q_timespan_inf=K::new_timespan(*qinf::TIMESPAN);
  assert_eq!(format!("{}", q_timespan_inf), String::from("0Wn"));

  // timespan ninf
  let q_timespan_ninf=K::new_timespan(*qninf::TIMESPAN);
  assert_eq!(format!("{}", q_timespan_ninf), String::from("-0Wn"));

  // minute
  let q_minute=K::new_minute(Duration::minutes(99));
  assert_eq!(format!("{}", q_minute), String::from("01:39"));

  // minute null
  let q_minute_null=K::new_minute(*qnull::MINUTE);
  assert_eq!(format!("{}", q_minute_null), String::from("0Nu"));

  // minute inf
  let q_minute_inf=K::new_minute(*qinf::MINUTE);
  assert_eq!(format!("{}", q_minute_inf), String::from("0Wu"));

  // minute ninf
  let q_minute_ninf=K::new_minute(*qninf::MINUTE);
  assert_eq!(format!("{}", q_minute_ninf), String::from("-0Wu"));

  // second
  let q_second=K::new_second(Duration::seconds(3702));
  assert_eq!(format!("{}", q_second), String::from("01:01:42"));

  // second null
  let q_second_null=K::new_second(*qnull::SECOND);
  assert_eq!(format!("{}", q_second_null), String::from("0Nv"));

  // second inf
  let q_second_inf=K::new_second(*qinf::SECOND);
  assert_eq!(format!("{}", q_second_inf), String::from("0Wv"));

  // second ninf
  let q_second_ninf=K::new_second(*qninf::SECOND);
  assert_eq!(format!("{}", q_second_ninf), String::from("-0Wv"));

  // time
  let q_time=K::new_time(Duration::milliseconds(27843489));
  assert_eq!(format!("{}", q_time), String::from("07:44:03.489"));

  // time null
  let q_time_null=K::new_time(*qnull::TIME);
  assert_eq!(format!("{}", q_time_null), String::from("0Nt"));

  // time inf
  let q_time_inf=K::new_time(*qinf::TIME);
  assert_eq!(format!("{}", q_time_inf), String::from("0Wt"));

  // second ninf
  let q_time_ninf=K::new_time(*qninf::TIME);
  assert_eq!(format!("{}", q_time_ninf), String::from("-0Wt"));

  // bool list
  let mut q_bool_list=K::new_bool_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_bool_list), String::from("`bool$()"));
  q_bool_list=K::new_bool_list(vec![true, false, true], qattribute::NONE);
  assert_eq!(format!("{}", q_bool_list), String::from("101b"));

  // guid list
  let mut q_guid_list=K::new_guid_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_guid_list), String::from("`guid$()"));
  q_guid_list=K::new_guid_list(vec![[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15], [240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255]], qattribute::NONE);
  assert_eq!(format!("{}", q_guid_list), String::from("00010203-0405-0607-0809-0a0b0c0d0e0f f0f1f2f3-f4f5-f6f7-f8f9-fafbfcfdfeff"));

  // byte list
  let mut q_byte_list=K::new_byte_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_byte_list), String::from("`byte$()"));
  q_byte_list=K::new_byte_list(vec![7, 12, 21, 144], qattribute::NONE);
  assert_eq!(format!("{}", q_byte_list), String::from("0x070c1590"));

  // short list
  let mut q_short_list=K::new_short_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_short_list), String::from("`short$()")); 
  q_short_list=K::new_short_list(vec![qnull::SHORT, -7, 12, 21, 144], qattribute::SORTED);
  assert_eq!(format!("{}", q_short_list), String::from("`s#0N -7 12 21 144h"));  

  // int list
  let mut q_int_list=K::new_int_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_int_list), String::from("`int$()"));
  q_int_list=K::new_int_list(vec![-10000, -10000, 21, 21, qinf::INT, 144000], qattribute::PARTED);
  assert_eq!(format!("{}", q_int_list), String::from("`p#-10000 -10000 21 21 0W 144000i"));

  // long list
  let mut q_long_list=K::new_long_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_long_list), String::from("`long$()"));
  q_long_list=K::new_long_list(vec![-86400000000000], qattribute::UNIQUE);
  assert_eq!(format!("{}", q_long_list), String::from("`u#,-86400000000000"));

  // real list
  let mut q_real_list=K::new_real_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_real_list), String::from("`real$()"));
  q_real_list=K::new_real_list(vec![30.2, 5.002], qattribute::NONE);
  assert_eq!(format!("{:.3}", q_real_list), String::from("30.200 5.002e"));

  // float list
  let mut q_float_list=K::new_float_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_float_list), String::from("`float$()"));
  q_float_list=K::new_float_list(vec![100.23, 0.4268, qnull::FLOAT, 15.882, qninf::FLOAT], qattribute::NONE);
  assert_eq!(format!("{}", q_float_list), String::from("100.23 0.4268 0n 15.882 -0w"));

  // string
  let mut q_string=K::new_string(String::from(""), qattribute::NONE);
  assert_eq!(format!("{}", q_string), String::from("\"\""));
  q_string=K::new_string(String::from("super"), qattribute::UNIQUE);
  assert_eq!(format!("{}", q_string), String::from("`u#\"super\""));

  // symbol list
  let mut q_symbol_list=K::new_symbol_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_symbol_list), String::from("`symbol$()"));
  q_symbol_list=K::new_symbol_list(vec![String::from("a"), String::from("b"), String::from("a"), String::from("c")], qattribute::GROUPED);
  assert_eq!(format!("{}", q_symbol_list), String::from("`g#`a`b`a`c"));

  // timestamp list
  let mut q_timestamp_list=K::new_timestamp_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_timestamp_list), String::from("`timestamp$()"));
  q_timestamp_list=K::new_timestamp_list(vec![*qnull::TIMESTAMP, Utc.ymd(2000, 2, 6).and_hms_nano(5, 11, 28, 4032), *qinf::TIMESTAMP], qattribute::NONE);
  assert_eq!(format!("{}", q_timestamp_list), String::from("0N 2000.02.06D05:11:28.000004032 0Wp"));

  // timestamp list2
  let q_timestamp_list2=K::new_timestamp_list(vec![*qninf::TIMESTAMP, Utc.ymd(2000, 2, 6).and_hms_nano(5, 11, 28, 4032)], qattribute::NONE);
  assert_eq!(format!("{}", q_timestamp_list2), String::from("-0W 2000.02.06D05:11:28.000004032"));

  // month list
  let mut q_month_list=K::new_month_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_month_list), String::from("`month$()"));
  q_month_list=K::new_month_list(vec![Utc.ymd(2006, 3, 9), Utc.ymd(1999, 5, 31), qnull::MONTH], qattribute::NONE);
  assert_eq!(format!("{}", q_month_list), String::from("2006.03 1999.05 0Nm"));

  // date list
  let mut q_date_list=K::new_date_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_date_list), String::from("`date$()"));
  q_date_list=K::new_date_list(vec![Utc.ymd(2001, 2, 18), Utc.ymd(2019, 12, 12), qinf::DATE, Utc.ymd(2003, 10, 16)], qattribute::NONE);
  assert_eq!(format!("{}", q_date_list), String::from("2001.02.18 2019.12.12 0W 2003.10.16"));

  // datetime list
  let mut q_datetime_list=K::new_datetime_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_datetime_list), String::from("`datetime$()"));
  q_datetime_list=K::new_datetime_list(vec![Utc.ymd(2002, 1, 26).and_hms_nano(9,39, 2, 368376238), *qinf::DATETIME], qattribute::SORTED);
  assert_eq!(format!("{}", q_datetime_list), String::from("`s#2002.01.26T09:39:02.368 0Wz"));

  // timespan list
  let mut q_timespan_list=K::new_timespan_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_timespan_list), String::from("`timespan$()"));
  q_timespan_list=K::new_timespan_list(vec![*qinf::TIMESPAN, Duration::nanoseconds(7240514990625504), Duration::nanoseconds(-107695363440640000)], qattribute::NONE);
  assert_eq!(format!("{}", q_timespan_list), String::from("0W 83D19:15:14.990625504 -1246D11:22:43.440640000"));

  // minute list
  let mut q_minute_list=K::new_minute_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_minute_list), String::from("`minute$()"));
  q_minute_list=K::new_minute_list(vec![Duration::minutes(504), Duration::seconds(-100)], qattribute::NONE);
  assert_eq!(format!("{}", q_minute_list), String::from("08:24 -00:01"));

  // second list
  let mut q_second_list=K::new_second_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_second_list), String::from("`second$()"));
  q_second_list=K::new_second_list(vec![Duration::seconds(-3554), *qinf::SECOND, Duration::seconds(13744), *qninf::SECOND, *qnull::SECOND], qattribute::NONE);
  assert_eq!(format!("{}", q_second_list), String::from("-00:59:14 0W 03:49:04 -0W 0Nv"));

  // time list
  let mut q_time_list=K::new_time_list(vec![], qattribute::NONE);
  assert_eq!(format!("{}", q_time_list), String::from("`time$()"));
  q_time_list=K::new_time_list(vec![Duration::milliseconds(642982), Duration::milliseconds(789848), *qninf::TIME, Duration::milliseconds(58725553)], qattribute::NONE);
  assert_eq!(format!("{}", q_time_list), String::from("00:10:42.982 00:13:09.848 -0W 16:18:45.553"));

  // compound list
  let mut q_compound_list=K::new_compound_list(vec![]);
  assert_eq!(format!("{}", q_compound_list), String::from("()"));
  q_compound_list=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("Ruby"), String::from("Diamond"), String::from("Sapphire")], qattribute::UNIQUE),
    K::new_timestamp(*qnull::TIMESTAMP),
    K::new_long_list(vec![0, 1, 2, qninf::LONG], qattribute::NONE),
    K::new_month_list(vec![Utc.ymd(2004, 2, 7)], qattribute::NONE)
  ]);
  assert_eq!(format!("{}", q_compound_list), String::from("(`u#`Ruby`Diamond`Sapphire;0Np;0 1 2 -0W;,2004.02m)"));

  // dictionary
  let keys=K::new_int_list(vec![20, 30, 40], qattribute::SORTED);
  let values=K::new_bool_list(vec![false, false, true], qattribute::NONE);
  let q_dictionary=K::new_dictionary(keys, values).unwrap();
  assert_eq!(format!("{}", q_dictionary), String::from("`s#20 30 40i!001b"));

  // table
  let headers=K::new_symbol_list(vec![String::from("fruit"), String::from("price")], qattribute::NONE);
  let columns=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("strawberry"), String::from("orange"), qnull::SYMBOL], qattribute::PARTED),
    K::new_float_list(vec![2.5, 1.25, 117.8], qattribute::NONE)
  ]);
  let q_table=K::new_dictionary(headers, columns).unwrap().flip().unwrap();
  assert_eq!(format!("{:.3}", q_table), String::from("+`fruit`price!(`p#`strawberry`orange`;2.500 1.250 117.800)"));

  // null
  let q_null=K::new_null();
  assert_eq!(format!("{}", q_null), String::from("::"));

  Ok(())
}

#[test]
fn getter_test() -> Result<(), &'static str>{

  // bool
  let q_bool=K::new_bool(true);
  assert_eq!(q_bool.get_bool(), Ok(true));
  assert_eq!(q_bool.get_byte(), Ok(1));
  assert_eq!(q_bool.get_type(), qtype::BOOL_ATOM);
  
  // guid
  let q_guid=K::new_guid([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
  assert_eq!(q_guid.get_guid(), Ok([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]));
  assert_eq!(q_guid.get_bool(), Err("not a bool"));
  assert_eq!(q_guid.get_type(), qtype::GUID_ATOM);

  // byte
  let q_byte=K::new_byte(0x77);
  assert_eq!(q_byte.get_byte(), Ok(0x77));
  assert_eq!(q_byte.get_guid(), Err("not a GUID"));
  assert_eq!(q_byte.get_type(), qtype::BYTE_ATOM);

  // short
  let q_short=K::new_short(-12);
  assert_eq!(q_short.get_short(), Ok(-12));
  assert_eq!(q_short.get_byte(), Err("not a byte compatible"));
  assert_eq!(q_short.get_type(), qtype::SHORT_ATOM);

  // int
  let q_int=K::new_int(144000);
  assert_eq!(q_int.get_int(), Ok(144000));
  assert_eq!(q_int.get_short(), Err("not a short"));
  assert_eq!(q_int.get_type(), qtype::INT_ATOM);

  // long
  let q_long=K::new_long(86400000000000);
  assert_eq!(q_long.get_long(), Ok(86400000000000));
  assert_eq!(q_long.get_int(), Err("not an int compatible"));
  assert_eq!(q_long.get_type(), qtype::LONG_ATOM);

  // real
  let q_real=K::new_real(0.25);
  assert_eq!(q_real.get_real(), Ok(0.25));
  assert_eq!(q_real.get_long(), Err("not a long compatible"));
  assert_eq!(q_real.get_type(), qtype::REAL_ATOM);

  // float
  let q_float=K::new_float(1000.23456);
  assert_eq!(q_float.get_float(), Ok(1000.23456));
  assert_eq!(q_float.get_real(), Err("not a real"));
  assert_eq!(q_float.get_type(), qtype::FLOAT_ATOM);

  // char
  let q_char=K::new_char('C');
  assert_eq!(q_char.get_char(), Ok('C'));
  assert_eq!(q_char.get_byte(), Ok('C' as u8));
  assert_eq!(q_char.get_float(), Err("not a float compatible"));
  assert_eq!(q_char.get_type(), qtype::CHAR);

  // symbol
  let q_symbol=K::new_symbol(String::from("Rust"));
  assert_eq!(q_symbol.get_symbol(), Ok("Rust"));
  assert_eq!(q_symbol.get_char(), Err("not a char"));
  assert_eq!(q_symbol.get_type(), qtype::SYMBOL_ATOM);

  // timestamp
  let q_timestamp=K::new_timestamp(Utc.ymd(2001, 9, 15).and_hms_nano(4, 2, 30, 37204));
  assert_eq!(q_timestamp.get_timestamp(), Ok(Utc.ymd(2001, 9, 15).and_hms_nano(4, 2, 30, 37204)));
  assert_eq!(q_timestamp.get_long(), Ok(53841750000037204));
  assert_eq!(q_timestamp.get_symbol(), Err("neither a symbol nor an error"));
  assert_eq!(q_timestamp.get_type(), qtype::TIMESTAMP_ATOM);

  // timestamp null
  let q_timestamp_null=K::new_timestamp(*qnull::TIMESTAMP);
  assert_eq!(q_timestamp_null.get_timestamp(), Ok(*qnull::TIMESTAMP));
  assert_eq!(q_timestamp_null.get_long(), Ok(qnull::LONG));

  // timestamp inf
  let q_timestamp_inf=K::new_timestamp(*qinf::TIMESTAMP);
  assert_eq!(q_timestamp_inf.get_timestamp(), Ok(*qinf::TIMESTAMP));
  assert_eq!(q_timestamp_inf.get_long(), Ok(qinf::LONG));

  // timestamp ninf
  let q_timestamp_ninf=K::new_timestamp(*qninf::TIMESTAMP);
  assert_eq!(q_timestamp_ninf.get_timestamp(), Ok(*qninf::TIMESTAMP));
  assert_eq!(q_timestamp_ninf.get_long(), Ok(qninf::LONG));

  // month
  let q_month=K::new_month(Utc.ymd(2007, 8, 30));
  assert_eq!(q_month.get_month(), Ok(Utc.ymd(2007, 8, 1)));
  assert_eq!(q_month.get_int(), Ok(91));
  assert_eq!(q_month.get_timestamp(), Err("not a timestamp"));
  assert_eq!(q_month.get_type(), qtype::MONTH_ATOM);

  // month null
  let q_month_null=K::new_month(qnull::MONTH);
  assert_eq!(q_month_null.get_month(), Ok(qnull::MONTH));
  assert_eq!(q_month_null.get_int(), Ok(qnull::INT));

  // month inf
  let q_month_inf=K::new_month(*qinf::MONTH);
  assert_eq!(q_month_inf.get_month(), Ok(*qinf::MONTH));
  assert_eq!(q_month_inf.get_int(), Ok(qinf::INT));

  // month ninf
  let q_month_ninf=K::new_month(*qninf::MONTH);
  assert_eq!(q_month_ninf.get_month(), Ok(*qninf::MONTH));
  assert_eq!(q_month_ninf.get_int(), Ok(qninf::INT));

  // date
  let q_date=K::new_date(Utc.ymd(2000, 5, 10));
  assert_eq!(q_date.get_date(), Ok(Utc.ymd(2000, 5, 10)));
  assert_eq!(q_date.get_int(), Ok(130));
  assert_eq!(q_date.get_month(), Err("not a month"));
  assert_eq!(q_date.get_type(), qtype::DATE_ATOM);

  // date null
  let q_date_null=K::new_date(qnull::DATE);
  assert_eq!(q_date_null.get_date(), Ok(qnull::DATE));
  assert_eq!(q_date_null.get_int(), Ok(qnull::INT));

  // date inf
  let q_date_inf=K::new_date(qinf::DATE);
  assert_eq!(q_date_inf.get_date(), Ok(qinf::DATE));
  assert_eq!(q_date_inf.get_int(), Ok(qinf::INT));

  // date ninf
  let q_date_ninf=K::new_date(*qninf::DATE);
  assert_eq!(q_date_ninf.get_date(), Ok(*qninf::DATE));
  assert_eq!(q_date_ninf.get_int(), Ok(qninf::INT));

  // datetime
  let q_datetime=K::new_datetime(Utc.ymd(2011, 4, 7).and_hms_milli(19, 5, 41, 385));
  assert_eq!(q_datetime.get_datetime(), Ok(Utc.ymd(2011, 4, 7).and_hms_milli(19, 5, 41, 385)));
  assert_eq!(q_datetime.get_float(), Ok(4114.795617881944));
  assert_eq!(q_datetime.get_date(), Err("not a date"));
  assert_eq!(q_datetime.get_type(), qtype::DATETIME_ATOM);

  // datetime null
  let q_datetime_null=K::new_datetime(qnull::DATETIME);
  assert_eq!(q_datetime_null.get_datetime(), Ok(qnull::DATETIME));
  assert!(q_datetime_null.get_float().unwrap().is_nan());

  // datetime inf
  let q_datetime_inf=K::new_datetime(*qinf::DATETIME);
  assert_eq!(q_datetime_inf.get_datetime(), Ok(*qinf::DATETIME));
  assert!(q_datetime_inf.get_float().unwrap().is_infinite());

  // datetime ninf
  let q_datetime_ninf=K::new_datetime(*qninf::DATETIME);
  assert_eq!(q_datetime_ninf.get_datetime(), Ok(*qninf::DATETIME));
  assert!(q_datetime_ninf.get_float().unwrap().is_infinite());
  assert!(q_datetime_ninf.get_float().unwrap().is_sign_negative());

  // timespan
  let q_timespan=K::new_timespan(Duration::nanoseconds(131400000000000));
  assert_eq!(q_timespan.get_timespan(), Ok(Duration::nanoseconds(131400000000000)));
  assert_eq!(q_timespan.get_long(), Ok(131400000000000));
  assert_eq!(q_timespan.get_datetime(), Err("not a datetime"));
  assert_eq!(q_timespan.get_type(), qtype::TIMESPAN_ATOM);

  // timespan null
  let q_timespan_null=K::new_timespan(*qnull::TIMESPAN);
  assert_eq!(q_timespan_null.get_timespan(), Ok(*qnull::TIMESPAN));
  assert_eq!(q_timespan_null.get_long(), Ok(qnull::LONG));

  // timespan inf
  let q_timespan_inf=K::new_timespan(*qinf::TIMESPAN);
  assert_eq!(q_timespan_inf.get_timespan(), Ok(*qinf::TIMESPAN));
  assert_eq!(q_timespan_inf.get_long(), Ok(qinf::LONG));

  // timespan ninf
  let q_timespan_ninf=K::new_timespan(*qninf::TIMESPAN);
  assert_eq!(q_timespan_ninf.get_timespan(), Ok(*qninf::TIMESPAN));
  assert_eq!(q_timespan_ninf.get_long(), Ok(qninf::LONG));

  // minute
  let q_minute=K::new_minute(Duration::minutes(30));
  assert_eq!(q_minute.get_minute(), Ok(Duration::minutes(30)));
  assert_eq!(q_minute.get_int(), Ok(30));
  assert_eq!(q_minute.get_timespan(), Err("not a timespan"));
  assert_eq!(q_minute.get_type(), qtype::MINUTE_ATOM);

  // minute null
  let q_minute_null=K::new_minute(*qnull::MINUTE);
  assert_eq!(q_minute_null.get_minute(), Ok(*qnull::MINUTE));
  assert_eq!(q_minute_null.get_int(), Ok(qnull::INT));

  // minute inf
  let q_minute_inf=K::new_minute(*qinf::MINUTE);
  assert_eq!(q_minute_inf.get_minute(), Ok(*qinf::MINUTE));
  assert_eq!(q_minute_inf.get_int(), Ok(qinf::INT));

  // minute ninf
  let q_minute_ninf=K::new_minute(*qninf::MINUTE);
  assert_eq!(q_minute_ninf.get_minute(), Ok(*qninf::MINUTE));
  assert_eq!(q_minute_ninf.get_int(), Ok(qninf::INT));

  // second
  let q_second=K::new_second(Duration::seconds(30));
  assert_eq!(q_second.get_second(), Ok(Duration::seconds(30)));
  assert_eq!(q_second.get_int(), Ok(30));
  assert_eq!(q_second.get_minute(), Err("not a minute"));
  assert_eq!(q_second.get_type(), qtype::SECOND_ATOM);

  // second null
  let q_second_null=K::new_second(*qnull::SECOND);
  assert_eq!(q_second_null.get_second(), Ok(*qnull::SECOND));
  assert_eq!(q_second_null.get_int(), Ok(qnull::INT));

  // second inf
  let q_second_inf=K::new_second(*qinf::SECOND);
  assert_eq!(q_second_inf.get_second(), Ok(*qinf::SECOND));
  assert_eq!(q_second_inf.get_int(), Ok(qinf::INT));

  // second ninf
  let q_second_ninf=K::new_second(*qninf::SECOND);
  assert_eq!(q_second_ninf.get_second(), Ok(*qninf::SECOND));
  assert_eq!(q_second_ninf.get_int(), Ok(qninf::INT));

  // time
  let q_time=K::new_time(Duration::milliseconds(3000));
  assert_eq!(q_time.get_time(), Ok(Duration::milliseconds(3000)));
  assert_eq!(q_time.get_int(), Ok(3000));
  assert_eq!(q_time.get_second(), Err("not a second"));
  assert_eq!(q_time.get_type(), qtype::TIME_ATOM);

  // time null
  let q_time_null=K::new_time(*qnull::TIME);
  assert_eq!(q_time_null.get_time(), Ok(*qnull::TIME));
  assert_eq!(q_time_null.get_int(), Ok(qnull::INT));

  // time inf
  let q_time_inf=K::new_time(*qinf::TIME);
  assert_eq!(q_time_inf.get_time(), Ok(*qinf::TIME));
  assert_eq!(q_time_inf.get_int(), Ok(qinf::INT));

  // time ninf
  let q_time_ninf=K::new_time(*qninf::TIME);
  assert_eq!(q_time_ninf.get_time(), Ok(*qninf::TIME));
  assert_eq!(q_time_ninf.get_int(), Ok(qninf::INT));

  // table
  let headers=K::new_symbol_list(vec![String::from("fruit"), String::from("price")], qattribute::NONE);
  let columns=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("strawberry"), String::from("orange"), qnull::SYMBOL], qattribute::PARTED),
    K::new_float_list(vec![2.5, 1.25, 117.8], qattribute::NONE)
  ]);
  let q_dictionary=K::new_dictionary(headers, columns).unwrap();
  assert_eq!(q_dictionary.get_type(), qtype::DICTIONARY);
  let q_table=q_dictionary.clone().flip().unwrap();
  match q_table.get_dictionary(){
    Ok(dictionary) => assert_eq!(dictionary.get_type(), qtype::DICTIONARY),
    Err(_) => assert!(false)
  };
  assert_eq!(q_table.get_type(), qtype::TABLE);

  Ok(())
}

#[test]
fn cast_test() -> Result<(), &'static str>{

  // atom
  let atom=K::new_bool(false);
  assert_eq!(atom.as_vec::<G>(), Err("not a list type"));

  // bool list
  let q_bool_list=K::new_bool_list(vec![true, false], qattribute::UNIQUE);
  assert_eq!(*q_bool_list.as_vec::<G>().unwrap(), vec![1_u8, 0]);
  assert_eq!(q_bool_list.get_attribute(), qattribute::UNIQUE);
  assert_eq!(q_bool_list.get_type(), qtype::BOOL_LIST);

  // guid list
  let q_guid_list=K::new_guid_list(vec![[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]], qattribute::NONE);
  assert_eq!(*q_guid_list.as_vec::<U>().unwrap(), vec![[0_u8,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]]);
  assert_eq!(q_guid_list.as_vec::<G>(), Err("failed to cast to specified type"));
  assert_eq!(q_guid_list.get_type(), qtype::GUID_LIST);

  // byte list
  let q_byte_list=K::new_byte_list(vec![7, 12, 21, 144], qattribute::NONE);
  assert_eq!(*q_byte_list.as_vec::<G>().unwrap(), vec![7_u8, 12, 21, 144]);
  assert_eq!(q_byte_list.as_vec::<U>(), Err("failed to cast to specified type"));
  assert_eq!(q_byte_list.get_type(), qtype::BYTE_LIST);

  // short list
  let q_short_list=K::new_short_list(vec![qnull::SHORT, -7, 12, 21, 144], qattribute::SORTED);
  assert_eq!(*q_short_list.as_vec::<H>().unwrap(), vec![qnull::SHORT, -7, 12, 21, 144]);
  assert_eq!(q_short_list.get_type(), qtype::SHORT_LIST);

  // int list
  let q_int_list=K::new_int_list(vec![-10000, -10000, 21, 21, qinf::INT, 144000], qattribute::PARTED);
  assert_eq!(*q_int_list.as_vec::<I>().unwrap(), vec![-10000, -10000, 21, 21, qinf::INT, 144000]);
  assert_eq!(q_int_list.as_vec::<H>(), Err("failed to cast to specified type"));
  assert_eq!(q_int_list.get_type(), qtype::INT_LIST);

  // long list
  let q_long_list=K::new_long_list(vec![-86400000000000], qattribute::UNIQUE);
  assert_eq!(*q_long_list.as_vec::<J>().unwrap(), vec![-86400000000000_i64]);
  assert_eq!(q_long_list.as_vec::<I>(), Err("failed to cast to specified type"));
  assert_eq!(q_long_list.get_type(), qtype::LONG_LIST);

  // real list
  let q_real_list=K::new_real_list(vec![30.2, 5.002], qattribute::NONE);
  assert_eq!(*q_real_list.as_vec::<E>().unwrap(), vec![30.2_f32, 5.002]);
  assert_eq!(q_real_list.as_vec::<J>(), Err("failed to cast to specified type"));
  assert_eq!(q_real_list.get_type(), qtype::REAL_LIST);

  // float list
  let q_float_list=K::new_float_list(vec![100.23, 0.4268, qnull::FLOAT, 15.882, qninf::FLOAT], qattribute::NONE);
  let cast=q_float_list.as_vec::<F>().unwrap();
  assert_eq_float!(cast[0], 100.23, 0.01);
  assert_eq_float!(cast[1], 0.4268, 0.0001);
  assert!(cast[2].is_nan());
  assert_eq_float!(cast[3], 15.882, 0.001);
  assert!(cast[4].is_infinite() && cast[4].is_sign_negative());
  assert_eq!(q_float_list.as_vec::<E>(), Err("failed to cast to specified type"));
  assert_eq!(q_float_list.get_type(), qtype::FLOAT_LIST);

  // string
  let q_string=K::new_string(String::from("super"), qattribute::UNIQUE);
  assert_eq!(*q_string.as_string().unwrap(), String::from("super"));
  assert_eq!(q_string.as_vec::<G>(), Err("not a list type"));
  assert_eq!(q_string.get_type(), qtype::STRING);

  // symbol list
  let q_symbol_list=K::new_symbol_list(vec![String::from("a"), String::from("b"), String::from("a"), String::from("c")], qattribute::GROUPED);
  assert_eq!(*q_symbol_list.as_vec::<S>().unwrap(), vec![String::from("a"), String::from("b"), String::from("a"), String::from("c")]);
  assert_eq!(q_symbol_list.get_type(), qtype::SYMBOL_LIST);

  // string
  let q_string=K::new_string(String::from("super"), qattribute::UNIQUE);
  assert_eq!(*q_string.as_string().unwrap(), String::from("super"));

  // timestamp list
  let q_timestamp_list=K::new_timestamp_list(vec![*qnull::TIMESTAMP, Utc.ymd(2000, 2, 6).and_hms_nano(5, 11, 28, 4032), *qinf::TIMESTAMP], qattribute::NONE);
  assert_eq!(*q_timestamp_list.as_vec::<J>().unwrap(), vec![qnull_base::J, 3129088000004032, qinf_base::J]);
  assert_eq!(q_timestamp_list.get_type(), qtype::TIMESTAMP_LIST);

  // month list
  let q_month_list=K::new_month_list(vec![Utc.ymd(2006, 3, 9), Utc.ymd(1999, 5, 31), qnull::MONTH], qattribute::NONE);
  assert_eq!(*q_month_list.as_vec::<I>().unwrap(), vec![74, -8, qnull_base::I]);
  assert_eq!(q_month_list.get_type(), qtype::MONTH_LIST);

  // date list
  let q_date_list=K::new_date_list(vec![Utc.ymd(2001, 2, 18), Utc.ymd(2019, 12, 12), qinf::DATE, Utc.ymd(2003, 10, 16)], qattribute::NONE);
  assert_eq!(*q_date_list.as_vec::<I>().unwrap(), vec![414, 7285, qinf_base::I, 1384]);
  assert_eq!(q_date_list.get_type(), qtype::DATE_LIST);

  // datetime list
  let q_datetime_list=K::new_datetime_list(vec![Utc.ymd(2002, 1, 26).and_hms_nano(9,39, 2, 368376238), *qinf::DATETIME], qattribute::SORTED);
  let cast=q_datetime_list.as_vec::<F>().unwrap();
  assert_eq_float!(cast[0], 756.4021, 0.0001);
  assert!(cast[1].is_infinite());
  assert_eq!(q_datetime_list.get_type(), qtype::DATETIME_LIST);

  // timespan list
  let q_timespan_list=K::new_timespan_list(vec![*qinf::TIMESPAN, Duration::nanoseconds(7240514990625504), Duration::nanoseconds(-107695363440640000)], qattribute::NONE);
  assert_eq!(*q_timespan_list.as_vec::<J>().unwrap(), vec![qinf_base::J, 7240514990625504, -107695363440640000]);
  assert_eq!(q_timespan_list.get_type(), qtype::TIMESPAN_LIST);

  // minute list
  let q_minute_list=K::new_minute_list(vec![Duration::minutes(504), Duration::seconds(-100)], qattribute::NONE);
  assert_eq!(*q_minute_list.as_vec::<I>().unwrap(), vec![504, -1]);
  assert_eq!(q_minute_list.get_type(), qtype::MINUTE_LIST);

  // second list
  let q_second_list=K::new_second_list(vec![Duration::seconds(-3554), *qinf::SECOND, Duration::seconds(13744), *qninf::SECOND, *qnull::SECOND], qattribute::NONE);
  assert_eq!(*q_second_list.as_vec::<I>().unwrap(), vec![-3554, qinf_base::I, 13744, qninf_base::I, qnull_base::I]);
  assert_eq!(q_second_list.get_type(), qtype::SECOND_LIST);

  // time list
  let q_time_list=K::new_time_list(vec![Duration::milliseconds(642982), Duration::milliseconds(789848), *qninf::TIME, Duration::milliseconds(58725553)], qattribute::NONE);
  assert_eq!(*q_time_list.as_vec::<I>().unwrap(), vec![642982, 789848, qninf_base::I, 58725553]);
  assert_eq!(q_time_list.get_type(), qtype::TIME_LIST);

  // compound list
  let q_compound_list=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("Ruby"), String::from("Diamond"), String::from("Sapphire")], qattribute::UNIQUE),
    K::new_timestamp(*qnull::TIMESTAMP),
    K::new_long_list(vec![0, 1, 2, qninf::LONG], qattribute::NONE),
    K::new_month_list(vec![Utc.ymd(2004, 2, 7)], qattribute::NONE)
  ]);
  match q_compound_list.as_vec::<K>(){
    Ok(list) => assert_eq!(list.len(), 4),
    Err(_) => assert!(false)
  };
  assert_eq!(q_compound_list.get_type(), qtype::COMPOUND_LIST);

  // dictionary
  let keys=K::new_int_list(vec![20, 30, 40], qattribute::SORTED);
  let values=K::new_bool_list(vec![false, false, true], qattribute::NONE);
  let q_dictionary=K::new_dictionary(keys, values).unwrap();
  match q_dictionary.as_vec::<K>(){
    Ok(list) => assert_eq!(list.len(), 2),
    Err(_) => assert!(false)
  };

  Ok(())
}

#[test]
fn length_test() -> Result<(), &'static str>{

  // atom
  let q_bool=K::new_bool(true);
  assert_eq!(q_bool.len(), 1);

  // guid
  let q_guid=K::new_guid([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
  assert_eq!(q_guid.len(), 1);

  // symbol
  let q_symbol=K::new_symbol(String::from("soup"));
  assert_eq!(q_symbol.len(), 1);

  // string
  let q_string=K::new_string(String::from("soup"), qattribute::NONE);
  assert_eq!(q_string.len(), 4);

  // list
  let q_symbol_list=K::new_symbol_list(vec![String::from("a"), String::from("b")], qattribute::NONE);
  assert_eq!(q_symbol_list.len(), 2);

  // compound list
  let q_compound_list=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("Ruby"), String::from("Diamond"), String::from("Sapphire")], qattribute::UNIQUE),
    K::new_timestamp(*qnull::TIMESTAMP),
    K::new_long_list(vec![0, 1, 2, qninf::LONG], qattribute::NONE),
    K::new_month_list(vec![Utc.ymd(2004, 2, 7)], qattribute::NONE)
  ]);
  assert_eq!(q_compound_list.len(), 4);
  
  // dictionary
  let headers=K::new_symbol_list(vec![String::from("fruit"), String::from("price")], qattribute::NONE);
  let columns=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("strawberry"), String::from("orange"), qnull::SYMBOL], qattribute::PARTED),
    K::new_float_list(vec![2.5, 1.25, 117.8], qattribute::NONE)
  ]);
  let q_dictionary=K::new_dictionary(headers, columns).unwrap();
  assert_eq!(q_dictionary.len(), 2);

  // table
  let q_table=q_dictionary.flip().unwrap();
  assert_eq!(q_table.len(), 3);

  // null
  let q_null=K::new_null();
  assert_eq!(q_null.len(), 1);

  Ok(())

}

#[test]
fn push_pop_test() -> Result<(), &'static str>{

  // empty list
  let mut q_empty_list=K::new_bool_list(Vec::<bool>::new(), qattribute::NONE);
  match q_empty_list.pop(){
    Ok(_) => assert!(false),
    Err(error) => assert_eq!(error, "no element in this object")
  };
  match q_empty_list.remove(0){
    Ok(_) => assert!(false),
    Err(error) => assert_eq!(error, "index out of bounds")
  };

  // bool list
  let mut q_bool_list=K::new_bool_list(vec![false], qattribute::NONE);
  q_bool_list.push(&true).unwrap();
  q_bool_list.insert(1, &false).unwrap();
  assert_eq!(*q_bool_list.as_vec::<G>().unwrap(), vec![0_u8, 0, 1]);
  let mut tail_bool=q_bool_list.pop_bool().unwrap();
  assert_eq!(tail_bool, true);
  let mut tail=q_bool_list.pop().unwrap();
  assert_eq!(tail.get_bool().unwrap(), false);
  tail_bool=q_bool_list.remove_bool(0).unwrap();
  assert_eq!(tail_bool, false);
  q_bool_list.push(&true).unwrap();
  tail=q_bool_list.remove(0).unwrap();
  assert_eq!(tail.get_bool().unwrap(), true);

  // guid list
  let mut q_guid_list=K::new_guid_list(vec![[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]], qattribute::NONE);
  q_guid_list.push(&[1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]).unwrap();
  q_guid_list.insert(1, &[2_u8,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]).unwrap();
  assert_eq!(*q_guid_list.as_vec::<U>().unwrap(), vec![[0_u8,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15], [2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17], [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]]);
  let mut tail_guid=q_guid_list.pop_guid().unwrap();
  assert_eq!(tail_guid, [1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
  tail=q_guid_list.pop().unwrap();
  assert_eq!(tail.get_guid().unwrap(), [2_u8,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]);
  tail_guid=q_guid_list.remove_guid(0).unwrap();
  assert_eq!(tail_guid, [0_u8,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
  q_guid_list.push(&[1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]).unwrap();
  tail=q_guid_list.remove(0).unwrap();
  assert_eq!(tail.get_guid().unwrap(), [1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);

  // byte list
  let mut q_byte_list=K::new_byte_list(vec![0x77], qattribute::NONE);
  q_byte_list.push(&0x99_u8).unwrap();
  println!("len: {}", q_byte_list.len());
  q_byte_list.insert(2, &0xae_u8).unwrap();
  assert_eq!(*q_byte_list.as_vec::<G>().unwrap(), vec![0x77, 0x99, 0xae]);
  let mut tail_byte=q_byte_list.pop_byte().unwrap();
  assert_eq!(tail_byte, 0xae_u8);
  tail=q_byte_list.pop().unwrap();
  assert_eq!(tail.get_byte().unwrap(), 0x99_u8);
  tail_byte=q_byte_list.remove_byte(0).unwrap();
  assert_eq!(tail_byte, 0x77_u8);
  q_byte_list.push(&0x99_u8).unwrap();
  tail=q_byte_list.remove(0).unwrap();
  assert_eq!(tail.get_byte().unwrap(), 0x99_u8);

  // short list
  let mut q_short_list=K::new_short_list(vec![12], qattribute::NONE);
  q_short_list.push(&-2000_i16).unwrap();
  q_short_list.insert(1, &50_i16).unwrap();
  assert_eq!(*q_short_list.as_vec::<H>().unwrap(), vec![12_i16, 50, -2000]);
  let mut tail_short=q_short_list.pop_short().unwrap();
  assert_eq!(tail_short, -2000_i16);
  tail=q_short_list.pop().unwrap();
  assert_eq!(tail.get_short().unwrap(), 50_i16);
  tail_short=q_short_list.remove_short(0).unwrap();
  assert_eq!(tail_short, 12_i16);
  q_short_list.push(&-1000_i16).unwrap();
  tail=q_short_list.remove(0).unwrap();
  assert_eq!(tail.get_short().unwrap(), -1000_i16);

  // int list
  let mut q_int_list=K::new_int_list(vec![144000], qattribute::NONE);
  q_int_list.push(&888).unwrap();
  q_int_list.insert(1, &-1).unwrap();
  assert_eq!(*q_int_list.as_vec::<I>().unwrap(), vec![144000, -1, 888]);
  let mut tail_int=q_int_list.pop_int().unwrap();
  assert_eq!(tail_int, 888);
  tail=q_int_list.pop().unwrap();
  assert_eq!(tail.get_int().unwrap(), -1);
  tail_int=q_int_list.remove_int(0).unwrap();
  assert_eq!(tail_int, 144000);
  q_int_list.push(&20).unwrap();
  tail=q_int_list.remove(0).unwrap();
  assert_eq!(tail.get_int().unwrap(), 20);

  // long list
  let mut q_long_list=K::new_long_list(vec![-86400], qattribute::NONE);
  q_long_list.push(&13800000000_i64).unwrap();
  q_long_list.insert(2, &-12600_i64).unwrap();
  assert_eq!(*q_long_list.as_vec::<J>().unwrap(), vec![-86400_i64, 13800000000, -12600]);
  let mut tail_long=q_long_list.pop_long().unwrap();
  assert_eq!(tail_long, -12600_i64);
  tail=q_long_list.pop().unwrap();
  assert_eq!(tail.get_long().unwrap(), 13800000000_i64);
  tail_long=q_long_list.remove_long(0).unwrap();
  assert_eq!(tail_long, -86400_i64);
  q_long_list.push(&12000_i64).unwrap();
  tail=q_long_list.remove(0).unwrap();
  assert_eq!(tail.get_long().unwrap(), 12000_i64);

  // real list
  let mut q_real_list=K::new_real_list(vec![9.22], qattribute::NONE);
  q_real_list.push(&-20.44_f32).unwrap();
  q_real_list.insert(1, &-0.1_f32).unwrap();
  assert_eq!(*q_real_list.as_vec::<E>().unwrap(), vec![9.22_f32, -0.1, -20.44]);
  let mut tail_real=q_real_list.pop_real().unwrap();
  assert_eq!(tail_real, -20.44_f32);
  tail=q_real_list.pop().unwrap();
  assert_eq!(tail.get_real().unwrap(), -0.1_f32);
  tail_real=q_real_list.remove_real(0).unwrap();
  assert_eq!(tail_real, 9.22_f32);
  q_real_list.push(&0.33_f32).unwrap();
  tail=q_real_list.remove(0).unwrap();
  assert_eq!(tail.get_real().unwrap(), 0.33_f32);

  // float list
  let mut q_float_list=K::new_float_list(vec![5634.7666], qattribute::NONE);
  q_float_list.push(&120.45).unwrap();
  q_float_list.insert(2, &1001.3).unwrap();
  assert_eq!(*q_float_list.as_vec::<F>().unwrap(), vec![5634.7666, 120.45, 1001.3]);
  let mut tail_float=q_float_list.pop_float().unwrap();
  assert_eq_float!(tail_float, 1001.3, 0.1);
  tail=q_float_list.pop().unwrap();
  assert_eq_float!(tail.get_float().unwrap(), 120.45, 0.01);
  tail_float=q_float_list.remove_float(0).unwrap();
  assert_eq!(tail_float, 5634.7666);
  q_float_list.push(&0.125).unwrap();
  tail=q_float_list.remove(0).unwrap();
  assert_eq!(tail.get_float().unwrap(), 0.125);

  // string
  let mut q_string=K::new_string(String::from("boring test"), qattribute::NONE);
  q_string.push(&'?').unwrap();
  q_string.insert(6, &'*').unwrap();
  assert_eq!(*q_string.as_string().unwrap(), String::from("boring* test?"));
  let mut tail_char=q_string.pop_char().unwrap();
  assert_eq!(tail_char, '?');
  tail=q_string.pop().unwrap();
  assert_eq!(tail.get_char().unwrap(), 't');
  tail_char=q_string.remove_char(10).unwrap();
  assert_eq!(tail_char, 's');
  q_string.push(&'a').unwrap();
  tail=q_string.remove(3).unwrap();
  assert_eq!(tail.get_char().unwrap(), 'i');

  // symbol list
  let mut q_symbol_list=K::new_symbol_list(vec![String::from("almond")], qattribute::NONE);
  q_symbol_list.push(&String::from("hazel")).unwrap();
  q_symbol_list.insert(1, &String::from("macadamia")).unwrap();
  assert_eq!(*q_symbol_list.as_vec::<S>().unwrap(), vec![String::from("almond"), String::from("macadamia"), String::from("hazel")]);
  let mut tail_symbol=q_symbol_list.pop_symbol().unwrap();
  assert_eq!(tail_symbol, String::from("hazel"));
  tail=q_symbol_list.pop().unwrap();
  assert_eq!(tail.get_symbol().unwrap(), "macadamia");
  tail_symbol=q_symbol_list.remove_symbol(0).unwrap();
  assert_eq!(tail_symbol, String::from("almond"));
  q_symbol_list.push(&String::from("pistachio")).unwrap();
  tail=q_symbol_list.remove(0).unwrap();
  assert_eq!(tail.get_symbol().unwrap(), String::from("pistachio"));

  // timestamp list
  let mut q_timestamp_list=K::new_timestamp_list(vec![Utc.ymd(2019, 8, 9).and_hms_nano(16, 28, 2, 468276775)], qattribute::NONE);
  q_timestamp_list.push(&Utc.ymd(2003, 12, 9).and_hms_nano(19, 58, 30, 326987333)).unwrap();
  q_timestamp_list.insert(0, &Utc.ymd(2001, 2, 18).and_hms_nano(0, 39, 8, 429879532)).unwrap();
  assert_eq!(*q_timestamp_list.as_vec::<J>().unwrap(), vec![35771948429879532, 618683282468276775, 124315110326987333]);
  let mut tail_timestamp=q_timestamp_list.pop_timestamp().unwrap();
  assert_eq!(tail_timestamp, Utc.ymd(2003, 12, 9).and_hms_nano(19, 58, 30, 326987333));
  tail=q_timestamp_list.pop().unwrap();
  assert_eq!(tail.get_timestamp().unwrap(), Utc.ymd(2019, 8, 9).and_hms_nano(16, 28, 2, 468276775));
  tail_timestamp=q_timestamp_list.remove_timestamp(0).unwrap();
  assert_eq!(tail_timestamp, Utc.ymd(2001, 2, 18).and_hms_nano(0, 39, 8, 429879532));
  q_timestamp_list.push(&Utc.ymd(2003, 12, 9).and_hms_nano(19, 58, 30, 326987333)).unwrap();
  tail=q_timestamp_list.remove(0).unwrap();
  assert_eq!(tail.get_timestamp().unwrap(), Utc.ymd(2003, 12, 9).and_hms_nano(19, 58, 30, 326987333));

  // month list
  let mut q_month_list=K::new_month_list(vec![Utc.ymd(2011, 5, 1)], qattribute::NONE);
  q_month_list.push(&Utc.ymd(2004, 8, 1)).unwrap();
  q_month_list.insert(2, &Utc.ymd(2013, 5, 1)).unwrap();
  assert_eq!(*q_month_list.as_vec::<I>().unwrap(), vec![136, 55, 160]);
  let mut tail_month=q_month_list.pop_month().unwrap();
  assert_eq!(tail_month, Utc.ymd(2013, 5, 1));
  tail=q_month_list.pop().unwrap();
  assert_eq!(tail.get_month().unwrap(), Utc.ymd(2004, 8, 1));
  tail_month=q_month_list.remove_month(0).unwrap();
  assert_eq!(tail_month, Utc.ymd(2011, 5, 1));
  q_month_list.push(&Utc.ymd(2003, 12, 1)).unwrap();
  tail=q_month_list.remove(0).unwrap();
  assert_eq!(tail.get_month().unwrap(), Utc.ymd(2003, 12, 1));

  // date list
  let mut q_date_list=K::new_date_list(vec![Utc.ymd(2014, 6, 4)], qattribute::NONE);
  q_date_list.push(&Utc.ymd(2013, 7, 3)).unwrap();
  q_date_list.insert(2, &Utc.ymd(2007, 1, 1)).unwrap();
  assert_eq!(*q_date_list.as_vec::<I>().unwrap(), vec![5268, 4932, 2557]);
  let mut tail_date=q_date_list.pop_date().unwrap();
  assert_eq!(tail_date, Utc.ymd(2007, 1, 1));
  tail=q_date_list.pop().unwrap();
  assert_eq!(tail.get_date().unwrap(), Utc.ymd(2013, 7, 3));
  tail_date=q_date_list.remove_date(0).unwrap();
  assert_eq!(tail_date, Utc.ymd(2014, 6, 4));
  q_date_list.push(&Utc.ymd(2003, 12, 9)).unwrap();
  tail=q_date_list.remove(0).unwrap();
  assert_eq!(tail.get_date().unwrap(), Utc.ymd(2003, 12, 9));

  // datetime list
  let mut q_datetime_list=K::new_datetime_list(vec![Utc.ymd(2018, 9, 22).and_hms_milli(4, 58, 30, 204)], qattribute::NONE);
  q_datetime_list.push(&Utc.ymd(2017, 8, 9).and_hms_milli(15, 32, 19, 600)).unwrap();
  q_datetime_list.insert(0, &Utc.ymd(1997, 6, 23).and_hms_milli(13, 8, 57, 654)).unwrap();
  assert_eq_float_vec!(*q_datetime_list.as_vec::<F>().unwrap(), vec![-921.4521_f64, 6839.207, 6430.647], 0.001);
  let mut tail_datetime=q_datetime_list.pop_datetime().unwrap();
  assert_eq!(tail_datetime, Utc.ymd(2017, 8, 9).and_hms_milli(15, 32, 19, 600));
  tail=q_datetime_list.pop().unwrap();
  assert_eq!(tail.get_datetime().unwrap(), Utc.ymd(2018, 9, 22).and_hms_milli(4, 58, 30, 204));
  tail_datetime=q_datetime_list.remove_datetime(0).unwrap();
  assert_eq!(tail_datetime, Utc.ymd(1997, 6, 23).and_hms_milli(13, 8, 57, 654));
  q_datetime_list.push(&Utc.ymd(2003, 12, 9).and_hms_milli(19, 58, 30, 326)).unwrap();
  tail=q_datetime_list.remove(0).unwrap();
  assert_eq!(tail.get_datetime().unwrap(), Utc.ymd(2003, 12, 9).and_hms_milli(19, 58, 30, 326));

  // timespan list
  let mut q_timespan_list=K::new_timespan_list(vec![Duration::nanoseconds(6782392639932)], qattribute::NONE);
  q_timespan_list.push(&Duration::nanoseconds(219849398328832)).unwrap();
  q_timespan_list.insert(1, &Duration::nanoseconds(57994284443)).unwrap();
  assert_eq!(*q_timespan_list.as_vec::<J>().unwrap(), vec![6782392639932_i64, 57994284443, 219849398328832]);
  let mut tail_timespan=q_timespan_list.pop_timespan().unwrap();
  assert_eq!(tail_timespan, Duration::nanoseconds(219849398328832_i64));
  tail=q_timespan_list.pop().unwrap();
  assert_eq!(tail.get_timespan().unwrap(), Duration::nanoseconds(57994284443_i64));
  tail_timespan=q_timespan_list.remove_timespan(0).unwrap();
  assert_eq!(tail_timespan, Duration::nanoseconds(6782392639932_i64));
  q_timespan_list.push(&Duration::nanoseconds(-232587934)).unwrap();
  tail=q_timespan_list.remove(0).unwrap();
  assert_eq!(tail.get_timespan().unwrap(), Duration::nanoseconds(-232587934));

  // minute list
  let mut q_minute_list=K::new_minute_list(vec![Duration::minutes(1024)], qattribute::NONE);
  q_minute_list.push(&Duration::minutes(-503)).unwrap();
  q_minute_list.insert(0, &Duration::minutes(12)).unwrap();
  assert_eq!(*q_minute_list.as_vec::<I>().unwrap(), vec![12, 1024, -503]);
  let mut tail_minute=q_minute_list.pop_minute().unwrap();
  assert_eq!(tail_minute, Duration::minutes(-503));
  tail=q_minute_list.pop().unwrap();
  assert_eq!(tail.get_minute().unwrap(), Duration::minutes(1024));
  tail_minute=q_minute_list.remove_minute(0).unwrap();
  assert_eq!(tail_minute, Duration::minutes(12_i64));
  q_minute_list.push(&Duration::minutes(-20)).unwrap();
  tail=q_minute_list.remove(0).unwrap();
  assert_eq!(tail.get_minute().unwrap(), Duration::minutes(-20));

  // second list
  let mut q_second_list=K::new_second_list(vec![Duration::seconds(-32467)], qattribute::NONE);
  q_second_list.push(&Duration::seconds(73984)).unwrap();
  q_second_list.insert(2, &Duration::seconds(140)).unwrap();
  assert_eq!(*q_second_list.as_vec::<I>().unwrap(), vec![-32467, 73984, 140]);
  let mut tail_second=q_second_list.pop_second().unwrap();
  assert_eq!(tail_second, Duration::seconds(140));
  tail=q_second_list.pop().unwrap();
  assert_eq!(tail.get_second().unwrap(), Duration::seconds(73984));
  tail_second=q_second_list.remove_second(0).unwrap();
  assert_eq!(tail_second, Duration::seconds(-32467));
  q_second_list.push(&Duration::seconds(-2934)).unwrap();
  tail=q_second_list.remove(0).unwrap();
  assert_eq!(tail.get_second().unwrap(), Duration::seconds(-2934));

  // time list
  let mut q_time_list=K::new_time_list(vec![Duration::milliseconds(902467)], qattribute::NONE);
  q_time_list.push(&Duration::milliseconds(23134)).unwrap();
  q_time_list.insert(1, &Duration::milliseconds(-30576)).unwrap();
  assert_eq!(*q_time_list.as_vec::<I>().unwrap(), vec![902467, -30576, 23134]);
  let mut tail_time=q_time_list.pop_time().unwrap();
  assert_eq!(tail_time, Duration::milliseconds(23134));
  tail=q_time_list.pop().unwrap();
  assert_eq!(tail.get_time().unwrap(), Duration::milliseconds(-30576));
  tail_time=q_time_list.remove_time(0).unwrap();
  assert_eq!(tail_time, Duration::milliseconds(902467_i64));
  q_time_list.push(&Duration::milliseconds(-23587934)).unwrap();
  tail=q_time_list.remove(0).unwrap();
  assert_eq!(tail.get_time().unwrap(), Duration::milliseconds(-23587934));

  // compound list
  let mut q_compound_list=K::new_compound_list(vec![K::new_long_list(vec![10, 20], qattribute::SORTED)]);
  q_compound_list.push(&K::new_compound_list(vec![K::new_string(String::from("complex"), qattribute::NONE), K::new_symbol(String::from("multitude"))])).unwrap();
  assert_eq!(format!("{}", q_compound_list), String::from("(`s#10 20;(\"complex\";`multitude))"));
  tail=q_compound_list.pop().unwrap();
  assert_eq!(format!("{}", tail), String::from("(\"complex\";`multitude)"));
  tail=q_compound_list.remove(0).unwrap();
  assert_eq!(format!("{}", tail), String::from("`s#10 20"));

  // dictionary
  assert_eq!(q_compound_list.push_pair(&3, &String::from("woops")), Err("not a dictionary"));

  let keys=K::new_int_list(vec![0, 1, 2], qattribute::NONE);
  let values=K::new_date_list(vec![Utc.ymd(2000, 1, 9), Utc.ymd(2001, 4, 10), Utc.ymd(2015, 3, 16)], qattribute::NONE);
  let mut q_dictionary=K::new_dictionary(keys, values).unwrap();

  // Try to insert wrong type element
  let _=q_dictionary.push_pair(&3, &String::from("woops"));
  assert_eq!(format!("{}", q_dictionary), String::from("0 1 2i!2000.01.09 2001.04.10 2015.03.16"));

  // Add correct type element
  q_dictionary.push_pair(&3, &Utc.ymd(2020, 8, 9)).unwrap();
  assert_eq!(format!("{}", q_dictionary), String::from("0 1 2 3i!2000.01.09 2001.04.10 2015.03.16 2020.08.09"));

  let (key, value) = q_dictionary.pop_pair().unwrap();
  assert_eq!(key.get_int().unwrap(), 3);
  assert_eq!(value.get_date().unwrap(), Utc.ymd(2020, 8, 9));

  Ok(())
}

#[async_std::test]
async fn functional_message_test(socket:&mut Qsocket) -> Result<(), Box<dyn std::error::Error>>{

  // Connect to q process
  let mut socket=QStream::connect(ConnectionMethod::TCP, "localhost", 5000, "kdbuser:pass").await.expect("Failed to connect");
  
  // bool
  let mut res_bool=socket.send_sync_message(&add_null!(K::new_bool(true))).await?;
  assert_eq!(res_bool.get_bool()?, true);

  // GUID
  let mut res_guid=socket.send_sync_message(&add_null!(K::new_guid([0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]))).await?;
  assert_eq!(res_guid.get_guid()?, [0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]);

  // GUID null
  res_guid=socket.send_sync_message(&add_null!(K::new_guid(qnull::GUID))).await?;
  assert_eq!(res_guid.get_guid()?, qnull::GUID);

  // byte
  let mut res_byte=socket.send_sync_message(&add_null!(K::new_byte(0x77))).await?;
  assert_eq!(res_byte.get_byte()?, 0x77);

  // short
  let mut res_short=socket.send_sync_message(&add_null!(K::new_short(17))).await?;
  assert_eq!(res_short.get_short()?, 17_i16);

  // short null
  res_short=socket.send_sync_message(&add_null!(K::new_short(qnull::SHORT))).await?;
  assert_eq!(res_short.get_short()?, qnull::SHORT);

  // short inf
  res_short=socket.send_sync_message(&add_null!(K::new_short(qinf::SHORT))).await?;
  assert_eq!(res_short.get_short()?, qinf::SHORT);

  // short ninf
  res_short=socket.send_sync_message(&add_null!(K::new_short(qninf::SHORT))).await?;
  assert_eq!(res_short.get_short()?, qninf::SHORT);

  // int
  let mut res_int=socket.send_sync_message(&add_null!(K::new_int(-34567789))).await?;
  assert_eq!(res_int.get_int()?, -34567789);

  // int null
  res_int=socket.send_sync_message(&add_null!(K::new_int(qnull::INT))).await?;
  assert_eq!(res_int.get_int()?, qnull::INT);

  // int inf
  res_int=socket.send_sync_message(&add_null!(K::new_int(qinf::INT))).await?;
  assert_eq!(res_int.get_int()?, qinf::INT);

  // int ninf
  res_int=socket.send_sync_message(&add_null!(K::new_int(qninf::INT))).await?;
  assert_eq!(res_int.get_int()?, qninf::INT);

  // long
  let mut res_long=socket.send_sync_message(&add_null!(K::new_long(86400000000000_i64))).await?;
  assert_eq!(res_long.get_long()?, 86400000000000_i64);

  // long null
  res_long=socket.send_sync_message(&add_null!(K::new_long(qnull::LONG))).await?;
  assert_eq!(res_long.get_long()?, qnull::LONG);

  // long inf
  res_long=socket.send_sync_message(&add_null!(K::new_long(qinf::LONG))).await?;
  assert_eq!(res_long.get_long()?, qinf::LONG);

  // long ninf
  res_long=socket.send_sync_message(&add_null!(K::new_long(qninf::LONG))).await?;
  assert_eq!(res_long.get_long()?, qninf::LONG);

  // real
  let mut res_real=socket.send_sync_message(&add_null!(K::new_real(10.25_f32))).await?;
  assert_eq!(res_real.get_real()?, 10.25_f32);

  // real null
  res_real=socket.send_sync_message(&add_null!(K::new_real(qnull::REAL))).await?;
  assert!(res_real.get_real()?.is_nan());

  // real inf
  res_real=socket.send_sync_message(&add_null!(K::new_real(qinf::REAL))).await?;
  assert!(res_real.get_real()?.is_infinite());

  // real ninf
  res_real=socket.send_sync_message(&add_null!(K::new_real(qninf::REAL))).await?;
  assert!(res_real.get_real()?.is_infinite() && res_real.get_real()?.is_sign_negative());

  // float
  let mut res_float=socket.send_sync_message(&add_null!(K::new_float(103.678))).await?;
  assert_eq_float!(res_float.get_float().expect("Failed to convert into f64"), 103.678, 0.001);

  // float null
  res_float=socket.send_sync_message(&add_null!(K::new_float(qnull::FLOAT))).await?;
  assert!(res_float.get_float().expect("Failed to convert into f64").is_nan());

  // float inf
  res_float=socket.send_sync_message(&add_null!(K::new_float(qinf::FLOAT))).await?;
  assert!(res_float.get_float().expect("Failed to convert into f64").is_infinite());

  // float ninf
  res_float=socket.send_sync_message(&add_null!(K::new_float(qninf::FLOAT))).await?;
  assert!(res_float.get_float().expect("Failed to convert into f64").is_infinite() && res_float.get_float().expect("Failed to convert into f64").is_sign_negative());

  // char 
  let mut res_char=socket.send_sync_message(&add_null!(K::new_char('q'))).await?;
  assert_eq!(res_char.get_char()?, 'q');

  // char null
  res_char=socket.send_sync_message(&add_null!(K::new_char(qnull::CHAR))).await?;
  assert_eq!(res_char.get_char()?, ' ');

  // symbol 
  let mut res_symbol=socket.send_sync_message(&add_null!(K::new_symbol(String::from("kdb+")))).await?;
  assert_eq!(res_symbol.get_symbol()?, "kdb+");

  // symbol null
  res_symbol=socket.send_sync_message(&add_null!(K::new_symbol(qnull::SYMBOL))).await?;
  assert_eq!(res_symbol.get_symbol()?, qnull::SYMBOL);

  // timestamp
  let mut res_timestamp=socket.send_sync_message(&add_null!(K::new_timestamp(Utc.ymd(2018, 2, 18).and_hms_nano(4, 0, 0, 100)))).await?;
  assert_eq!(res_timestamp.get_timestamp()?, Utc.ymd(2018, 2, 18).and_hms_nano(4, 0, 0, 100));

  // timestamp null
  res_timestamp=socket.send_sync_message(&add_null!(K::new_timestamp(*qnull::TIMESTAMP))).await?;
  assert_eq!(res_timestamp.get_timestamp()?, *qnull::TIMESTAMP);

  // timestamp inf
  res_timestamp=socket.send_sync_message(&add_null!(K::new_timestamp(*qinf::TIMESTAMP))).await?;
  assert_eq!(res_timestamp.get_timestamp()?, *qinf::TIMESTAMP);

  // timestamp ninf
  res_timestamp=socket.send_sync_message(&add_null!(K::new_timestamp(*qninf::TIMESTAMP))).await?;
  assert_eq!(res_timestamp.get_timestamp()?, *qninf::TIMESTAMP);

  // month   
  let mut res_month=socket.send_sync_message(&add_null!(K::new_month(Utc.ymd(2013, 9, 1)))).await?;
  assert_eq!(res_month.get_month()?, Utc.ymd(2013, 9, 1));

  // month null
  res_month=socket.send_sync_message(&add_null!(K::new_month(qnull::MONTH))).await?;
  assert_eq!(res_month.get_month()?, qnull::MONTH);

  // month inf
  res_month=socket.send_sync_message(&add_null!(K::new_month(*qinf::MONTH))).await?;
  assert_eq!(res_month.get_month()?, *qinf::MONTH);

  // month ninf
  res_month=socket.send_sync_message(&add_null!(K::new_month(*qninf::MONTH))).await?;
  assert_eq!(res_month.get_month()?, *qninf::MONTH);

  // date 
  let mut res_date=socket.send_sync_message(&add_null!(K::new_date(Utc.ymd(2000, 2, 9)))).await?;
  assert_eq!(res_date.get_date()?, Utc.ymd(2000, 2, 9));

  // date null
  res_date=socket.send_sync_message(&add_null!(K::new_date(qnull::DATE))).await?;
  assert_eq!(res_date.get_date()?, qnull::DATE);

  // date inf
  res_date=socket.send_sync_message(&add_null!(K::new_date(qinf::DATE))).await?;
  assert_eq!(res_date.get_date()?, qinf::DATE);

  // date ninf
  res_date=socket.send_sync_message(&add_null!(K::new_date(*qninf::DATE))).await?;
  assert_eq!(res_date.get_date()?, *qninf::DATE);

  // datetime
  let mut res_datetime=socket.send_sync_message(&add_null!(K::new_datetime(Utc.ymd(2004, 6, 17).and_hms_milli(11, 32, 40, 803)))).await?;
  assert_eq!(res_datetime.get_datetime()?, Utc.ymd(2004, 6, 17).and_hms_milli(11, 32, 40, 803));

  // datetime null
  res_datetime=socket.send_sync_message(&add_null!(K::new_datetime(qnull::DATETIME))).await?;
  assert_eq!(res_datetime.get_datetime()?, qnull::DATETIME);

  // datetime inf
  res_datetime=socket.send_sync_message(&add_null!(K::new_datetime(*qinf::DATETIME))).await?;
  assert_eq!(res_datetime.get_datetime()?, *qinf::DATETIME);

  // datetime ninf
  res_datetime=socket.send_sync_message(&add_null!(K::new_datetime(*qninf::DATETIME))).await?;
  assert_eq!(res_datetime.get_datetime()?, *qninf::DATETIME);

  // timespan
  let mut res_timespan=socket.send_sync_message(&add_null!(K::new_timespan(Duration::nanoseconds(999000000)))).await?;
  assert_eq!(res_timespan.get_timespan()?, Duration::nanoseconds(999000000));

  // timespan null
  res_timespan=socket.send_sync_message(&add_null!(K::new_timespan(*qnull::TIMESPAN))).await?;
  assert_eq!(res_timespan.get_timespan()?, *qnull::TIMESPAN);

  // timespan inf
  res_timespan=socket.send_sync_message(&add_null!(K::new_timespan(*qinf::TIMESPAN))).await?;
  assert_eq!(res_timespan.get_timespan()?, *qinf::TIMESPAN);

  // timespan ninf
  res_timespan=socket.send_sync_message(&add_null!(K::new_timespan(*qninf::TIMESPAN))).await?;
  assert_eq!(res_timespan.get_timespan()?, *qninf::TIMESPAN);

  // minute
  let mut res_minute=socket.send_sync_message(&add_null!(K::new_minute(Duration::minutes(1231)))).await?;
  assert_eq!(res_minute.get_minute()?, Duration::minutes(1231));

  // minute null
  res_minute=socket.send_sync_message(&add_null!(K::new_minute(*qnull::MINUTE))).await?;
  assert_eq!(res_minute.get_minute()?, *qnull::MINUTE);

  // minute inf
  res_minute=socket.send_sync_message(&add_null!(K::new_minute(*qinf::MINUTE))).await?;
  assert_eq!(res_minute.get_minute()?, *qinf::MINUTE);

  // minute ninf
  res_minute=socket.send_sync_message(&add_null!(K::new_minute(*qninf::MINUTE))).await?;
  assert_eq!(res_minute.get_minute()?, *qninf::MINUTE);

  // second
  let mut res_second=socket.send_sync_message(&add_null!(K:: new_second(Duration::seconds(11846)))).await?;
  assert_eq!(res_second.get_second()?, Duration::seconds(11846));

  // second null
  res_second=socket.send_sync_message(&add_null!(K:: new_second(*qnull::SECOND))).await?;
  assert_eq!(res_second.get_second()?, *qnull::SECOND);

  // second inf
  res_second=socket.send_sync_message(&add_null!(K:: new_second(*qinf::SECOND))).await?;
  assert_eq!(res_second.get_second()?, *qinf::SECOND);

  // second null
  res_second=socket.send_sync_message(&add_null!(K:: new_second(*qninf::SECOND))).await?;
  assert_eq!(res_second.get_second()?, *qninf::SECOND);

  // time 
  let mut res_time=socket.send_sync_message(&add_null!(K::new_time(Duration::milliseconds(78967302)))).await?;
  assert_eq!(res_time.get_time()?, Duration::milliseconds(78967302));

  // time null
  res_time=socket.send_sync_message(&add_null!(K::new_time(*qnull::TIME))).await?;
  assert_eq!(res_time.get_time()?, *qnull::TIME);

  // time inf
  res_time=socket.send_sync_message(&add_null!(K::new_time(*qinf::TIME))).await?;
  assert_eq!(res_time.get_time()?, *qinf::TIME);

  // time inf
  res_time=socket.send_sync_message(&add_null!(K::new_time(*qninf::TIME))).await?;
  assert_eq!(res_time.get_time()?, *qninf::TIME);

  // bool list
  res_bool=socket.send_sync_message(&add_null!(K::new_bool_list(vec![true, false], qattribute::UNIQUE))).await?;
  assert_eq!(res_bool.get_attribute(), qattribute::UNIQUE);
  assert_eq!(*res_bool.as_vec::<G>()?, vec![1_u8, 0]);

  // GUID list
  let guid_query=add_null!(K::new_guid_list(vec![[0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]], qattribute::NONE));
  let res_guid=socket.send_sync_message(&guid_query).await?;
  assert_eq!(*res_guid.as_vec::<U>()?, vec![[0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]]);
  
  // byte list
  res_byte=socket.send_sync_message(&add_null!(K::new_byte_list(vec![0x3c_u8, 0x22, 0x4f], qattribute::NONE))).await?;
  assert_eq!(*res_byte.as_vec::<G>()?, vec![0x3c, 0x22, 0x4f]);

  // short list
  res_short=socket.send_sync_message(&add_null!(K::new_short_list(vec![70_i16, 128, 1028, 2000], qattribute::SORTED))).await?;
  assert_eq!(res_short.get_attribute(), qattribute::SORTED);
  assert_eq!(*res_short.as_vec::<H>()?, vec![70_i16, 128, 1028, 2000]);

  // int list
  res_int=socket.send_sync_message(&add_null!(K::new_int_list(vec![234789, -34567789], qattribute::NONE))).await?;
  assert_eq!(*res_int.as_vec::<I>()?, vec![234789_i32, -34567789]);

  // long list
  res_long=socket.send_sync_message(&add_null!(K::new_long_list(vec![86400000000000_i64, -86400000000000], qattribute::NONE))).await?;
  assert_eq!(*res_long.as_vec::<J>()?, vec![86400000000000_i64, -86400000000000_i64]);

  // real list
  res_real=socket.send_sync_message(&add_null!(K::new_real_list(vec![-1.25_f32, 100.23, 3000.5639], qattribute::SORTED))).await?;
  assert_eq!(res_real.get_attribute(), qattribute::SORTED);
  assert_eq!(*res_real.as_vec::<E>()?, vec![-1.25, 100.23, 3000.5639]);

  // float list
  res_float=socket.send_sync_message(&add_null!(K::new_float_list(vec![103.678_f64, 0.00034], qattribute::NONE))).await?;
  assert_eq_float_vec!(*res_float.as_vec::<F>().expect("Failed to convert into f64 vec"), vec![103.678, 0.00034], 0.00001);

  // string 
  res_char=socket.send_sync_message(&add_null!(K::new_string(String::from("aabbccc"), qattribute::PARTED))).await?;
  assert_eq!(res_char.get_attribute(), qattribute::PARTED);
  assert_eq!(*res_char.as_string()?, String::from("aabbccc"));

  // symbol list
  res_symbol=socket.send_sync_message(&add_null!(K::new_symbol_list(vec![String::from("kdb+"), String::from("q")], qattribute::UNIQUE))).await?;
  assert_eq!(*res_symbol.as_vec::<S>()?, vec![String::from("kdb+"), String::from("q")]);

  // timestamp list
  let timestamp_query=add_null!(K::new_timestamp_list(vec![Utc.ymd(2018, 2, 18).and_hms_nano(4, 0, 0, 100), Utc.ymd(2019, 12, 3).and_hms_nano(4, 5, 10, 3456)], qattribute::NONE));
  res_timestamp=socket.send_sync_message(&timestamp_query).await?;
  assert_eq!(*res_timestamp.as_vec::<J>()?, vec![572241600000000100_i64, 628661110000003456]);
  
  // month list
  res_month=socket.send_sync_message(&add_null!(K::new_month_list(vec![Utc.ymd(2013, 9, 1), Utc.ymd(2009, 2, 1)], qattribute::NONE))).await?;
  assert_eq!(*res_month.as_vec::<I>()?, vec![164_i32, 109]);

  // date list 
  let res_date=socket.send_sync_message(&add_null!(K::new_date_list(vec![Utc.ymd(2000, 2, 9)], qattribute::NONE))).await?;
  assert_eq!(*res_date.as_vec::<I>()?, vec![39]);

  // datetime list
  res_datetime=socket.send_sync_message(&add_null!(K::new_datetime_list(vec![Utc.ymd(2004, 6, 17).and_hms_milli(11, 32, 40, 803)], qattribute::NONE))).await?;
  assert_eq_float_vec!(*res_datetime.as_vec::<F>()?, vec![1629.481], 0.001);

  // timespan list
  let timespan_query=add_null!(K::new_timespan_list(vec![Duration::nanoseconds(999), Duration::nanoseconds(10000), Duration::nanoseconds(100000000)], qattribute::NONE));
  res_timespan=socket.send_sync_message(&timespan_query).await?;
  assert_eq!(*res_timespan.as_vec::<J>()?, vec![999_i64, 10000, 100000000]);

  // minute list
  res_minute=socket.send_sync_message(&add_null!(K::new_minute_list(vec![Duration::minutes(741), Duration::minutes(182)], qattribute::NONE))).await?;
  assert_eq!(*res_minute.as_vec::<I>()?, vec![741, 182]);

  // second list
  res_second=socket.send_sync_message(&add_null!(K::new_second_list(vec![Duration::seconds(11846), Duration::seconds(14449)], qattribute::NONE))).await?;
  assert_eq!(*res_second.as_vec::<I>()?, vec![11846, 14449]);

  // time list
  let time_query=add_null!(K::new_time_list(vec![Duration::milliseconds(78967302), Duration::milliseconds(255000)], qattribute::NONE));
  res_time=socket.send_sync_message(&time_query).await?;
  assert_eq!(*res_time.as_vec::<I>()?, vec![78967302, 255000]);

  // compound list
  let compound_query=K::new_compound_list(vec![
    K::new_string(String::from("set"), qattribute::NONE),
    K::new_symbol(String::from("a")),
    K::new_compound_list(vec![
      K::new_long(42),
      K::new_real_list(vec![3.927524_f32, 5.170911], qattribute::SORTED),
      K::new_timestamp(Utc.ymd(2020, 2, 10).and_hms_nano(3, 19, 3, 247856731)),
      K::new_symbol_list(vec![String::from("KxSystems"), String::from("kdb+")], qattribute::NONE),
      K::new_datetime_list(vec![Utc.ymd(2020, 10, 1).and_hms_milli(3, 30, 12, 45), Utc.ymd(2008, 2, 18).and_hms_milli(21, 39, 10, 567)], qattribute::NONE),
      K::new_char('k')
    ])
  ]);
  socket.send_async_message(&compound_query).await?;

  let mut res_compound=socket.send_sync_message(&"a").await?;
  assert_eq!(res_compound.remove(0)?.get_long()?, 42);
  let real_list=res_compound.remove(0)?;
  assert_eq!(real_list.get_attribute(), qattribute::SORTED);
  assert_eq!(*real_list.as_vec::<E>()?, vec![3.927524_f32, 5.170911]);
  assert_eq!(res_compound.remove(0)?.get_timestamp()?, Utc.ymd(2020, 2, 10).and_hms_nano(3, 19, 3, 247856731));
  assert_eq!(*res_compound.remove(0)?.as_vec::<S>()?, vec![String::from("KxSystems"), String::from("kdb+")]);
  assert_eq_float_vec!(res_compound.remove(0)?.as_vec::<F>()?, vec![7579.146, 2970.902], 0.001);
  assert_eq!(res_compound.remove(0)?.get_char()?, 'k');

  // dictionary
  let mut q_dictionary=K::new_dictionary(
    K::new_symbol_list(vec![String::from("a"), String::from("b"), String::from("c")], qattribute::NONE),
    K::new_int_list(vec![10, 20, 30], qattribute::NONE)
  )?;

  let mut res_dictionary=socket.send_sync_message(&add_null!(q_dictionary)).await?;
  assert_eq!(format!("{}", res_dictionary), String::from("`a`b`c!10 20 30i"));

  // sorted dictionary
  q_dictionary=K::new_dictionary(
    K::new_long_list(vec![1_i64, 2, 3], qattribute::SORTED),
    K::new_compound_list(vec![K::new_string(String::from("string"), qattribute::NONE), K::new_bool_list(vec![true, false], qattribute::NONE), K::new_date(Utc.ymd(2021, 3, 9))])
  )?;
  res_dictionary=socket.send_sync_message(&add_null!(q_dictionary)).await?;
  assert_eq!(format!("{}", res_dictionary), String::from("`s#1 2 3!(\"string\";10b;2021.03.09)"));

  // table
  q_dictionary=K::new_dictionary(
    K::new_symbol_list(vec![String::from("a"), String::from("b"), String::from("c")], qattribute::NONE),
    K::new_compound_list(vec![
      K::new_int_list(vec![10, 20, 30], qattribute::NONE),
      K::new_symbol_list(vec![String::from("honey"), String::from("sugar"), String::from("maple")], qattribute::NONE),
      K::new_bool_list(vec![false, false, true], qattribute::NONE)
    ])
  )?;
  let q_table=q_dictionary.flip().unwrap();

  let res_table=socket.send_sync_message(&add_null!(q_table)).await?;
  assert_eq!(format!("{}", res_table), String::from("+`a`b`c!(10 20 30i;`honey`sugar`maple;001b)"));

  // Close socket
  socket.shutdown().await?;
  
  Ok(())
}

#[async_std::test]
async fn compression_test() -> Result<(), Box<dyn std::error::Error>>{

  // Connect to q process
  let mut socket=QStream::connect(ConnectionMethod::TCP, "localhost", 5000_u16, "kdbuser:pass").await.expect("Failed to connect");
  
  // uncompressed message //---------------------------/

  // Set test table remotely
  socket.send_async_message(&"tab:([]time:2000.01.01D00:00:00+86400000000000*til 1000; sym:raze 250#/: `AAPL`MSFT`AMZ`GOOGL)").await?;

  // Prepare q table which will NOT be compressed
  let mut time_vec=vec![Utc.timestamp_nanos(KDB_TIMESTAMP_OFFSET); 1000];
  for i in 0..1000{
    time_vec[i]=time_vec[i]+Duration::nanoseconds(ONE_DAY_NANOS * i as i64);
  }
 
  let columns=K::new_compound_list(vec![
    K::new_timestamp_list(time_vec, qattribute::NONE),
    K::new_symbol_list([vec![String::from("AAPL"); 250], vec![String::from("MSFT"); 250], vec![String::from("AMZ"); 250], vec![String::from("GOOGL"); 250]].concat(), qattribute::PARTED)
  ]);
  let header=K::new_symbol_list(vec![String::from("time"), String::from("sym")], qattribute::NONE);
  let mut original=K::new_dictionary(header, columns)?.flip().unwrap();

  // Assign sent table as `tab2`
  let mut table_query=K::new_compound_list(vec![K::new_string(String::from("set"), qattribute::NONE), K::new_symbol(String::from("tab2")), original]);
  socket.send_async_message(&table_query).await?;

  // Compare with `tab` sent before `tab2`
  let mut res_compare=socket.send_sync_message(&"tab ~ tab2").await?;
  assert_eq!(res_compare.get_bool()?, true);

  // compressed message //-----------------------------/

  // Set test table remotely
  socket.send_async_message(&"tab:([]time:1000#2000.01.01D00:00:00; sym:raze 250#/: `AAPL`MSFT`AMZ`GOOGL)").await?;
  
  // Prepare a table which should be compressed
  // Use uniform time column.
  let new_time_vec=K::new_timestamp_list(vec![Utc.timestamp_nanos(KDB_TIMESTAMP_OFFSET); 1000], qattribute::NONE);
  original=table_query.pop()?;
  let time_column=original.get_mut_column("time")?;
  let _=std::mem::replace(time_column, new_time_vec);
  table_query.push(&original).unwrap();

  socket.send_async_message(&table_query).await?;

  res_compare=socket.send_sync_message(&"tab ~ tab2").await?;
  assert_eq!(res_compare.get_bool()?, true);

  Ok(())
}
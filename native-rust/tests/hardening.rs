use b27_primitives::{compress,decompress,stream};
#[test]fn all_profiles(){for b in [vec![],vec![255;8192],"tiếng Việt, oà òa, a\u{301}, tiếng việt".repeat(100).into_bytes()]{for m in 0..10{assert_eq!(decompress(&compress(&b,m).unwrap()).unwrap(),b)}}}
#[test]fn strict_and_budget(){let mut b=vec![];stream::encode(&b"hello"[..],&mut b,6).unwrap();assert!(stream::decode_strict(&b[..],Vec::new(),4).is_err());let old=b"B27S\0\0\0\0";assert!(stream::decode_strict(&old[..],Vec::new(),100).is_err());assert!(stream::decode(&old[..],Vec::new(),100).is_ok());}

//! Base27 packing primitives. Not yet a full native B27N decoder.
#[no_mangle]
pub extern "C" fn b27_pack5(a:u32,b:u32,c:u32,d:u32,e:u32)->u32 {
 let s=[a,b,c,d,e]; if s.iter().any(|&x|x>=27){return u32::MAX}
 s.iter().fold(0,|v,x|v*27+x)
}
#[no_mangle]
pub extern "C" fn b27_digit(v:u32,i:u32)->u32 {
 if v>=27u32.pow(5)||i>=5{return u32::MAX}
 (v/27u32.pow(4-i))%27
}
#[cfg(test)]
mod tests{use super::*;#[test]fn boundaries(){assert_eq!(b27_pack5(26,26,26,26,26),14348906);assert_eq!(b27_pack5(27,0,0,0,0),u32::MAX);for v in [0,1,12345,14348906]{assert_eq!(b27_pack5(b27_digit(v,0),b27_digit(v,1),b27_digit(v,2),b27_digit(v,3),b27_digit(v,4)),v)}}}

//! B27N native codec, raw profiles 0/1/4/7. Linguistic profiles require JS bridge.
pub mod lz;pub mod huffman;pub mod range;
use sha2::{Sha256,Digest};
pub const LIMIT:usize=16*1024*1024;
#[no_mangle]pub extern "C" fn b27_pack5(a:u32,b:u32,c:u32,d:u32,e:u32)->u32{let s=[a,b,c,d,e];if s.iter().any(|&x|x>=27){return u32::MAX}s.iter().fold(0,|v,x|v*27+x)}
#[no_mangle]pub extern "C" fn b27_digit(v:u32,i:u32)->u32{if v>=27u32.pow(5)||i>=5{return u32::MAX}(v/27u32.pow(4-i))%27}
pub fn payload_encode(b:&[u8],tag:u8)->Result<Vec<u8>,&'static str>{if b.len()>LIMIT*5+1024{return Err("limit")}match tag{0=>Ok(b.to_vec()),1..=3=>Ok(huffman::encode(b)),4..=6=>Ok(lz::encode(b)),7..=9=>Ok(range::encode(&lz::encode(b))),_=>Err("mode")}}
pub fn payload_decode(b:&[u8],tag:u8,limit:usize)->Result<Vec<u8>,&'static str>{if limit>LIMIT*5+1024{return Err("limit")}match tag{0 if b.len()<=limit=>Ok(b.to_vec()),1..=3=>huffman::decode(b,limit),4..=6=>lz::decode(b,limit),7..=9=>lz::decode(&range::decode(b,limit.saturating_mul(2)+1024)?,limit),_=>Err("mode/limit")}}
pub fn compress(b:&[u8],mode:u8)->Result<Vec<u8>,&'static str>{if b.len()>LIMIT{return Err("limit")}let(mut tag,mut p)=(0,b.to_vec());if mode==255{for t in [1,4,7]{let q=payload_encode(b,t)?;if q.len()<p.len(){p=q;tag=t}}}else{if ![0,1,4,7].contains(&mode){return Err("raw profile only")}tag=mode;p=payload_encode(b,tag)?}let mut out=b"B27N".to_vec();out.push(tag);out.extend_from_slice(&(b.len() as u32).to_be_bytes());out.extend_from_slice(&Sha256::digest(b));out.extend(p);Ok(out)}
pub fn decompress(b:&[u8])->Result<Vec<u8>,&'static str>{if b.len()<41||&b[..4]!=b"B27N"{return Err("B27N required")}let tag=b[4];if ![0,1,4,7].contains(&tag){return Err("linguistic profile requires JS bridge")}let n=u32::from_be_bytes(b[5..9].try_into().unwrap()) as usize;if n>LIMIT||b.len()>LIMIT*3+2048{return Err("limit")}let p=payload_decode(&b[41..],tag,n)?;if p.len()!=n||Sha256::digest(&p)[..]!=b[9..41]{return Err("integrity")}Ok(p)}
// WASM ABI owns buffers in linear memory; callers copy data before next operation.
use std::cell::RefCell;
thread_local!{static INPUT:RefCell<Vec<u8>>=const{RefCell::new(Vec::new())};static OUTPUT:RefCell<Vec<u8>>=const{RefCell::new(Vec::new())};}
#[no_mangle]pub extern "C" fn b27_input(n:u32)->*mut u8{if n as usize>LIMIT*3+2048{return std::ptr::null_mut()}INPUT.with(|x|{let mut x=x.borrow_mut();x.resize(n as usize,0);x.as_mut_ptr()})}
#[no_mangle]pub extern "C" fn b27_run(op:u32,mode:u32,limit:u32)->i32{OUTPUT.with(|o|o.borrow_mut().clear());if mode>255{return -1}let r=INPUT.with(|x|{let x=x.borrow();match op{0=>compress(&x,mode as u8),1=>decompress(&x),2=>payload_encode(&x,mode as u8),3=>payload_decode(&x,mode as u8,limit as usize),_=>Err("op")}});match r{Ok(v)=>{OUTPUT.with(|o|*o.borrow_mut()=v);0},Err(_)=>-1}}
#[no_mangle]pub extern "C" fn b27_output_ptr()->*const u8{OUTPUT.with(|o|o.borrow().as_ptr())}
#[no_mangle]pub extern "C" fn b27_output_len()->u32{OUTPUT.with(|o|o.borrow().len() as u32)}
#[cfg(test)]mod tests{use super::*;#[test]fn roundtrip(){let mut seed=42u32;for n in 0..256{let x:Vec<_>=(0..n).map(|_|{seed=seed.wrapping_mul(1664525).wrapping_add(1013904223);(seed>>24)as u8}).collect();for m in [0,1,4,7,255]{let y=compress(&x,m).unwrap();assert_eq!(decompress(&y).unwrap(),x);let mut z=y;z[9]^=1;assert!(decompress(&z).is_err())}}}#[test]fn base(){assert_eq!(b27_pack5(26,26,26,26,26),14348906);assert_eq!(b27_digit(14348906,4),26)}}

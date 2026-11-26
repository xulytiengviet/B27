use std::collections::HashMap;
pub fn encode(b:&[u8])->Vec<u8>{
 let mut out=Vec::new();let mut map=HashMap::new();let mut i=0;
 let key=|i:usize| if i+2<b.len(){Some((b[i] as u32)<<16|(b[i+1] as u32)<<8|b[i+2] as u32)}else{None};
 while i<b.len(){let f=out.len();out.push(0);for j in 0..8{if i==b.len(){break}let mut n=0;let mut d=0;
 if let Some(p)=key(i).and_then(|k|map.get(&k).copied()){d=i-p;if d>0&&d<=65535{while n<258&&i+n<b.len()&&b[p+n]==b[i+n]{n+=1}}}
 if n>=3{out[f]|=1<<j;out.extend_from_slice(&[(d>>8) as u8,d as u8,(n-3) as u8]);}
 else{n=1;out.push(b[i]);}
 for k in i..i+n{if let Some(x)=key(k){map.insert(x,k);}}i+=n;
 // Keep only entries that could still match; periodic bounded cleanup.
 if i/65536!=(i-n)/65536{map.retain(|_,p|i-*p<=65535);}
 }}out
}
pub fn decode(b:&[u8],limit:usize)->Result<Vec<u8>,&'static str>{
 let mut out=Vec::new();let mut i=0;while i<b.len(){let f=b[i];i+=1;if i==b.len(){return Err("empty group")}
 for j in 0..8{if i==b.len(){break}if f&(1<<j)!=0{if b.len()-i<3{return Err("match")}let d=b[i] as usize*256+b[i+1] as usize;let n=b[i+2] as usize+3;i+=3;if d==0||d>out.len()||n>limit.saturating_sub(out.len()){return Err("distance/limit")}for _ in 0..n{let x=out[out.len()-d];out.push(x)}}else{if out.len()>=limit{return Err("limit")}out.push(b[i]);i+=1}}}Ok(out)
}

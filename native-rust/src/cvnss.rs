//! Port of the compression-relevant CQN/CVSS converter rules.
use crate::{rules::*,LIMIT};
use unicode_normalization::UnicodeNormalization;
use regex::Regex;
use std::sync::OnceLock;
fn lookup<'a>(table:&[(&str,&'a str)],s:&str)->Option<&'a str>{table.binary_search_by(|(k,_)|k.cmp(&s)).ok().map(|i|table[i].1)}
fn base(s:&str)->char{let c=s.chars().next().unwrap_or('\0');for g in VOWELS{if g.contains(c){return g.chars().next().unwrap()}}c}
fn front(s:&str)->bool{matches!(base(s),'i'|'ê'|'e')||s.is_empty()}
fn tail(s:&str)->&str{s.get(s.chars().next().map(char::len_utf8).unwrap_or(0)..).unwrap_or("")}
fn case(src:&str,s:String)->String{if src.chars().any(char::is_alphabetic)&&src==src.to_uppercase(){return s.to_uppercase()}if src.chars().next().is_some_and(|c|c.is_uppercase()){let mut it=s.chars();if let Some(c)=it.next(){return c.to_uppercase().chain(it).collect()}}s}
pub fn encode_word(word:&str)->String{let low:String=word.to_lowercase().nfc().collect();let (mut onset,mut v)=("",low.clone());for &(i,_) in INITIALS{if let Some(rest)=low.strip_prefix(i){onset=i;v=rest.to_owned();break}}
if onset=="gi"&&v!="a"&&lookup(FORWARD,&format!("i{v}")).is_some(){v=format!("i{v}")}
if onset=="qu"&&base(&v)=='y'{v=format!("u{v}")}
if onset=="g"&&base(&v)=='i'{onset="gi"}if onset=="g"&&(matches!(base(&v),'e'|'ê')||v.is_empty()){onset="gh"}
if onset=="c"&&front(&v){onset="k"}if onset=="ng"&&front(&v){onset="ngh"}
let i=INITIALS.iter().find(|(x,_)|*x==onset).map(|(_,y)|*y).unwrap_or(onset);case(word,format!("{}{}",i,lookup(FORWARD,&v).unwrap_or(&v)))}
fn split_code(low:&str)->(&str,&str){if lookup(REVERSE,low).is_some(){return("",low)}for &i in CODE_INITIALS{if let Some(rest)=low.strip_prefix(i){if !rest.is_empty()&&lookup(REVERSE,rest).is_some(){return(i,rest)}}}for &i in CODE_INITIALS{if let Some(rest)=low.strip_prefix(i){return(i,rest)}}("",low)}
fn build(i:&str,v:&str)->String{let onset=match i{"q"=>"qu","j"=>"gi","z"=>"d","d"=>"đ","f"=>"ph","k"=>"kh","w"=>if front(v){"ngh"}else{"ng"},"g"=>if front(v){"gh"}else{"g"},"c"=>if front(v){"k"}else{"c"},_=>i};let mut v=v.to_owned();
if onset.is_empty()&&!v.starts_with("ia"){if let Some(c)=v.chars().next(){if let Some(p)="iìỉĩíị".chars().position(|x|x==c){v=format!("{}{}","yỳỷỹýỵ".chars().nth(p).unwrap(),tail(&v))}}}
if onset=="gi"&&base(&v)=='i'{if v.chars().next().is_some_and(|c|"iìỉĩíị".contains(c))&&["","m","n","p","t","c","ch","ng","nh"].contains(&tail(&v)){return format!("g{v}")}if !tail(&v).is_empty(){v=tail(&v).to_owned()}}
if onset=="qu"&&base(&v)=='u'&&v.chars().count()>1{v=tail(&v).to_owned()}format!("{onset}{v}")}
pub fn decode_word(word:&str)->String{let low:String=word.to_lowercase().nfc().collect();let(i,v)=split_code(&low);case(word,build(i,lookup(REVERSE,v).unwrap_or(v)))}
pub fn candidates(word:&str)->Vec<String>{let low:String=word.to_lowercase().nfc().collect();let(i,v)=split_code(&low);let mut out=vec![];if let Some((_,values))=CANDIDATES.iter().find(|(k,_)|*k==v){for s in *values{let x=case(word,build(i,s));if !out.contains(&x){out.push(x)}}}else{out.push(decode_word(word))}out}
fn letters()->&'static Regex{static R:OnceLock<Regex>=OnceLock::new();R.get_or_init(||Regex::new(r"\p{L}+").unwrap())}
pub fn convert(s:&str,reverse:bool)->String{let norm:String=s.nfc().collect();letters().replace_all(&norm,|c:&regex::Captures|if reverse{decode_word(&c[0])}else{encode_word(&c[0])}).into_owned()}
const A:&[u8]=b"abcdefghijklmnopqrstuvwxyz ";
fn pack(s:&[u8])->Vec<u8>{let mut out=vec![];for c in s.chunks(5){let mut v=0u32;for b in c{v=v*27+A.iter().position(|x|x==b).unwrap() as u32}out.extend_from_slice(&v.to_be_bytes()[1..])}out}
fn unpack(b:&[u8],n:usize)->Result<Vec<u8>,&'static str>{if b.len()!=3*n.div_ceil(5){return Err("pack size")}let mut out=vec![0;n];for(i,c)in b.chunks_exact(3).enumerate(){let m=(n-i*5).min(5);let mut v=u32::from_be_bytes([0,c[0],c[1],c[2]]);if v>=27u32.pow(m as u32){return Err("digit")}for j in (0..m).rev(){out[i*5+j]=A[(v%27)as usize];v/=27}}Ok(out)}
fn record(out:&mut Vec<u8>,tag:u8,b:&[u8],n:usize){out.push(tag);out.extend_from_slice(&(n as u32).to_be_bytes());out.extend_from_slice(b)}
fn flush(out:&mut Vec<u8>,buf:&mut Vec<u8>,tag:u8){if buf.is_empty(){return}if tag==1{record(out,tag,&pack(buf),buf.len())}else{record(out,tag,buf,buf.len())}buf.clear()}
pub fn transform(b:&[u8],packed:bool)->Vec<u8>{let mut out=vec![];let Ok(s)=std::str::from_utf8(b)else{record(&mut out,0,b,b.len());return out};static R:OnceLock<Regex>=OnceLock::new();let r=R.get_or_init(||Regex::new(r"[\p{L}\p{M}]+| +|[^\p{L}\p{M} ]+").unwrap());let(mut code,mut raw)=(vec![],vec![]);let tag=if packed{1}else{2};for m in r.find_iter(s){let w=m.as_str();let x=if w.bytes().all(|b|b==b' '){Some(w.to_owned())}else if w==w.to_lowercase()&&w.encode_utf16().count()<=64{let q=convert(w,false);if !q.is_empty()&&q.bytes().all(|b|b.is_ascii_lowercase())&&convert(&q,true)==w{Some(q)}else{None}}else{None};if let Some(x)=x{flush(&mut out,&mut raw,0);code.extend_from_slice(x.as_bytes())}else{flush(&mut out,&mut code,tag);raw.extend_from_slice(w.as_bytes())}}flush(&mut out,&mut code,tag);flush(&mut out,&mut raw,0);out}
pub fn inverse(b:&[u8],max:usize)->Result<Vec<u8>,&'static str>{let mut out=vec![];let mut i=0;while i<b.len(){if b.len()-i<5{return Err("record header")}let tag=b[i];let n=u32::from_be_bytes(b[i+1..i+5].try_into().unwrap())as usize;i+=5;if n==0||n>LIMIT*4||tag>2{return Err("record")}let size=if tag==1{3*n.div_ceil(5)}else{n};if size>b.len()-i{return Err("truncated record")}let p=&b[i..i+size];i+=size;if tag==0{if p.len()>max.saturating_sub(out.len()){return Err("limit")}out.extend_from_slice(p)}else{let x=if tag==1{unpack(p,n)?}else{p.to_vec()};if !x.iter().all(|b|b.is_ascii_lowercase()||*b==b' '){return Err("alphabet")}let s=std::str::from_utf8(&x).map_err(|_|"utf8")?;for w in s.split_inclusive(' '){let t=w.trim_end_matches(' ');let d=if t.is_empty(){String::new()}else{decode_word(t)};let spaces=w.len()-t.len();if d.len()+spaces>max.saturating_sub(out.len()){return Err("limit")}out.extend_from_slice(d.as_bytes());out.extend(std::iter::repeat(b' ').take(spaces));}}}Ok(out)}

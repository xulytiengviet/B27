'use strict';
const assert=require('node:assert/strict'), b=require('./b27.cjs'), c=require('./cvnss.cjs'), z=require('node:zlib'), fs=require('node:fs'), crypto=require('node:crypto'), {performance}=require('node:perf_hooks');
assert(c.selfTest().ok);let count=0;
function check(x){for(const mode of ['auto','linguistic']){assert.deepEqual(b.decompress(b.compress(x,mode)),x);count++}}
for(let i=0;i<27;i++){const t=b.trits(i);assert.equal(9*(t[0]+1)+3*(t[1]+1)+t[2]+1,i)}
for(let n=0;n<100;n++){const s='abcdefghijklmnopqrstuvwxyz '.repeat(4).slice(0,n);assert.equal(b.unpack(b.pack(s),s.length),s);check(Buffer.from(s))}
for(const s of ['', 'Tiếng Việt, tiếng Việt!', 'oà òa uỷ ủy yên iên sẽ hề mồ mộ võ', 'a\u0301\r\n\t😀\0', 'hello world ed es od of os', 'tiếng việt  tiếng việt'])check(Buffer.from(s));
let seed=123456;for(let n=0;n<256;n++){const x=Buffer.alloc(n);for(let j=0;j<n;j++){seed=(Math.imul(seed,1664525)+1013904223)>>>0;x[j]=seed>>>24}check(x)}
const broken=b.compress(Buffer.from('abc'));broken[9]^=1;assert.throws(()=>b.decompress(broken));
const fixtures=process.argv.slice(2);const samples=fixtures.length?fixtures.map(p=>[p,fs.readFileSync(p)]):[['synthetic-vi',Buffer.from('tiếng việt là ngôn ngữ của người việt nam. nghiên cứu khoa học và công nghệ. '.repeat(1000))],['mixed',Buffer.from('Tiếng Việt ABC 123 😀 oà òa \r\n'.repeat(500))]];
const rows=[];for(const [name,x] of samples){for(const [method,enc,dec] of [['B27-auto',x=>b.compress(x),b.decompress],['B27-CVNSS',x=>b.compress(x,'linguistic'),b.decompress],['DEFLATE',x=>z.deflateRawSync(x,{level:6}),z.inflateRawSync],['Brotli',x=>z.brotliCompressSync(x,{params:{[z.constants.BROTLI_PARAM_QUALITY]:6}}),z.brotliDecompressSync]]){let a=[],d=[],y;for(let i=0;i<7;i++){let t=performance.now();y=enc(x);let u=performance.now();assert.deepEqual(dec(y),x);let v=performance.now();if(i){a.push(u-t);d.push(v-u)}}const med=a=>a.sort((x,y)=>x-y)[Math.floor(a.length/2)];rows.push({name,sha256:crypto.createHash('sha256').update(x).digest('hex'),method,input:x.length,output:y.length,encode_ms:med(a),decode_ms:med(d),exact:true})}}
console.log(JSON.stringify({node:process.version,platform:process.platform,arch:process.arch,converter:c.audit(),roundtrips:count,rows,synthetic:!fixtures.length},null,2));

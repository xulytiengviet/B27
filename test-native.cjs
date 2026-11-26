'use strict';
const a=require('node:assert/strict'),n=require('./native.cjs'),old=require('./b27.cjs'),{TritTrie}=require('./engine/trie.cjs'),h=require('./engine/huffman.cjs'),lz=require('./engine/lz27.cjs'),r=require('./engine/range27.cjs');
let count=0,seed=42;const trie=new TritTrie();for(let i=0;i<27;i++)trie.set(old.trits(i),i);for(let i=0;i<27;i++)a.equal(trie.get(old.trits(i)),i);
const cases=[Buffer.alloc(0),Buffer.alloc(400,255),Buffer.from(Array.from({length:256},(_,i)=>i)),Buffer.from('tiếng việt  oà òa uỷ ủy 😀\r\na\u0301\0'.repeat(50))];for(let len=0;len<128;len++){const b=Buffer.alloc(len);for(let i=0;i<len;i++){seed=(Math.imul(seed,1664525)+1013904223)>>>0;b[i]=seed>>>24}cases.push(b)}
for(const b of cases){for(const codec of [h,lz,r]){a.deepEqual(codec.decode(codec.encode(b),b.length),b);count++}for(const mode of ['auto','store','huffman','lz','range']){const p=n.compress(b,mode);a.deepEqual(n.decompress(p),b);const q=Buffer.from(p);q[9]^=1;a.throws(()=>n.decompress(q));count++}a.deepEqual(n.decompress(old.compress(b)),b);count++}
console.log(JSON.stringify({roundtrips:count,fixtures:cases.length,legacy:true}));

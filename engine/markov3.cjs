'use strict';
// Context = previous trit (3 states) + start state. Laplace prior, deterministic rescaling.
class Markov3{constructor(){this.counts=Array.from({length:4},()=>[1,1,1]);this.context=3}cdf(){const [a,b,c]=this.counts[this.context];return [0,a,a+b,a+b+c]}update(t){if(!Number.isInteger(t)||t<0||t>2)throw Error('trit');const a=this.counts[this.context];a[t]++;if(a[0]+a[1]+a[2]>=4096)for(let i=0;i<3;i++)a[i]=(a[i]+1)>>1;this.context=t}}
module.exports={Markov3};
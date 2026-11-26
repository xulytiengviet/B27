'use strict';
class TritTrie {
 constructor(){this.root={children:[null,null,null],value:undefined}}
 set(path,value){let n=this.root;for(const t of path){if(![-1,0,1].includes(t))throw Error('trit');n=n.children[t+1]??(n.children[t+1]={children:[null,null,null],value:undefined})}n.value=value}
 get(path){let n=this.root;for(const t of path){if(![-1,0,1].includes(t))throw Error('trit');n=n.children[t+1];if(!n)return undefined}return n.value}
}
module.exports={TritTrie};
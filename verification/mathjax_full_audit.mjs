import fs from "node:fs";
import {mathjax} from "mathjax-full/js/mathjax.js";
import {TeX} from "mathjax-full/js/input/tex.js";
import {liteAdaptor} from "mathjax-full/js/adaptors/liteAdaptor.js";
import {RegisterHTMLHandler} from "mathjax-full/js/handlers/html.js";
import {AllPackages} from "mathjax-full/js/input/tex/AllPackages.js";
const input=process.argv[2]??"ray to outram park/data/full_markdown_math_audit.json";
const output=process.argv[3]??"ray to outram park/data/full_markdown_math_mathjax.json";
const data=JSON.parse(fs.readFileSync(input,"utf8"));
const adaptor=liteAdaptor();RegisterHTMLHandler(adaptor);
const tex=new TeX({packages:AllPackages,formatError:(jax,err)=>{throw err;}});
const html=mathjax.document("",{InputJax:tex});
let pass=0,fail=0;const failures=[];
for(const b of data.blocks){try{html.convert(b.body,{display:true});pass++;}catch(e){fail++;failures.push({audit_index:b.audit_index,equation_id:b.equation_id,start_line:b.start_line,end_line:b.end_line,section:b.section,error:String(e?.message??e)});}}
const out={parser:"MathJax",version:"3.2.2",total:data.blocks.length,pass,fail,warning:0,failures};
fs.writeFileSync(output,JSON.stringify(out,null,2));console.log(JSON.stringify(out));process.exit(fail?1:0);

import fs from "node:fs";
import {mathjax} from "mathjax-full/js/mathjax.js";
import {TeX} from "mathjax-full/js/input/tex.js";
import "mathjax-full/js/input/tex/ams/AmsConfiguration.js";
import "mathjax-full/js/input/tex/newcommand/NewcommandConfiguration.js";
import "mathjax-full/js/input/tex/boldsymbol/BoldsymbolConfiguration.js";
import "mathjax-full/js/input/tex/textmacros/TextMacrosConfiguration.js";
import {SVG} from "mathjax-full/js/output/svg.js";
import {liteAdaptor} from "mathjax-full/js/adaptors/liteAdaptor.js";
import {RegisterHTMLHandler} from "mathjax-full/js/handlers/html.js";

const input=process.argv[2]??"ray to outram park/data/full_markdown_math_audit.json";
const output=process.argv[3]??"ray to outram park/data/full_markdown_math_mathjax.json";
const data=JSON.parse(fs.readFileSync(input,"utf8"));
const packages=["base","ams","newcommand","boldsymbol","textmacros"]; // registered explicitly above; bussproofs intentionally absent
let adaptor,tex,svg,html;
function initialize(){
  adaptor=liteAdaptor();
  RegisterHTMLHandler(adaptor);
  tex=new TeX({packages,formatError:(jax,err)=>{throw err;}});
  svg=new SVG({fontCache:"none"});
  html=mathjax.document("",{InputJax:tex,OutputJax:svg});
}
function render(expr){html.convert(expr,{display:true});}
const selfTests=[
  String.raw`N_V(t)=\int_V c(\mathbf{x},t)\,dV`,
  String.raw`[\mathbf J]=\mathrm{mol\,m^{-2}\,s^{-1}}`,
  String.raw`S_{i,\mathrm{gen}}=\begin{cases}S_0,&0\le r<r_1,\\0,&r_1<r<R\end{cases}`,
  String.raw`\frac{d}{dr}(r^2w')`,
  String.raw`\boldsymbol\Phi=(\mathbf I-\mathbf K)^{-1}\mathbf B`,
  String.raw`\mathbf M=\begin{pmatrix}a&b\\c&d\end{pmatrix}`,
  String.raw`\frac{d}{dr}\left[r^2D_i\left(\phi_i^{(m)}\phi_i^{(n)\prime}-\phi_i^{(n)}\phi_i^{(m)\prime}\right)\right]`
];
try{
 initialize();
 for(const x of selfTests)render(x);
}catch(e){
 const out={status:"HARNESS_CONFIGURATION_ERROR",parser:"MathJax",version:"3.2.2",packages,self_test_pass:false,total:0,pass:0,fail:0,warning:0,harness_errors:1,error:String(e?.message??e),failures:[]};
 fs.writeFileSync(output,JSON.stringify(out,null,2));console.error(JSON.stringify(out));process.exit(2);
}
let pass=0,fail=0;const failures=[];
for(const b of data.blocks){
 try{render(b.body);pass++;}
 catch(e){fail++;failures.push({classification:"EXPRESSION_PARSE_ERROR",audit_index:b.audit_index,equation_id:b.equation_id,start_line:b.start_line,end_line:b.end_line,section:b.section,error:String(e?.message??e)});}
}
const out={status:fail?"EXPRESSION_FAILURES":"PASS",parser:"MathJax",version:"3.2.2",output_jax:"SVG",packages,bussproofs_loaded:false,self_test_pass:true,self_tests:selfTests.length,total:data.blocks.length,pass,fail,warning:0,harness_errors:0,failures};
fs.writeFileSync(output,JSON.stringify(out,null,2));console.log(JSON.stringify({...out,failures:failures.slice(0,20)}));process.exit(fail?1:0);

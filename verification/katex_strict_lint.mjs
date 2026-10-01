import fs from "node:fs";
import katex from "katex";
const input=process.argv[2]??"ray to outram park/data/markdown_math_expressions.json";
const output=process.argv[3]??"ray to outram park/data/markdown_math_errors.json";
const data=JSON.parse(fs.readFileSync(input,"utf8"));let pass=0,fail=0;const results=[];
for(const e of data.expressions){try{katex.renderToString(e.raw,{displayMode:e.type==="display",throwOnError:true,strict:"error",trust:false,output:"htmlAndMathml"});pass++;results.push({...e,status:"PASS"});}catch(err){fail++;results.push({...e,status:"FAIL",error_class:err?.name??"KaTeXError",parser_error:String(err?.message??err)});}}
const out={parser:"KaTeX",version:"0.16.22",strict:"error",throwOnError:true,source:data.source,line_count:data.line_count,total_expressions:data.expressions.length,stable_tag_count:data.stable_tag_count,stable_unique_tag_count:data.stable_unique_tag_count,parser_pass:pass,parser_fail:fail,warning_count:0,structural_findings:data.structural_findings,command_inventory:data.command_inventory,results};fs.writeFileSync(output,JSON.stringify(out,null,2));console.log(JSON.stringify({total:out.total_expressions,pass,fail,structural:out.structural_findings.length,stable:out.stable_unique_tag_count}));process.exit(fail===0&&out.structural_findings.length===0?0:1);

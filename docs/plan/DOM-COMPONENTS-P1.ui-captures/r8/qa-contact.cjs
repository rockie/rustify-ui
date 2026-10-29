// Derived thumbnails for primary-agent authenticity QA only. Judges use originals.
const fs=require('node:fs'),path=require('node:path'),{PNG}=require('pngjs');
const p=__dirname,records=fs.existsSync(path.join(p,'capture-log.json'))?JSON.parse(fs.readFileSync(path.join(p,'capture-log.json'))).runs.flatMap(r=>r.records):JSON.parse(fs.readFileSync(path.join(p,'capture-progress.json'))).records,q=path.join(p,'qa');fs.mkdirSync(q,{recursive:true});
const maps=[];
for(let n=0;n<records.length;n+=12){
 const sheet=new PNG({width:1440,height:900});sheet.data.fill(230);const rows=records.slice(n,n+12);
 for(let k=0;k<rows.length;k++){
  const src=PNG.sync.read(fs.readFileSync(path.join(p,rows[k].file))),scale=Math.min(350/src.width,288/src.height),w=Math.floor(src.width*scale),h=Math.floor(src.height*scale),ox=(k%4)*360+Math.floor((360-w)/2),oy=Math.floor(k/4)*300+Math.floor((300-h)/2);
  for(let y=0;y<h;y++)for(let x=0;x<w;x++){const si=(Math.min(src.height-1,Math.floor(y/scale))*src.width+Math.min(src.width-1,Math.floor(x/scale)))*4,di=((oy+y)*1440+ox+x)*4;sheet.data[di]=src.data[si];sheet.data[di+1]=src.data[si+1];sheet.data[di+2]=src.data[si+2];sheet.data[di+3]=255;}
 }
 const file=`contact-${String(n/12+1).padStart(2,'0')}.png`;fs.writeFileSync(path.join(q,file),PNG.sync.write(sheet));maps.push({file,order:'left-to-right, top-to-bottom',originals:rows.map(r=>r.file)});
}
fs.writeFileSync(path.join(q,'manifest.json'),JSON.stringify({purpose:'Primary-agent authenticity QA only. Derived contacts are not review evidence and not supplied to judges.',sheets:maps},null,2));console.log(JSON.stringify({originals:records.length,sheets:maps.length}));

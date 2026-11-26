"""Linux complete-process benchmark, actual user/system CPU and max RSS via GNU time."""
import hashlib,json,pathlib,subprocess,sys,tempfile,statistics
exe=str(pathlib.Path('native-rust/target/release/b27').resolve())
rows=[]
for name in sys.argv[1:]:
    src=pathlib.Path(name).resolve()
    with tempfile.TemporaryDirectory() as d:
        p=pathlib.Path(d)
        for method in ('b27','xz'):
            enc=p/'compressed';out=p/'restored';samples=[]
            for _ in range(5):
                enc.unlink(missing_ok=True);out.unlink(missing_ok=True)
                commands=([exe,'cs',str(src),str(enc),'6'],[exe,'ds',str(enc),str(out)]) if method=='b27' else (['xz','-6','-c',str(src)],['xz','-dc',str(enc)])
                metrics=[]
                for cmd,dest in zip(commands,[enc,out]):
                    log=p/'time'
                    with (dest.open('wb') if method=='xz' else open('/dev/null','wb')) as stdout:
                        subprocess.run(['/usr/bin/time','-f','%U %S %M','-o',str(log),*cmd],stdout=stdout,check=True)
                    u,s,m=map(float,log.read_text().split());metrics.append((u+s,m))
                assert src.read_bytes()==out.read_bytes()
                samples.append(metrics)
            rows.append(dict(file=str(src),sha256=hashlib.sha256(src.read_bytes()).hexdigest(),method=method,input=src.stat().st_size,output=enc.stat().st_size,encode_cpu_s=statistics.median(x[0][0] for x in samples),decode_cpu_s=statistics.median(x[1][0] for x in samples),encode_rss_kib=max(x[0][1] for x in samples),decode_rss_kib=max(x[1][1] for x in samples)))
print(json.dumps(rows,indent=2))

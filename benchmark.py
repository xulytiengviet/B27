"""Reproducible complete-file comparison; no third-party dependencies."""
import hashlib, json, lzma, pathlib, statistics, subprocess, sys, tempfile, time
def median(fn):
    fn()
    results=[]
    for _ in range(5):
        start=time.perf_counter_ns(); value=fn(); results.append((time.perf_counter_ns()-start)/1e6)
    return value,statistics.median(results)
rows=[]
for path in sys.argv[1:]:
    x=pathlib.Path(path).read_bytes()
    if len(x)>16*1024*1024: raise ValueError("16 MiB limit")
    with tempfile.TemporaryDirectory() as tmp:
        p=pathlib.Path(tmp); src=p/'in'; enc=p/'out.b27'; dst=p/'restored'; src.write_bytes(x)
        def b27():
            enc.unlink(missing_ok=True)
            subprocess.run(['node','b27.cjs','c',str(src),str(enc)],check=True)
            return enc.read_bytes()
        def undo():
            dst.unlink(missing_ok=True)
            subprocess.run(['node','b27.cjs','d',str(enc),str(dst)],check=True)
            return dst.read_bytes()
        for name,fn,back in [('B27-cli',b27,lambda y:undo()),('XZ-LZMA2-6',lambda:lzma.compress(x,preset=6),lzma.decompress)]:
            y,t=median(fn); result,d=median(lambda:back(y))
            assert result==x
            rows.append(dict(file=path,sha256=hashlib.sha256(x).hexdigest(),method=name,input=len(x),output=len(y),encode_ms=t,decode_ms=d,exact=True))
print(json.dumps({'timing_note':'B27 CLI includes process startup and disk I/O; XZ is in-process, so timings are not a fair codec CPU comparison. Size is complete-file comparison.','rows':rows},indent=2))

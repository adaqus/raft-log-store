# Benchmarking

## Using wrk

```bash
wrk -c 10 -d 60s -t 4 -R 20000 -s write.lua http://127.0.0.1:9050/write
```

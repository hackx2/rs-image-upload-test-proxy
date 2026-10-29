# a rust sha256 image upload proxy test

Client speaks to the proxy server, which forwards a request (including the upload bytes) to the backend rust server.

Why sha256? idk 😿😿

## run
1.
```sh
> cd frontend
> haxe build.hxml
```
2.
```sh
> cd backend
> cargo run
```
3.
```sh
> curl -X POST --data-binary "@the/super/cool/path/to/the/image.png" http://127.0.0.1:3001/upload
```

## prerequisites
~ haxe (nightly) <br>
&emsp;~ [hx2go](https://github.com/go2hx/hx2go) % `haxelib git hx2go https://github.com/go2hx/hx2go 0d9dc2a5fa9ad3d1525466a9ce3ada8b7db25aef` <br>
~ rust (editon:2024) <br>
&emsp;~ deps % `cargo` <br>
~ ffmpeg

---
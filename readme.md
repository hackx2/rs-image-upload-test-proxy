# a rust sha256 image upload proxy test

Client speaks to the proxy server, which forwards a request (including the upload bytes) to the backend rust server.

Why sha256? idk 😿😿

## run
1.
```sh
> cd proxy
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

## deps
```sh
> haxe
> rust
```

---
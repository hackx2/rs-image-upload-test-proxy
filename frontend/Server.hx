package;

import haxe.io.BytesOutput;
import haxe.io.Bytes;
import sys.Http;
import sys.io.File;
import sys.net.Host;
import flow.http.Server;
import flow.http.Request;
import flow.http.Response;
import flow.http.StatusCode;
import flow.http.Cookies.Cookie;
import flow.http.header.MimeType;
import flow.http.header.ContentTypeOptions;
import flow.http.header.Headers.HeaderName;

using dotenv.DotEnv;
using StringTools;

inline function notFound() {
	return Response.txt(NotFound, 'not found');
}

function main() {
	dotenv.DotEnv.load('../');

	final ip = 'FRONTEND_IP'.env();
	final port = 'FRONTEND_PORT'.env();
	final backend_port = 'BACKEND_PORT'.env();

	final backend_url = 'http://$ip:$backend_port';

	final server = new Server(new Host(ip), Std.parseInt(port));
	server.max_body_bytes = 100 * 1024 * 1024;
	server.handle = req -> {
		switch [req.method, req.path] {
			case [Get, '/'] | [Get, '/upload']:
				return Response.html(OK, File.getContent('public/upload.html')).header(CacheControl, 'no-store');

			case [Post, '/upload']:
				final http = new Http('$backend_url/upload');
				http.cnxTimeout = 67;
				http.setPostBytes(req.body);
				var body:String = null;
				http.onData = d -> body = d;
				var error:String = null;
				http.onError = e -> error = e;
				var status:StatusCode = 0;
				http.onStatus = s -> status = s;

				http.request(true);

				trace(body);

				if (error != null)
					return Response.txt(BadGateway, 'backend err: $error');
				return Response.json(OK, body);

			case [Get, path] if (path.startsWith('/data/')):
				final http = new Http(backend_url + path);

				final out = new BytesOutput();
				http.customRequest(false, out);

				final bytes = out.getBytes();

				// display 404 page
				if (bytes.length == 0) {
					return notFound();
				}
				// trace(path.indexOf('.'));
				// trace(path.split('.')[path.split('.').length-1]);

				final ext = path.split('.')[path.split('.').length - 1];
				// trace(MimeType.ext(ext));
				// trace(out.getBytes().length);
				return Response.bytes(OK, out.getBytes(), MimeType.ext(ext));

			case _: return notFound();
		};
	};

	server.listen(512, () -> {
		trace('Server is listening on ${server.host.toString()}:${server.port} hehe');
	});
}

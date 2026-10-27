package;

import haxe.Json;

import sys.Http;
import sys.io.File;
import sys.net.Host;

import flow.http.Server;
import flow.http.Request;
import flow.http.Response;
import flow.http.StatusCode;
import flow.http.Cookies.Cookie;
import flow.http.header.ContentTypeOptions;
import flow.http.header.Headers.HeaderName;

function main() {
	final server = new Server(new Host("127.0.0.1"), 3000);
	server.max_body_bytes = 100 * 1024 * 1024;

	server.handle = req -> {
		switch [req.method, req.path] {
			case [Get, '/'] | [Get, '/upload']:
				return Response.html(OK, File.getContent('public/upload.html')).header(CacheControl, 'no-store');

			case [Post, '/upload']:
				final http = new Http("http://127.0.0.1:3001/upload");
				http.setPostBytes(req.body);
				var body:String = null;
				http.onData = d -> body = d;
				var error:String = null;
				http.onError = e -> error = e;
				var status:StatusCode = 0;
				http.onStatus = s -> status = s;

				http.request(true);

				if (error != null) return Response.txt(BadGateway, 'backend err: $error');
				return Response.txt(OK, body);
			case _: return Response.txt(NotFound, 'not found');
		};
	};

	server.listen(512, () -> {
		trace('Server is listening on ${server.host.toString()}:${server.port} hehe');
	});
}

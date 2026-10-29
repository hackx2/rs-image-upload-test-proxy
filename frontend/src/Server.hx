import go.Go;
import go.Log;
import go.net.Url;
import go.net.http.Dir;
import go.net.http.Httputil;
import go.github_com.gin_gonic.Gin;

import haxe.io.Bytes;
import haxe.io.BytesOutput;
import haxe.http.HttpStatus;

function main() {
	final backend_url = 'http://${EnvMacro.env('BACKEND_IP')}:${EnvMacro.env('BACKEND_PORT')}';

	final r = Gin._default();

	r.loadHTMLFiles('../../public/upload.html');

	r.GET("/", (c) -> {
		c.HTML(HttpStatus.OK, "upload.html", ["title" => "heart"]);
	});

	final target = (Url.parse(backend_url))!;
	final proxy = Httputil.newSingleHostReverseProxy(target);
	r.POST("/upload", Gin.wrapH(proxy));
	
	r.GET("/data/*f", (c) -> {
		c.redirect(HttpStatus.Found, '${backend_url}/data/${c.param("f")}');
	});

	final err = r.run('${EnvMacro.env('FRONTEND_IP')}:${EnvMacro.env('FRONTEND_PORT')}');
	if (err != null) {
		Log.fatalf("failed to run server: %v", [err]);
	}
}

package;

import haxe.macro.Expr;
import haxe.io.Path;
import sys.io.File;

// ported from: https://github.com/benmerckx/dotenv/
using StringTools;

final class EnvMacro {
	@:persistent static var loaded:Map<String, String> = new Map();

	public static macro function env(name:String, byDefault:String = null):Expr {
		return macro $v{
			switch Sys.getEnv(name) {
				case null: switch loaded[name] {
						case null: byDefault;
						case v: v;
					}
				case v: v;
			}
		}
	}

	public static function load(path = '.', filename = '.env') {
		var content = File.getContent(Path.join([path, filename]));
		for (key => value in parse(content))
			loaded[key] = value;
	}

	public static function parse(content:String):Map<String, String> {
		var result = new Map();
		var matcher = ~/^\s*([\w\.\-]+)\s*=\s*(.*)?\s*$/;
		for (line in content.split('\n')) {
			if (matcher.match(line)) {
				var key = matcher.matched(1).trim(),
					value = matcher.matched(2);
				value = value == null ? '' : value;
				value = value.trim();
				if (value.length > 0 && value.charAt(0) == '"' && value.charAt(value.length - 1) == '"') {
					value = value.replace('\\n', '\n');
					value = value.substr(1, value.length - 2);
				}
				result.set(key, value);
			}
		}
		return result;
	}
}

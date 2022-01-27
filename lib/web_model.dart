import 'dart:typed_data';
import 'dart:js' as js;
import 'common.dart';

class PlatformFFI {
  static void clearRgbaFrame() {}

  static Uint8List getRgba() {
    var data = js.context.callMethod('getRgba');
    if (data) return Uint8List.sublistView(data);
    return null;
  }

  static Future<String> getVersion() async {
    return '';
  }

  static String getByName(String name, [String arg = '']) {
    return js.context.callMethod('getByName', [name, arg]);
  }

  static void setByName(String name, [String value = '']) {
    js.context.callMethod('setByName', [name, value]);
  }

  static Future<Null> init() async {
    isWeb = true;
    js.context.callMethod('init');
  }
}

final localeName = js.context.callMethod('getLanguage') as String;

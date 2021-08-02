import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/gestures.dart';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:device_info/device_info.dart';
import 'dart:io';
import 'dart:math';
import 'dart:ffi';
import 'dart:convert';
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:tuple/tuple.dart';
import 'dart:async';
import 'common.dart';

class RgbaFrame extends Struct {
  @Uint32()
  int len;
  Pointer<Uint8> data;
}

typedef F2 = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);
typedef F3 = void Function(Pointer<Utf8>, Pointer<Utf8>);
typedef F4 = void Function(Pointer<RgbaFrame>);
typedef F5 = Pointer<RgbaFrame> Function();

class FfiModel with ChangeNotifier {
  PeerInfo _pi;
  Display _display;
  var _decoding = false;
  bool _waitForImage;
  bool _initialized = false;
  final _permissions = Map<String, bool>();
  bool _secure;
  bool _direct;

  get permissions => _permissions;
  get initialized => _initialized;
  get display => _display;
  get secure => _secure;
  get direct => _direct;
  get pi => _pi;

  FfiModel() {
    Translator.call = translate;
    clear();
    () async {
      await FFI.init();
      _initialized = true;
      notifyListeners();
    }();
  }

  void updatePermission(Map<String, dynamic> evt) {
    evt.forEach((k, v) {
      if (k == 'name') return;
      _permissions[k] = v == 'true';
    });
    print('$_permissions');
  }

  bool keyboard() => _permissions['keyboard'] != false;

  void clear() {
    _pi = PeerInfo();
    _display = Display();
    _waitForImage = false;
    _secure = null;
    _direct = null;
    clearPermissions();
  }

  void setConnectionType(bool secure, bool direct) {
    _secure = secure;
    _direct = direct;
  }

  Image getConnectionImage() {
    String icon;
    if (secure == true && direct == true) {
      icon = 'secure';
    } else if (secure == false && direct == true) {
      icon = 'insecure';
    } else if (secure == false && direct == false) {
      icon = 'insecure_relay';
    } else if (secure == true && direct == false) {
      icon = 'secure_relay';
    }
    return icon == null
        ? null
        : Image.asset('assets/$icon.png', width: 48, height: 48);
  }

  void clearPermissions() {
    _permissions.clear();
  }

  void update(
      String id,
      BuildContext context,
      void Function(
    Map<String, dynamic> evt,
    String id,
  )
          handleMsgbox) {
    var pos;
    for (;;) {
      var evt = FFI.popEvent();
      if (evt == null) break;
      var name = evt['name'];
      if (name == 'msgbox') {
        handleMsgbox(evt, id);
      } else if (name == 'peer_info') {
        handlePeerInfo(evt, context);
      } else if (name == 'connection_ready') {
        FFI.ffiModel.setConnectionType(
            evt['secure'] == 'true', evt['direct'] == 'true');
      } else if (name == 'switch_display') {
        handleSwitchDisplay(evt);
      } else if (name == 'cursor_data') {
        FFI.cursorModel.updateCursorData(evt);
      } else if (name == 'cursor_id') {
        FFI.cursorModel.updateCursorId(evt);
      } else if (name == 'cursor_position') {
        pos = evt;
      } else if (name == 'clipboard') {
        Clipboard.setData(ClipboardData(text: evt['content']));
      } else if (name == 'permission') {
        FFI.ffiModel.updatePermission(evt);
      }
    }
    if (pos != null) FFI.cursorModel.updateCursorPosition(pos);
    if (!_decoding) {
      var rgba = FFI.getRgba();
      if (rgba != null) {
        if (_waitForImage) {
          _waitForImage = false;
          dismissLoading();
        }
        _decoding = true;
        final pid = FFI.id;
        ui.decodeImageFromPixels(
            rgba, _display.width, _display.height, ui.PixelFormat.bgra8888,
            (image) {
          FFI.clearRgbaFrame();
          _decoding = false;
          if (FFI.id != pid) return;
          try {
            // my throw exception, because the listener maybe already dispose
            FFI.imageModel.update(image);
          } catch (e) {
            print('update image: $e');
          }
        });
      }
    }
  }

  void handleSwitchDisplay(Map<String, dynamic> evt) {
    var old = _pi.currentDisplay;
    _pi.currentDisplay = int.parse(evt['display']);
    _display.x = double.parse(evt['x']);
    _display.y = double.parse(evt['y']);
    _display.width = int.parse(evt['width']);
    _display.height = int.parse(evt['height']);
    if (old != _pi.currentDisplay)
      FFI.cursorModel.updateDisplayOrigin(_display.x, _display.y);
    notifyListeners();
  }

  void handlePeerInfo(Map<String, dynamic> evt, BuildContext context) {
    dismissLoading();
    _pi.version = evt['version'];
    _pi.username = evt['username'];
    _pi.hostname = evt['hostname'];
    _pi.platform = evt['platform'];
    _pi.sasEnabled = evt['sas_enabled'] == "true";
    _pi.currentDisplay = int.parse(evt['current_display']);
    List<dynamic> displays = json.decode(evt['displays']);
    _pi.displays = [];
    for (int i = 0; i < displays.length; ++i) {
      Map<String, dynamic> d0 = displays[i];
      var d = Display();
      d.x = d0['x'].toDouble();
      d.y = d0['y'].toDouble();
      d.width = d0['width'];
      d.height = d0['height'];
      _pi.displays.add(d);
    }
    if (_pi.currentDisplay < _pi.displays.length) {
      _display = _pi.displays[_pi.currentDisplay];
      initializeCursorAndCanvas();
    }
    if (displays.length > 0) {
      showLoading(translate('Connected, waiting for image...'), context);
      _waitForImage = true;
    }
    notifyListeners();
  }
}

class ImageModel with ChangeNotifier {
  ui.Image _image;

  ui.Image get image => _image;

  void update(ui.Image image) {
    if (_image == null && image != null) {
      final size = MediaQueryData.fromWindow(ui.window).size;
      final xscale = size.width / image.width;
      final yscale = size.height / image.height;
      FFI.canvasModel.scale = max(xscale, yscale);
    }
    _image = image;
    if (image != null) notifyListeners();
  }

  double get maxScale {
    if (_image == null) return 1.0;
    final size = MediaQueryData.fromWindow(ui.window).size;
    final xscale = size.width / _image.width;
    final yscale = size.height / _image.height;
    return max(1.0, max(xscale, yscale));
  }

  double get minScale {
    if (_image == null) return 1.0;
    final size = MediaQueryData.fromWindow(ui.window).size;
    final xscale = size.width / _image.width;
    final yscale = size.height / _image.height;
    return min(xscale, yscale);
  }
}

class CanvasModel with ChangeNotifier {
  double _x;
  double _y;
  double _scale;

  CanvasModel() {
    clear();
  }

  double get x => _x;
  double get y => _y;
  double get scale => _scale;

  void update(double x, double y, double scale) {
    _x = x;
    _y = y;
    _scale = scale;
    notifyListeners();
  }

  set scale(v) {
    _scale = v;
    notifyListeners();
  }

  void panX(double dx) {
    _x += dx;
    notifyListeners();
  }

  void resetOffset() {
    _x = 0;
    _y = 0;
    notifyListeners();
  }

  void panY(double dy) {
    _y += dy;
    notifyListeners();
  }

  void updateScale(double v) {
    if (FFI.imageModel.image == null) return;
    final offset = FFI.cursorModel.offset;
    var r = FFI.cursorModel.getVisibleRect();
    final px0 = (offset.dx - r.left) * _scale;
    final py0 = (offset.dy - r.top) * _scale;
    _scale *= v;
    final maxs = FFI.imageModel.maxScale;
    final mins = FFI.imageModel.minScale;
    if (_scale > maxs) _scale = maxs;
    if (_scale < mins) _scale = mins;
    r = FFI.cursorModel.getVisibleRect();
    final px1 = (offset.dx - r.left) * _scale;
    final py1 = (offset.dy - r.top) * _scale;
    _x -= px1 - px0;
    _y -= py1 - py0;
    notifyListeners();
  }

  void clear() {
    _x = 0;
    _y = 0;
    _scale = 1.0;
  }
}

class CursorModel with ChangeNotifier {
  ui.Image _image;
  final _images = Map<int, Tuple3<ui.Image, double, double>>();
  double _x = -10000;
  double _y = -10000;
  double _hotx = 0;
  double _hoty = 0;
  double _displayOriginX = 0;
  double _displayOriginY = 0;

  ui.Image get image => _image;
  double get x => _x - _displayOriginX;
  double get y => _y - _displayOriginY;
  Offset get offset => Offset(_x, _y);
  double get hotx => _hotx;
  double get hoty => _hoty;

  // remote physical display coordinate
  Rect getVisibleRect() {
    final size = MediaQueryData.fromWindow(ui.window).size;
    final xoffset = FFI.canvasModel.x;
    final yoffset = FFI.canvasModel.y;
    final scale = FFI.canvasModel.scale;
    final x0 = _displayOriginX - xoffset / scale;
    final y0 = _displayOriginY - yoffset / scale;
    return Rect.fromLTWH(x0, y0, size.width / scale, size.height / scale);
  }

  double adjustForKeyboard() {
    final m = MediaQueryData.fromWindow(ui.window);
    var keyboardHeight = m.viewInsets.bottom;
    final size = m.size;
    if (keyboardHeight < 100) return 0;
    final s = FFI.canvasModel.scale;
    final thresh = (size.height - keyboardHeight) / 2;
    var h = (_y - getVisibleRect().top) * s; // local physical display height
    return h - thresh;
  }

  void updatePan(double dx, double dy) {
    if (FFI.imageModel.image == null) return;
    final scale = FFI.canvasModel.scale;
    dx /= scale;
    dy /= scale;
    final r = getVisibleRect();
    var cx = r.center.dx;
    var cy = r.center.dy;
    var tryMoveCanvasX = false;
    if (dx > 0) {
      final maxCanvasCanMove =
          _displayOriginX + FFI.imageModel.image.width - r.right;
      tryMoveCanvasX = _x + dx > cx && maxCanvasCanMove > 0;
      if (tryMoveCanvasX) {
        dx = min(dx, maxCanvasCanMove);
      } else {
        final maxCursorCanMove = r.right - _x;
        dx = min(dx, maxCursorCanMove);
      }
    } else if (dx < 0) {
      final maxCanvasCanMove = _displayOriginX - r.left;
      tryMoveCanvasX = _x + dx < cx && maxCanvasCanMove < 0;
      if (tryMoveCanvasX) {
        dx = max(dx, maxCanvasCanMove);
      } else {
        final maxCursorCanMove = r.left - _x;
        dx = max(dx, maxCursorCanMove);
      }
    }
    var tryMoveCanvasY = false;
    if (dy > 0) {
      final mayCanvasCanMove =
          _displayOriginY + FFI.imageModel.image.height - r.bottom;
      tryMoveCanvasY = _y + dy > cy && mayCanvasCanMove > 0;
      if (tryMoveCanvasY) {
        dy = min(dy, mayCanvasCanMove);
      } else {
        final mayCursorCanMove = r.bottom - _y;
        dy = min(dy, mayCursorCanMove);
      }
    } else if (dy < 0) {
      final mayCanvasCanMove = _displayOriginY - r.top;
      tryMoveCanvasY = _y + dy < cy && mayCanvasCanMove < 0;
      if (tryMoveCanvasY) {
        dy = max(dy, mayCanvasCanMove);
      } else {
        final mayCursorCanMove = r.top - _y;
        dy = max(dy, mayCursorCanMove);
      }
    }

    if (dx == 0 && dy == 0) return;
    _x += dx;
    _y += dy;
    if (tryMoveCanvasX && dx != 0) {
      FFI.canvasModel.panX(-dx);
    }
    if (tryMoveCanvasY && dy != 0) {
      FFI.canvasModel.panY(-dy);
    }

    FFI.moveMouse(_x, _y);
    notifyListeners();
  }

  void updateCursorData(Map<String, dynamic> evt) {
    var id = int.parse(evt['id']);
    _hotx = double.parse(evt['hotx']);
    _hoty = double.parse(evt['hoty']);
    var width = int.parse(evt['width']);
    var height = int.parse(evt['height']);
    List<dynamic> colors = json.decode(evt['colors']);
    final rgba = Uint8List.fromList(colors.map((s) => s as int).toList());
    var pid = FFI.id;
    ui.decodeImageFromPixels(rgba, width, height, ui.PixelFormat.rgba8888,
        (image) {
      if (FFI.id != pid) return;
      _image = image;
      _images[id] = Tuple3(image, _hotx, _hoty);
      try {
        // my throw exception, because the listener maybe already dispose
        notifyListeners();
      } catch (e) {
        print('notify cursor: $e');
      }
    });
  }

  void updateCursorId(Map<String, dynamic> evt) {
    final tmp = _images[int.parse(evt['id'])];
    if (tmp != null) {
      _image = tmp.item1;
      _hotx = tmp.item2;
      _hoty = tmp.item3;
      notifyListeners();
    }
  }

  void updateCursorPosition(Map<String, dynamic> evt) {
    _x = double.parse(evt['x']);
    _y = double.parse(evt['y']);
    notifyListeners();
  }

  void updateDisplayOrigin(double x, double y) {
    _displayOriginX = x;
    _displayOriginY = y;
    _x = x + 1;
    _y = y + 1;
    FFI.moveMouse(x, y);
    FFI.canvasModel.resetOffset();
    notifyListeners();
  }

  void updateDisplayOriginWithCursor(
      double x, double y, double xCursor, double yCursor) {
    _displayOriginX = x;
    _displayOriginY = y;
    _x = xCursor;
    _y = yCursor;
    FFI.moveMouse(x, y);
    notifyListeners();
  }

  void clear() {
    _x = -10000;
    _x = -10000;
    _image = null;
    _images.clear();
  }
}

class FFI {
  static String id = "";
  static String _dir = '';
  static F2 _getByName;
  static F3 _setByName;
  static F4 _freeRgba;
  static F5 _getRgba;
  static Pointer<RgbaFrame> _lastRgbaFrame;
  static var shift = false;
  static var ctrl = false;
  static var alt = false;
  static var command = false;
  static final imageModel = ImageModel();
  static final ffiModel = FfiModel();
  static final cursorModel = CursorModel();
  static final canvasModel = CanvasModel();

  static String getId() {
    return getByName('remote_id');
  }

  static void tap(bool right) {
    sendMouse('down', right ? 'right' : 'left');
    sendMouse('up', right ? 'right' : 'left');
  }

  static void scroll(double y) {
    var y2 = y.round();
    if (y2 == 0) return;
    setByName('send_mouse',
        json.encode(modify({'type': 'wheel', 'y': y2.toString()})));
  }

  static void reconnect() {
    setByName('reconnect');
    FFI.ffiModel.clearPermissions();
  }

  static void resetModifiers() {
    shift = ctrl = alt = command = false;
  }

  static Map<String, String> modify(Map<String, String> evt) {
    if (ctrl) evt['ctrl'] = 'true';
    if (shift) evt['shift'] = 'true';
    if (alt) evt['alt'] = 'true';
    if (command) evt['command'] = 'true';
    return evt;
  }

  static void sendMouse(String type, String buttons) {
    if (!ffiModel.keyboard()) return;
    setByName(
        'send_mouse', json.encode(modify({'type': type, 'buttons': buttons})));
  }

  static void inputKey(String name) {
    if (!ffiModel.keyboard()) return;
    setByName('input_key', json.encode(modify({'name': name})));
  }

  static void moveMouse(double x, double y) {
    if (!ffiModel.keyboard()) return;
    var x2 = x.toInt();
    var y2 = y.toInt();
    setByName('send_mouse', json.encode(modify({'x': '$x2', 'y': '$y2'})));
  }

  static List<Peer> peers() {
    try {
      List<dynamic> peers = json.decode(getByName('peers'));
      return peers
          .map((s) => s as List<dynamic>)
          .map((s) =>
              Peer.fromJson(s[0] as String, s[1] as Map<String, dynamic>))
          .toList();
    } catch (e) {
      print('peers(): $e');
    }
    return [];
  }

  static void connect(String id) {
    setByName('connect', id);
    FFI.id = id;
  }

  static void clearRgbaFrame() {
    if (_lastRgbaFrame != null && _lastRgbaFrame != nullptr)
      _freeRgba(_lastRgbaFrame);
  }

  static Uint8List getRgba() {
    _lastRgbaFrame = _getRgba();
    if (_lastRgbaFrame == null || _lastRgbaFrame == nullptr) return null;
    final ref = _lastRgbaFrame.ref;
    return Uint8List.sublistView(ref.data.asTypedList(ref.len));
  }

  static Map<String, dynamic> popEvent() {
    var s = getByName('event');
    if (s == '') return null;
    try {
      Map<String, dynamic> event = json.decode(s);
      return event;
    } catch (e) {
      print('popEvent(): $e');
    }
    return null;
  }

  static void login(String password, bool remember) {
    setByName(
        'login',
        json.encode({
          'password': password,
          'remember': remember ? 'true' : 'false',
        }));
  }

  static void close() {
    savePreference(id, cursorModel.x, cursorModel.y, canvasModel.x,
        canvasModel.y, canvasModel.scale, ffiModel.pi.currentDisplay);
    id = "";
    setByName('close', '');
    imageModel.update(null);
    cursorModel.clear();
    ffiModel.clear();
    canvasModel.clear();
    resetModifiers();
  }

  static void setByName(String name, [String value = '']) {
    var a = name.toNativeUtf8();
    var b = value.toNativeUtf8();
    _setByName(a, b);
    calloc.free(a);
    calloc.free(b);
  }

  static String getByName(String name, [String arg = '']) {
    var a = name.toNativeUtf8();
    var b = arg.toNativeUtf8();
    var p = _getByName(a, b);
    assert(p != nullptr && p != null);
    var res = p.toDartString();
    calloc.free(p);
    calloc.free(a);
    calloc.free(b);
    return res;
  }

  static Future<Null> init() async {
    final dylib = Platform.isAndroid
        ? DynamicLibrary.open('librustdesk.so')
        : DynamicLibrary.process();
    _getByName = dylib.lookupFunction<F2, F2>('get_by_name');
    _setByName =
        dylib.lookupFunction<Void Function(Pointer<Utf8>, Pointer<Utf8>), F3>(
            'set_by_name');
    _freeRgba = dylib
        .lookupFunction<Void Function(Pointer<RgbaFrame>), F4>('free_rgba');
    _getRgba = dylib.lookupFunction<F5, F5>('get_rgba');
    _dir = (await getApplicationDocumentsDirectory()).path;
    DeviceInfoPlugin deviceInfo = DeviceInfoPlugin();
    AndroidDeviceInfo androidInfo = await deviceInfo.androidInfo;
    final name = '${androidInfo.brand}-${androidInfo.model}';
    final id = androidInfo.id;
    setByName('info1', id);
    setByName('info2', name);
    setByName('init', _dir);
  }
}

class Peer {
  final String id;
  final String username;
  final String hostname;
  final String platform;

  Peer.fromJson(String id, Map<String, dynamic> json)
      : id = id,
        username = json['username'],
        hostname = json['hostname'],
        platform = json['platform'];
}

class Display {
  double x = 0;
  double y = 0;
  int width = 0;
  int height = 0;
}

class PeerInfo {
  String version;
  String username;
  String hostname;
  String platform;
  bool sasEnabled;
  int currentDisplay;
  List<Display> displays;
}

void savePreference(String id, double xCursor, double yCursor, double xCanvas,
    double yCanvas, double scale, int currentDisplay) async {
  SharedPreferences prefs = await SharedPreferences.getInstance();
  final p = Map<String, dynamic>();
  p['xCursor'] = xCursor;
  p['yCursor'] = yCursor;
  p['xCanvas'] = xCanvas;
  p['yCanvas'] = yCanvas;
  p['scale'] = scale;
  p['currentDisplay'] = currentDisplay;
  prefs.setString('peer' + id, json.encode(p));
}

Future<Map<String, dynamic>> getPreference(String id) async {
  SharedPreferences prefs = await SharedPreferences.getInstance();
  var p = prefs.getString('peer' + id);
  if (p == null) return null;
  Map<String, dynamic> m = json.decode(p);
  return m;
}

Future<String> getPassword(String id) async {
  SharedPreferences prefs = await SharedPreferences.getInstance();
  var p = prefs.getString('peer' + id + '-password');
  if (p == null) return "";
  return p;
}

void savePassword(String id, String password) async {
  SharedPreferences prefs = await SharedPreferences.getInstance();
  prefs.setString('peer' + id + '-password', password);
}

void initializeCursorAndCanvas() async {
  var p = await getPreference(FFI.id);
  int currentDisplay = 0;
  if (p != null) {
    currentDisplay = p['currentDisplay'];
  }
  if (p == null || currentDisplay != FFI.ffiModel.pi.currentDisplay) {
    FFI.cursorModel
        .updateDisplayOrigin(FFI.ffiModel.display.x, FFI.ffiModel.display.y);
    return;
  }
  double xCursor = p['xCursor'];
  double yCursor = p['yCursor'];
  double xCanvas = p['xCanvas'];
  double yCanvas = p['yCanvas'];
  double scale = p['scale'];
  FFI.cursorModel.updateDisplayOriginWithCursor(
      FFI.ffiModel.display.x, FFI.ffiModel.display.y, xCursor, yCursor);
  FFI.canvasModel.update(xCanvas, yCanvas, scale);
}

final bool isCn = Platform.localeName.endsWith('CN');

final langs = <String, Map<String, String>>{
  'cn': <String, String>{
    'Remote ID': '远程ID',
    'Paste': '粘贴',
    'Are you sure to close the connection?': '是否确认关闭连接？',
  },
  'en': <String, String>{}
};

String translate(String name) {
  if (name.startsWith('Failed') && name.contains(':')) {
    return name.split(': ').map((x) => translate(x)).join(': ');
  }
  final tmp = isCn ? langs['cn'] : langs['en'];
  final v = tmp[name];
  if (v == null) {
    var a = 'translate';
    var b = '{"locale": "${Platform.localeName}", "text": "${name}"}';
    return FFI.getByName(a, b);
  } else {
    return v;
  }
}

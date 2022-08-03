import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:device_info_plus/device_info_plus.dart';
import 'package:external_path/external_path.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:path_provider/path_provider.dart';

import '../common.dart';
import '../generated_bridge.dart';

class RgbaFrame extends Struct {
  @Uint32()
  external int len;
  external Pointer<Uint8> data;
}

typedef F2 = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);
typedef F3 = void Function(Pointer<Utf8>, Pointer<Utf8>);
typedef HandleEvent = void Function(Map<String, dynamic> evt);

/// FFI wrapper around the native Rust core.
/// Hides the platform differences.
class PlatformFFI {
  String _dir = '';
  String _homeDir = '';
  F2? _getByName;
  F3? _setByName;
  var _eventHandlers = Map<String, Map<String, HandleEvent>>();
  late RustdeskImpl _ffiBind;
  late String _appType;
  void Function(Map<String, dynamic>)? _eventCallback;

  PlatformFFI._();

  static final PlatformFFI instance = PlatformFFI._();
  final _toAndroidChannel = MethodChannel("mChannel");

  RustdeskImpl get ffiBind => _ffiBind;

  static get localeName => Platform.localeName;

  static Future<String> getVersion() async {
    PackageInfo packageInfo = await PackageInfo.fromPlatform();
    return packageInfo.version;
  }

  bool registerEventHandler(
      String event_name, String handler_name, HandleEvent handler) {
    debugPrint('registerEventHandler $event_name $handler_name');
    var handlers = _eventHandlers[event_name];
    if (handlers == null) {
      _eventHandlers[event_name] = {handler_name: handler};
      return true;
    } else {
      if (handlers.containsKey(handler_name)) {
        return false;
      } else {
        handlers[handler_name] = handler;
        return true;
      }
    }
  }

  void unregisterEventHandler(String event_name, String handler_name) {
    debugPrint('unregisterEventHandler $event_name $handler_name');
    var handlers = _eventHandlers[event_name];
    if (handlers != null) {
      handlers.remove(handler_name);
    }
  }

  /// Send **get** command to the Rust core based on [name] and [arg].
  /// Return the result as a string.
  String getByName(String name, [String arg = '']) {
    if (_getByName == null) return '';
    var a = name.toNativeUtf8();
    var b = arg.toNativeUtf8();
    var p = _getByName!(a, b);
    assert(p != nullptr);
    var res = p.toDartString();
    calloc.free(p);
    calloc.free(a);
    calloc.free(b);
    return res;
  }

  /// Send **set** command to the Rust core based on [name] and [value].
  void setByName(String name, [String value = '']) {
    if (_setByName == null) return;
    var a = name.toNativeUtf8();
    var b = value.toNativeUtf8();
    _setByName!(a, b);
    calloc.free(a);
    calloc.free(b);
  }

  /// Init the FFI class, loads the native Rust core library.
  Future<Null> init(String appType) async {
    _appType = appType;
    // if (isDesktop) {
    //   // TODO
    //   return;
    // }
    final dylib = Platform.isAndroid
        ? DynamicLibrary.open('librustdesk.so')
        : Platform.isLinux
            ? DynamicLibrary.open("/usr/lib/rustdesk/librustdesk.so")
            : Platform.isWindows
                ? DynamicLibrary.open("librustdesk.dll")
                : Platform.isMacOS
                    ? DynamicLibrary.open("librustdesk.dylib")
                    : DynamicLibrary.process();
    debugPrint('initializing FFI ${_appType}');
    try {
      _getByName = dylib.lookupFunction<F2, F2>('get_by_name');
      _setByName =
          dylib.lookupFunction<Void Function(Pointer<Utf8>, Pointer<Utf8>), F3>(
              'set_by_name');
      _dir = (await getApplicationDocumentsDirectory()).path;
      _ffiBind = RustdeskImpl(dylib);
      _startListenEvent(_ffiBind); // global event
      try {
        if (isAndroid) {
          // only support for android
          _homeDir = (await ExternalPath.getExternalStorageDirectories())[0];
        } else {
          _homeDir = (await getDownloadsDirectory())?.path ?? "";
        }
      } catch (e) {
        print(e);
      }
      String id = 'NA';
      String name = 'Flutter';
      DeviceInfoPlugin deviceInfo = DeviceInfoPlugin();
      if (Platform.isAndroid) {
        AndroidDeviceInfo androidInfo = await deviceInfo.androidInfo;
        name = '${androidInfo.brand}-${androidInfo.model}';
        id = androidInfo.id.hashCode.toString();
        androidVersion = androidInfo.version.sdkInt ?? 0;
      } else if (Platform.isIOS) {
        IosDeviceInfo iosInfo = await deviceInfo.iosInfo;
        name = iosInfo.utsname.machine ?? "";
        id = iosInfo.identifierForVendor.hashCode.toString();
      } else if (Platform.isLinux) {
        LinuxDeviceInfo linuxInfo = await deviceInfo.linuxInfo;
        name = linuxInfo.name;
        id = linuxInfo.machineId ?? linuxInfo.id;
      } else if (Platform.isWindows) {
        WindowsDeviceInfo winInfo = await deviceInfo.windowsInfo;
        name = winInfo.computerName;
        id = winInfo.computerName;
      } else if (Platform.isMacOS) {
        MacOsDeviceInfo macOsInfo = await deviceInfo.macOsInfo;
        name = macOsInfo.computerName;
        id = macOsInfo.systemGUID ?? "";
      }
      print(
          "_appType:$_appType,info1-id:$id,info2-name:$name,dir:$_dir,homeDir:$_homeDir");
      setByName('info1', id);
      setByName('info2', name);
      setByName('home_dir', _homeDir);
      setByName('init', _dir);
    } catch (e) {
      print(e);
    }
    version = await getVersion();
  }

  bool _tryHandle(Map<String, dynamic> evt) {
    final name = evt['name'];
    if (name != null) {
      final handlers = _eventHandlers[name];
      if (handlers != null) {
        if (handlers.isNotEmpty) {
          handlers.values.forEach((handler) {
            handler(evt);
          });
          return true;
        }
      }
    }
    return false;
  }

  /// Start listening to the Rust core's events and frames.
  void _startListenEvent(RustdeskImpl rustdeskImpl) {
    () async {
      await for (final message
          in rustdeskImpl.startGlobalEventStream(appType: _appType)) {
        try {
          Map<String, dynamic> event = json.decode(message);
          // _tryHandle here may be more flexible than _eventCallback
          if (!_tryHandle(event)) {
            if (_eventCallback != null) {
              _eventCallback!(event);
            }
          }
        } catch (e) {
          print('json.decode fail(): $e');
        }
      }
    }();
  }

  void setEventCallback(void Function(Map<String, dynamic>) fun) async {
    _eventCallback = fun;
  }

  void setRgbaCallback(void Function(Uint8List) fun) async {}

  void startDesktopWebListener() {}

  void stopDesktopWebListener() {}

  void setMethodCallHandler(FMethod callback) {
    _toAndroidChannel.setMethodCallHandler((call) async {
      callback(call.method, call.arguments);
      return null;
    });
  }

  invokeMethod(String method, [dynamic arguments]) async {
    if (!isAndroid) return Future<bool>(() => false);
    return await _toAndroidChannel.invokeMethod(method, arguments);
  }
}

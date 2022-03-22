import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/models/native_model.dart';
import '../common.dart';
import '../pages/server_page.dart';
import 'model.dart';

final _emptyIdShow = translate("connecting_status");

class ServerModel with ChangeNotifier {
  Timer? _interval;
  bool _isStart = false;
  bool _mediaOk = false;
  bool _inputOk = false;
  late bool _audioOk;
  late bool _fileOk;
  final _serverId = TextEditingController(text: _emptyIdShow);
  final _serverPasswd = TextEditingController(text: "");

  List<Client> _clients = [];

  bool get isStart => _isStart;

  bool get mediaOk => _mediaOk;

  bool get inputOk => _inputOk;

  bool get audioOk => _audioOk;

  bool get fileOk => _fileOk;

  TextEditingController get serverId => _serverId;

  TextEditingController get serverPasswd => _serverPasswd;

  List<Client> get clients => _clients;

  ServerModel() {
    ()async{
      await Future.delayed(Duration(seconds: 2));
      final audioOption = FFI.getByName('option', 'enable-audio');
      _audioOk = audioOption.isEmpty;   // audio true by default

      final fileOption = FFI.getByName('option', 'enable-file-transfer');
      _fileOk = fileOption.isEmpty;
      Map<String, String> res = Map()
        ..["name"] = "enable-keyboard"
        ..["value"] = 'N';
      FFI.setByName('option', jsonEncode(res)); // input false by default
    }();
  }

  toggleAudio(){
    _audioOk = !_audioOk;
    Map<String, String> res = Map()
      ..["name"] = "enable-audio"
      ..["value"] = _audioOk ? '' : 'N';
    FFI.setByName('option', jsonEncode(res));
    notifyListeners();
  }

  toggleFile() {
    _fileOk = !_fileOk;
    Map<String, String> res = Map()
      ..["name"] = "enable-file-transfer"
      ..["value"] = _fileOk ? '' : 'N';
    FFI.setByName('option', jsonEncode(res));
    notifyListeners();
  }

  toggleInput(){
    if(_inputOk){
      PlatformFFI.invokeMethod("stop_input");
    }else{
      showInputWarnAlert();
    }
  }

  toggleService() async {
    if(_isStart){
      final res = await DialogManager.show<bool>((setState, close) => CustomAlertDialog(
        title: Text("是否关闭"),
        content: Text("关闭录屏服务将自动关闭所有已连接的控制"),
        actions: [
          TextButton(onPressed: ()=>close(), child: Text("Cancel")),
          ElevatedButton(onPressed: ()=>close(true), child: Text("Ok")),
        ],
      ));
      if(res == true){
        stopService();
      }
    }else{
      final res = await DialogManager.show<bool>((setState, close) => CustomAlertDialog(
        title: Text("是否开启录屏服务"),
        content: Text("将自动开启监听服务"),
        actions: [
          TextButton(onPressed: ()=>close(), child: Text("Cancel")),
          ElevatedButton(onPressed: ()=>close(true), child: Text("Ok")),
        ],
      ));
      if(res == true){
        startService();
      }
    }
  }

  Future<Null> startService() async {
    _isStart = true;
    notifyListeners();
    FFI.setByName("ensure_init_event_queue");
    _interval = Timer.periodic(Duration(milliseconds: 30), (timer) {
      FFI.ffiModel.update("", (_, __) {});
    });
    await FFI.invokeMethod("init_service");
    FFI.setByName("start_service");
    getIDPasswd();
  }

  Future<Null> stopService() async {
    _isStart = false;
    _interval?.cancel();
    _interval = null;
    FFI.serverModel.closeAll();
    await FFI.invokeMethod("stop_service");
    FFI.setByName("stop_service");
    notifyListeners();
  }

  Future<Null> initInput() async {
    await FFI.invokeMethod("init_input");
  }

  getIDPasswd() async {
    if (_serverId.text != _emptyIdShow && _serverPasswd.text != "") {
      return;
    }
    var count = 0;
    const maxCount = 10;
    while (count < maxCount) {
      await Future.delayed(Duration(seconds: 1));
      final id = FFI.getByName("server_id");
      final passwd = FFI.getByName("server_password");
      if (id.isEmpty) {
        continue;
      } else {
        _serverId.text = id;
      }

      if (passwd.isEmpty) {
        continue;
      } else {
        _serverPasswd.text = passwd;
      }

      debugPrint(
          "fetch id & passwd again at $count:id:${_serverId.text},passwd:${_serverPasswd.text}");
      count++;
      if (_serverId.text != _emptyIdShow && _serverPasswd.text.isNotEmpty) {
        break;
      }
    }
    notifyListeners();
  }


  changeStatue(String name, bool value) {
    debugPrint("changeStatue value $value");
    switch (name) {
      case "media":
        _mediaOk = value;
        debugPrint("value $value,_isStart:$_isStart");
        if(value && !_isStart){
          startService();
        }
        break;
      case "input":
        if(_inputOk!= value){
          Map<String, String> res = Map()
            ..["name"] = "enable-keyboard"
            ..["value"] = value ? '' : 'N';
          FFI.setByName('option', jsonEncode(res));
        }
        _inputOk = value;
        break;
      default:
        return;
    }
    notifyListeners();
  }

  updateClientState() {
    var res = FFI.getByName("clients_state");
    debugPrint("getByName clients_state string:$res");
    try {
      final List clientsJson = jsonDecode(res);
      _clients = clientsJson
          .map((clientJson) => Client.fromJson(jsonDecode(res)))
          .toList();
      debugPrint("updateClientState:${_clients.toString()}");
      notifyListeners();
    } catch (e) {}
  }

  loginRequest(Map<String, dynamic> evt) {
    try {
      final client = Client.fromJson(jsonDecode(evt["client"]));
      final Map<String, dynamic> response = Map();
      response["id"] = client.id;
      DialogManager.show((setState, close) => CustomAlertDialog(
              title: Text(client.isFileTransfer?"File":"Screen" + "Control Request"),
              content: Column(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(translate("Do you accept?")),
                  SizedBox(height: 20),
                  clientInfo(client),
                  Text(translate("It will be control your device!")),
                ],
              ),
              actions: [
                TextButton(
                    child: Text(translate("Dismiss")),
                    onPressed: () {
                      response["res"] = false;
                      FFI.setByName("login_res", jsonEncode(response));
                      close();
                    }),
                ElevatedButton(
                    child: Text(translate("Accept")),
                    onPressed: () async {
                      response["res"] = true;
                      FFI.setByName("login_res", jsonEncode(response));
                      if (!client.isFileTransfer) {
                        bool res = await FFI.invokeMethod(
                            "start_capture"); // to Android service
                        debugPrint("_toAndroidStartCapture:$res");
                      }
                      _clients.add(client);
                      notifyListeners();
                      close();
                    }),
              ],onWillPop:  ()async=>true,),barrierDismissible: true);
    } catch (e) {
      debugPrint("loginRequest failed,error:$e");
    }
  }

  void onClientAuthorized(Map<String, dynamic> evt) {
    try{
      _clients.add(Client.fromJson(jsonDecode(evt['client'])));
      notifyListeners();
    }catch(e){

    }
  }

  void onClientRemove(Map<String, dynamic> evt) {
    try {
      final id = int.parse(evt['id'] as String);
      Client client = _clients.singleWhere((c) => c.id == id);
      _clients.remove(client);
      notifyListeners();
    } catch (e) {
      // singleWhere fail ,reset the login dialog
      DialogManager.reset();
      debugPrint("onClientRemove failed,error:$e");
    }
  }

  closeAll() {
    _clients.forEach((client) {
      FFI.setByName("close_conn", client.id.toString());
    });
    _clients = [];
  }
}

class Client {
  int id = 0; // client connections inner count id
  bool authorized = false;
  bool isFileTransfer = false;
  String name = "";
  String peerId = ""; // peer user's id,show at app
  bool keyboard = false;
  bool clipboard = false;
  bool audio = false;

  Client(this.authorized, this.isFileTransfer, this.name, this.peerId,this.keyboard,this.clipboard,this.audio);

  Client.fromJson(Map<String, dynamic> json) {
    id = json['id'];
    authorized = json['authorized'];
    isFileTransfer = json['is_file_transfer'];
    name = json['name'];
    peerId = json['peer_id'];
    keyboard= json['keyboard'];
    clipboard= json['clipboard'];
    audio= json['audio'];
  }

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = new Map<String, dynamic>();
    data['id'] = this.id;
    data['is_start'] = this.authorized;
    data['is_file_transfer'] = this.isFileTransfer;
    data['name'] = this.name;
    data['peer_id'] = this.peerId;
    data['keyboard'] = this.keyboard;
    data['clipboard'] = this.clipboard;
    data['audio'] = this.audio;
    return data;
  }
}

showInputWarnAlert() async {
  if (globalKey.currentContext == null) return;
  DialogManager.reset();
  await showDialog<bool>(
      context: globalKey.currentContext!,
      builder: (alertContext) {
        // TODO t
        DialogManager.register(alertContext);
        return AlertDialog(
          title: Text("获取输入权限引导"),
          content: Text.rich(TextSpan(style: TextStyle(), children: [
            TextSpan(text: "请在接下来的系统设置页\n进入"),
            TextSpan(text: " [服务] ", style: TextStyle(color: MyTheme.accent)),
            TextSpan(text: "配置页面\n将"),
            TextSpan(
                text: " [RustDesk Input] ",
                style: TextStyle(color: MyTheme.accent)),
            TextSpan(text: "服务开启")
          ])),
          actions: [
            TextButton(
                child: Text(translate("Do nothing")),
                onPressed: () {
                  DialogManager.reset();
                }),
            ElevatedButton(
                child: Text(translate("Go System Setting")),
                onPressed: () {
                  FFI.serverModel.initInput();
                  DialogManager.reset();
                }),
          ],
        );
      });
  DialogManager.drop();
}

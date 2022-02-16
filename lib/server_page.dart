import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/model.dart';
import 'package:provider/provider.dart';

import 'common.dart';
import 'model.dart';

class ServerPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    checkService();
    return ChangeNotifierProvider.value(
        value: FFI.serverModel,
        child: Scaffold(
            backgroundColor: MyTheme.grayBg,
            appBar: AppBar(
              centerTitle: true,
              title: const Text("Share My Screen"),
              actions: [
                PopupMenuButton<String>(
                    itemBuilder: (context) {
                      return [
                        PopupMenuItem(
                          child: Text(translate("Change ID")),
                          value: "changeID",
                          enabled: false,
                        ),
                        PopupMenuItem(
                          child: Text("Set your own password"),
                          value: "changePW",
                          enabled: false,
                        )
                      ];
                    },
                    onSelected: (value) =>
                        debugPrint("PopupMenuItem onSelected:$value"))
              ],
            ),
            body: SingleChildScrollView(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.start,
                  children: [
                    ServerInfo(),
                    PermissionChecker(),
                    ConnectionManager(),
                    SizedBox.fromSize(size: Size(0, 15.0)), // Bottom padding
                  ],
                ),
              ),
            )));
  }
}

void checkService() {
  // 检测当前服务状态，若已存在服务则异步更新数据回来
  FFI.invokeMethod("check_service"); // jvm
  FFI.serverModel.updateClientState();
}

class ServerInfo extends StatefulWidget {
  @override
  _ServerInfoState createState() => _ServerInfoState();
}

class _ServerInfoState extends State<ServerInfo> {
  var _passwdShow = false;

  // TODO set ID / PASSWORD
  var _serverId = TextEditingController(text: "");
  var _serverPasswd = TextEditingController(text: "");
  var _emptyIdShow = translate("connecting_status");

  @override
  void initState() {
    super.initState();
    var id = FFI.getByName("server_id");
    _serverId.text = id == "" ? _emptyIdShow : id;
    _serverPasswd.text = FFI.getByName("server_password");
    if (_serverId.text == _emptyIdShow || _serverPasswd.text == "") {
      fetchConfigAgain();
    }
  }

  @override
  Widget build(BuildContext context) {
    return myCard(Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        TextFormField(
          readOnly: true,
          style: TextStyle(
              fontSize: 25.0,
              fontWeight: FontWeight.bold,
              color: MyTheme.accent),
          controller: _serverId,
          decoration: InputDecoration(
            icon: const Icon(Icons.perm_identity),
            labelText: translate("ID"),
            labelStyle:
                TextStyle(fontWeight: FontWeight.bold, color: MyTheme.accent50),
          ),
          onSaved: (String value) {},
        ),
        TextFormField(
          readOnly: true,
          obscureText: !_passwdShow,
          style: TextStyle(
              fontSize: 25.0,
              fontWeight: FontWeight.bold,
              color: MyTheme.accent),
          controller: _serverPasswd,
          decoration: InputDecoration(
              icon: const Icon(Icons.lock),
              labelText: translate("Password"),
              labelStyle: TextStyle(
                  fontWeight: FontWeight.bold, color: MyTheme.accent50),
              suffix: IconButton(
                  icon: Icon(Icons.visibility),
                  onPressed: () {
                    setState(() {
                      _passwdShow = !_passwdShow;
                    });
                  })),
          onSaved: (String value) {},
        ),
      ],
    ));
  }

  fetchConfigAgain() async {
    FFI.setByName("start_service");
    var count = 0;
    const maxCount = 10;
    while (count < maxCount) {
      if (_serverId.text != _emptyIdShow && _serverPasswd.text != "") {
        break;
      }
      await Future.delayed(Duration(seconds: 2));
      var id = FFI.getByName("server_id");
      _serverId.text = id == "" ? _emptyIdShow : id;
      _serverPasswd.text = FFI.getByName("server_password");
      debugPrint(
          "fetch id & passwd again at $count:id:${_serverId.text},passwd:${_serverPasswd.text}");
      count++;
    }
    FFI.setByName("stop_service");
  }
}

class PermissionChecker extends StatefulWidget {
  @override
  _PermissionCheckerState createState() => _PermissionCheckerState();
}

class _PermissionCheckerState extends State<PermissionChecker> {
  @override
  void initState() {
    super.initState();
    currentCtx = context;
  }

  @override
  Widget build(BuildContext context) {
    final serverModel = Provider.of<ServerModel>(context);

    return myCard(Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        cardTitle(translate("Configuration Permissions")),
        PermissionRow(translate("Media"), serverModel.mediaOk, _toAndroidInitService),
        const Divider(height: 0),
        PermissionRow(translate("Input"), serverModel.inputOk, _toAndroidInitInput),
        const Divider(),
        serverModel.mediaOk
            ? ElevatedButton.icon(
                icon: Icon(Icons.stop),
                onPressed: _toAndroidStopService,
                label: Text(translate("Stop service")))
            : ElevatedButton.icon(
                icon: Icon(Icons.play_arrow),
                onPressed: _toAndroidInitService,
                label: Text(translate("Start Service"))),
      ],
    ));
  }
}

BuildContext loginReqAlertCtx;

void showLoginReqAlert(BuildContext context, String peerID, String name) async {
  await showDialog(
      context: context,
      builder: (alertContext) {
        loginReqAlertCtx = alertContext;
        return AlertDialog(
          title: Text("Control Request"),
          content:Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(translate("Do you accept?")),
              SizedBox(width: 20),
              Row(
                children: [
                  CircleAvatar(child: Text(name[0])),
                  SizedBox(width: 10),
                  Text(name),
                  SizedBox(width: 5),
                  Text(peerID)
                ],
              ),
            ],
          ),
          actions: [
            TextButton(
                child: Text(translate("Accept")),
                onPressed: () {
                  FFI.setByName("login_res", "true");
                  if (!FFI.serverModel.isFileTransfer) {
                    _toAndroidStartCapture();
                  }
                  FFI.serverModel.setPeer(true);
                  Navigator.of(alertContext).pop();
                }),
            TextButton(
                child: Text(translate("Dismiss")),
                onPressed: () {
                  FFI.setByName("login_res", "false");
                  Navigator.of(alertContext).pop();
                })
          ],
        );
      });
  loginReqAlertCtx = null;
}

clearLoginReqAlert() {
  if (loginReqAlertCtx != null) {
    Navigator.of(loginReqAlertCtx).pop();
    FFI.serverModel.updateClientState();
  }
}

class PermissionRow extends StatelessWidget {
  PermissionRow(this.name, this.isOk, this.onPressed);

  final String name;
  final bool isOk;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text.rich(TextSpan(children: [
          TextSpan(
              text: name + ":  ",
              style: TextStyle(fontSize: 16.0, color: MyTheme.accent50)),
          TextSpan(
              text: isOk ? translate("ON") : translate("OFF"),
              style: TextStyle(
                  fontSize: 16.0, color: isOk ? Colors.green : Colors.grey)),
        ])),
        TextButton(
            onPressed: isOk ? null : onPressed,
            child: Text(
              translate("OPEN"),
              style: TextStyle(fontWeight: FontWeight.bold),
            )),
      ],
    );
  }
}

class ConnectionManager extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final serverModel = Provider.of<ServerModel>(context);
    var info =
        "${serverModel.peerName != "" ? serverModel.peerName : "NA"}-${serverModel.peerID != "" ? serverModel.peerID : "NA"}";
    return serverModel.isPeerStart
        ? myCard(Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              cardTitle("当前连接"),
              Padding(
                padding: EdgeInsets.symmetric(vertical: 5.0),
                child: Text(info, style: TextStyle(color: Colors.grey)),
              ),
              ElevatedButton.icon(
                  style: ButtonStyle(
                      backgroundColor: MaterialStateProperty.all(Colors.red)),
                  icon: Icon(Icons.close),
                  onPressed: () {
                    FFI.setByName("close_conn");
                    // _toAndroidStopCapture();
                    serverModel.setPeer(false);
                  },
                  label: Text("断开连接"))
            ],
          ))
        : SizedBox.shrink();
  }
}

Widget cardTitle(String text) {
  return Padding(
      padding: EdgeInsets.symmetric(vertical: 5.0),
      child: Text(
        text,
        style: TextStyle(
          fontFamily: 'WorkSans',
          fontWeight: FontWeight.bold,
          fontSize: 22,
          color: MyTheme.accent80,
        ),
      ));
}

Widget myCard(Widget child) {
  return Container(
      width: double.maxFinite,
      child: Card(
        margin: EdgeInsets.fromLTRB(15.0, 15.0, 15.0, 0),
        child: Padding(
          padding: EdgeInsets.symmetric(vertical: 15.0, horizontal: 30.0),
          child: child,
        ),
      ));
}

Future<Null> _toAndroidInitService() async {
  bool res = await FFI.invokeMethod("init_service");
  FFI.setByName("start_service");
  debugPrint("_toAndroidInitService:$res");
}

Future<Null> _toAndroidStartCapture() async {
  bool res = await FFI.invokeMethod("start_capture");
  debugPrint("_toAndroidStartCapture:$res");
}

// Future<Null> _toAndroidStopCapture() async {
//   bool res = await FFI.invokeMethod("stop_capture");
//   debugPrint("_toAndroidStopCapture:$res");
// }

Future<Null> _toAndroidStopService() async {
  FFI.setByName("close_conn");
  FFI.serverModel.setPeer(false);

  bool res = await FFI.invokeMethod("stop_service");
  FFI.setByName("stop_service");
  debugPrint("_toAndroidStopSer:$res");
}

Future<Null> _toAndroidInitInput() async {
  bool res = await FFI.invokeMethod("init_input");
  debugPrint("_toAndroidInitInput:$res");
}

void toAndroidChannelInit() {
  FFI.setMethodCallHandler((method, arguments) {
    debugPrint("flutter got android msg");
    try {
      switch (method) {
        case "try_start_without_auth":
          {
            FFI.serverModel.updateClientState();
            debugPrint(
                "pre show loginAlert:${FFI.serverModel.isFileTransfer.toString()}");
            showLoginReqAlert(
                currentCtx, FFI.serverModel.peerID, FFI.serverModel.peerName);
            debugPrint("from jvm:try_start_without_auth done");
            break;
          }
        case "start_capture":
          {
            clearLoginReqAlert();
            FFI.serverModel.updateClientState();
            break;
          }
        case "stop_capture":
          {
            FFI.serverModel.setPeer(false);
            break;
          }
        case "on_permission_changed":
          {
            var name = arguments["name"] as String;
            var value = arguments["value"] as String == "true";
            debugPrint("from jvm:on_permission_changed,$name:$value");
            FFI.serverModel.changeStatue(name, value);
            break;
          }
      }
    } catch (e) {
      debugPrint("MethodCallHandler err:$e");
    }
    return "";
  });
}

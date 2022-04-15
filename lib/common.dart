import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'dart:async';
import 'package:flutter_easyloading/flutter_easyloading.dart';

final globalKey = GlobalKey<NavigatorState>();
final navigationBarKey = GlobalKey();

var isAndroid = false;
var isIOS = false;
var isWeb = false;
var isDesktop = false;
var version = "";
int androidVersion = 0;

typedef F = String Function(String);
typedef FMethod = String Function(String, dynamic);

class Translator {
  static late F call;
}

class MyTheme {
  MyTheme._();

  static const Color grayBg = Color(0xFFEEEEEE);
  static const Color white = Color(0xFFFFFFFF);
  static const Color accent = Color(0xFF0071FF);
  static const Color accent50 = Color(0x770071FF);
  static const Color accent80 = Color(0xAA0071FF);
  static const Color canvasColor = Color(0xFF212121);
  static const Color border = Color(0xFFCCCCCC);
  static const Color idColor = Color(0xFF00B6F0);
  static const Color darkGray = Color(0xFFB9BABC);
}

final ButtonStyle flatButtonStyle = TextButton.styleFrom(
  minimumSize: Size(88, 36),
  padding: EdgeInsets.symmetric(horizontal: 16.0),
  shape: const RoundedRectangleBorder(
    borderRadius: BorderRadius.all(Radius.circular(2.0)),
  ),
);

void showLoading(String text) {
  DialogManager.reset();
  EasyLoading.dismiss();
  EasyLoading.show(
      indicator: Container(
          constraints: BoxConstraints(maxWidth: 240),
          child:
              Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            SizedBox(height: 30),
            Center(child: CircularProgressIndicator()),
            SizedBox(height: 20),
            Center(
                child: Text(Translator.call(text),
                    style: TextStyle(fontSize: 15))),
            SizedBox(height: 20),
            Center(
                child: TextButton(
                    style: flatButtonStyle,
                    onPressed: () {
                      EasyLoading.dismiss();
                      backToHome();
                    },
                    child: Text(Translator.call('Cancel'),
                        style: TextStyle(color: MyTheme.accent))))
          ])),
      maskType: EasyLoadingMaskType.black);
}

backToHome() {
  Navigator.popUntil(globalKey.currentContext!, ModalRoute.withName("/"));
}

typedef DialogBuilder = CustomAlertDialog Function(
    StateSetter setState, Function([dynamic]) close);

class DialogManager {
  static BuildContext? _dialogContext;

  static void reset([result]) {
    if (_dialogContext != null) {
      Navigator.pop(_dialogContext!, result);
    }
    _dialogContext = null;
  }

  static void register(BuildContext dialogContext) {
    _dialogContext = dialogContext;
  }

  static void drop() {
    _dialogContext = null;
  }

  static Future<T?> show<T>(DialogBuilder builder,
      {bool barrierDismissible = false}) async {
    if (globalKey.currentContext == null) return null;
    EasyLoading.dismiss();
    DialogManager.reset();
    final res = await showDialog<T>(
        context: globalKey.currentContext!,
        barrierDismissible: barrierDismissible,
        builder: (context) {
          DialogManager.register(context);
          return StatefulBuilder(
              builder: (_, setState) => builder(setState, DialogManager.reset));
        });
    DialogManager.drop();
    return res;
  }
}

class CustomAlertDialog extends StatelessWidget {
  CustomAlertDialog(
      {required this.title,
      required this.content,
      required this.actions,
      this.onWillPop,
      this.contentPadding});

  final Widget title;
  final Widget content;
  final List<Widget> actions;
  final WillPopCallback? onWillPop;
  final double? contentPadding;

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
        onWillPop: onWillPop ?? () async => false,
        child: AlertDialog(
          title: title,
          contentPadding: EdgeInsets.symmetric(
              horizontal: contentPadding ?? 25, vertical: 10),
          content: content,
          actions: actions,
        ));
  }
}

// EasyLoading
void msgBox(String type, String title, String text, {bool? hasCancel}) {
  var wrap = (String text, void Function() onPressed) => ButtonTheme(
      padding: EdgeInsets.symmetric(horizontal: 20, vertical: 10),
      materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
      //limits the touch area to the button area
      minWidth: 0,
      //wraps child's width
      height: 0,
      child: TextButton(
          style: flatButtonStyle,
          onPressed: onPressed,
          child: Text(Translator.call(text),
              style: TextStyle(color: MyTheme.accent))));

  EasyLoading.dismiss();
  DialogManager.reset();
  final buttons = [
    Expanded(child: Container()),
    wrap(Translator.call('OK'), () {
      EasyLoading.dismiss();
      backToHome();
    })
  ];
  if (hasCancel == null) {
    hasCancel = type != 'error';
  }
  if (hasCancel) {
    buttons.insert(
        1,
        wrap(Translator.call('Cancel'), () {
          EasyLoading.dismiss();
        }));
  }
  EasyLoading.show(
      status: "",
      maskType: EasyLoadingMaskType.black,
      indicator: Container(
          constraints: BoxConstraints(maxWidth: 300),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(Translator.call(title), style: TextStyle(fontSize: 21)),
              SizedBox(height: 20),
              Text(Translator.call(text), style: TextStyle(fontSize: 15)),
              SizedBox(height: 20),
              Row(
                children: buttons,
              )
            ],
          )));
}

Color str2color(String str, [alpha = 0xFF]) {
  var hash = 160 << 16 + 114 << 8 + 91;
  for (var i = 0; i < str.length; i += 1) {
    hash = str.codeUnitAt(i) + ((hash << 5) - hash);
  }
  hash = hash % 16777216;
  return Color((hash & 0xFF7FFF) | (alpha << 24));
}

const K = 1024;
const M = K * K;
const G = M * K;

String readableFileSize(double size) {
  if (size < K) {
    return size.toString() + " B";
  } else if (size < M) {
    return (size / K).toStringAsFixed(2) + " KB";
  } else if (size < G) {
    return (size / M).toStringAsFixed(2) + " MB";
  } else {
    return (size / G).toStringAsFixed(2) + " GB";
  }
}

/// Flutter can't not catch PointerMoveEvent when size is 1
/// This will happen in Android AccessibilityService Input
/// android can't init dispatching size yet ,see: https://stackoverflow.com/questions/59960451/android-accessibility-dispatchgesture-is-it-possible-to-specify-pressure-for-a
/// use this temporary solution until flutter or android fixes the bug
class AccessibilityListener extends StatelessWidget {
  final Widget? child;
  static final offset = 100;

  AccessibilityListener({this.child});

  @override
  Widget build(BuildContext context) {
    return Listener(
        onPointerDown: (evt) {
          if (evt.size == 1 && GestureBinding.instance != null) {
            GestureBinding.instance!.handlePointerEvent(PointerAddedEvent(
                pointer: evt.pointer + offset, position: evt.position));
            GestureBinding.instance!.handlePointerEvent(PointerDownEvent(
                pointer: evt.pointer + offset,
                size: 0.1,
                position: evt.position));
          }
        },
        onPointerUp: (evt) {
          if (evt.size == 1 && GestureBinding.instance != null) {
            GestureBinding.instance!.handlePointerEvent(PointerUpEvent(
                pointer: evt.pointer + offset,
                size: 0.1,
                position: evt.position));
            GestureBinding.instance!.handlePointerEvent(PointerRemovedEvent(
                pointer: evt.pointer + offset, position: evt.position));
          }
        },
        onPointerMove: (evt) {
          if (evt.size == 1 && GestureBinding.instance != null) {
            GestureBinding.instance!.handlePointerEvent(PointerMoveEvent(
                pointer: evt.pointer + offset,
                size: 0.1,
                delta: evt.delta,
                position: evt.position));
          }
        },
        child: child);
  }
}

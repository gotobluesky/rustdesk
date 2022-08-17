import 'dart:convert';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/desktop/pages/desktop_home_page.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:flutter_hbb/models/server_model.dart';
import 'package:get/get.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:url_launcher/url_launcher_string.dart';

const double _kTabWidth = 235;
const double _kTabHeight = 42;
const double _kCardFixedWidth = 560;
const double _kCardLeftMargin = 15;
const double _kContentHMargin = 15;
const double _kContentHSubMargin = _kContentHMargin + 33;
const double _kCheckBoxLeftMargin = 10;
const double _kRadioLeftMargin = 10;
const double _kListViewBottomMargin = 15;
const double _kTitleFontSize = 20;
const double _kContentFontSize = 15;
const Color _accentColor = MyTheme.accent;

class _TabInfo {
  late final int index;
  late final String label;
  late final IconData unselected;
  late final IconData selected;
  _TabInfo(this.index, this.label, this.unselected, this.selected);
}

class DesktopSettingPage extends StatefulWidget {
  DesktopSettingPage({Key? key}) : super(key: key);

  @override
  State<DesktopSettingPage> createState() => _DesktopSettingPageState();
}

class _DesktopSettingPageState extends State<DesktopSettingPage>
    with TickerProviderStateMixin, AutomaticKeepAliveClientMixin {
  final List<_TabInfo> _setting_tabs = <_TabInfo>[
    _TabInfo(
        0, 'User Interface', Icons.language_outlined, Icons.language_sharp),
    _TabInfo(1, 'Security', Icons.enhanced_encryption_outlined,
        Icons.enhanced_encryption_sharp),
    _TabInfo(2, 'Display', Icons.desktop_windows_outlined,
        Icons.desktop_windows_sharp),
    _TabInfo(3, 'Audio', Icons.volume_up_outlined, Icons.volume_up_sharp),
    _TabInfo(4, 'Connection', Icons.link_outlined, Icons.link_sharp),
  ];
  final _TabInfo _about_tab =
      _TabInfo(5, 'About RustDesk', Icons.info_outline, Icons.info_sharp);

  late PageController controller;
  RxInt _selectedIndex = 0.obs;

  @override
  bool get wantKeepAlive => true;

  @override
  void initState() {
    super.initState();
    controller = PageController();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return Scaffold(
      body: Row(
        children: <Widget>[
          Container(
            width: _kTabWidth,
            child: Column(
              children: [
                _header(),
                Flexible(child: _listView(tabs: _setting_tabs)),
                _listItem(tab: _about_tab),
                SizedBox(
                  height: 120,
                )
              ],
            ),
          ),
          const VerticalDivider(thickness: 1, width: 1),
          Expanded(
            child: PageView(
              controller: controller,
              children: [
                _UserInterface(),
                _Safety(),
                _Display(),
                _Audio(),
                _Connection(),
                _About(),
              ],
            ),
          )
        ],
      ),
    );
  }

  Widget _header() {
    return Row(
      children: [
        SizedBox(
          height: 62,
          child: Text(
            translate('Settings'),
            textAlign: TextAlign.left,
            style: TextStyle(
              color: _accentColor,
              fontSize: _kTitleFontSize,
              fontWeight: FontWeight.w400,
            ),
          ),
        ).marginOnly(left: 20, top: 10),
        Spacer(),
      ],
    );
  }

  Widget _listView({required List<_TabInfo> tabs}) {
    return ListView(
      children: tabs.map((tab) => _listItem(tab: tab)).toList(),
    );
  }

  Widget _listItem({required _TabInfo tab}) {
    return Obx(() {
      bool selected = tab.index == _selectedIndex.value;
      return Container(
        width: _kTabWidth,
        height: _kTabHeight,
        child: InkWell(
          onTap: () {
            if (_selectedIndex.value != tab.index) {
              controller.jumpToPage(tab.index);
            }
            _selectedIndex.value = tab.index;
          },
          child: Row(children: [
            Container(
              width: 4,
              height: _kTabHeight * 0.7,
              color: selected ? _accentColor : null,
            ),
            Icon(
              selected ? tab.selected : tab.unselected,
              color: selected ? _accentColor : null,
              size: 20,
            ).marginOnly(left: 13, right: 10),
            Text(
              translate(tab.label),
              style: TextStyle(
                  color: selected ? _accentColor : null,
                  fontWeight: FontWeight.w400,
                  fontSize: _kContentFontSize),
            ),
          ]),
        ),
      );
    });
  }
}

//#region pages

class _UserInterface extends StatefulWidget {
  _UserInterface({Key? key}) : super(key: key);

  @override
  State<_UserInterface> createState() => _UserInterfaceState();
}

class _UserInterfaceState extends State<_UserInterface>
    with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return ListView(
      children: [
        _Card(title: 'Language', children: [language()]),
        _Card(title: 'Theme', children: [theme()]),
      ],
    ).marginOnly(bottom: _kListViewBottomMargin);
  }

  Widget language() {
    return _futureBuilder(future: () async {
      String langs = await bind.mainGetLangs();
      String lang = await bind.mainGetLocalOption(key: "lang");
      return {"langs": langs, "lang": lang};
    }(), hasData: (res) {
      Map<String, String> data = res as Map<String, String>;
      List<dynamic> langsList = jsonDecode(data["langs"]!);
      Map<String, String> langsMap = {for (var v in langsList) v[0]: v[1]};
      List<String> keys = langsMap.keys.toList();
      List<String> values = langsMap.values.toList();
      keys.insert(0, "default");
      values.insert(0, "Default");
      String currentKey = data["lang"]!;
      if (!keys.contains(currentKey)) {
        currentKey = "default";
      }
      return _ComboBox(
        keys: keys,
        values: values,
        initialKey: currentKey,
        onChanged: (key) async {
          await bind.mainSetLocalOption(key: "lang", value: key);
          Get.forceAppUpdate();
        },
      ).marginOnly(left: _kContentHMargin);
    });
  }

  Widget theme() {
    var change = () {
      bool dark = !isDarkTheme();
      Get.changeTheme(dark ? MyTheme.darkTheme : MyTheme.lightTheme);
      Get.find<SharedPreferences>().setString("darkTheme", dark ? "Y" : "");
      Get.forceAppUpdate();
    };

    return GestureDetector(
      child: Row(
        children: [
          Checkbox(value: isDarkTheme(), onChanged: (_) => change()),
          Expanded(child: Text(translate('Dark Theme'))),
        ],
      ).marginOnly(left: _kCheckBoxLeftMargin),
      onTap: change,
    );
  }
}

class _Safety extends StatefulWidget {
  const _Safety({Key? key}) : super(key: key);

  @override
  State<_Safety> createState() => _SafetyState();
}

class _SafetyState extends State<_Safety> with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return ListView(
      children: [
        permissions(),
        password(),
        whitelist(),
      ],
    ).marginOnly(bottom: _kListViewBottomMargin);
  }

  Widget permissions() {
    return _Card(title: 'Permissions', children: [
      _OptionCheckBox('Enable Keyboard/Mouse', 'enable-keyboard'),
      _OptionCheckBox('Enable Clipboard', 'enable-clipboard'),
      _OptionCheckBox('Enable File Transfer', 'enable-file-transfer'),
      _OptionCheckBox('Enable Audio', 'enable-audio'),
      _OptionCheckBox('Enable Remote Restart', 'enable-remote-restart'),
      _OptionCheckBox('Enable remote configuration modification',
          'allow-remote-config-modification'),
    ]);
  }

  Widget password() {
    return ChangeNotifierProvider.value(
        value: gFFI.serverModel,
        child: Consumer<ServerModel>(builder: ((context, model, child) {
          List<String> keys = [
            kUseTemporaryPassword,
            kUsePermanentPassword,
            kUseBothPasswords,
          ];
          List<String> values = [
            translate("Use temporary password"),
            translate("Use permanent password"),
            translate("Use both passwords"),
          ];
          bool tmp_enabled = model.verificationMethod != kUsePermanentPassword;
          bool perm_enabled = model.verificationMethod != kUseTemporaryPassword;
          String currentValue = values[keys.indexOf(model.verificationMethod)];
          List<Widget> radios = values
              .map((value) => _Radio<String>(
                  value: value,
                  groupValue: currentValue,
                  label: value,
                  onChanged: ((value) {
                    model.verificationMethod = keys[values.indexOf(value)];
                  })))
              .toList();

          var onChanged = tmp_enabled
              ? (value) {
                  if (value != null)
                    model.temporaryPasswordLength = value.toString();
                }
              : null;
          List<Widget> lengthRadios = ['6', '8', '10']
              .map((value) => GestureDetector(
                    child: Row(
                      children: [
                        Radio(
                            value: value,
                            groupValue: model.temporaryPasswordLength,
                            onChanged: onChanged),
                        Text(value),
                      ],
                    ).paddingSymmetric(horizontal: 10),
                    onTap: () => onChanged?.call(value),
                  ))
              .toList();

          return _Card(title: 'Password', children: [
            radios[0],
            _SubLabeledWidget(
                'Temporary Password Length',
                Row(
                  children: [
                    ...lengthRadios,
                  ],
                ),
                enabled: tmp_enabled),
            radios[1],
            _SubButton(
                'Set permanent password', setPasswordDialog, perm_enabled),
            radios[2],
          ]);
        })));
  }

  Widget whitelist() {
    return _Card(title: 'IP Whitelisting', children: [
      _Button('IP Whitelisting', changeWhiteList, tip: 'whitelist_tip')
    ]);
  }
}

class _Connection extends StatefulWidget {
  const _Connection({Key? key}) : super(key: key);

  @override
  State<_Connection> createState() => _ConnectionState();
}

class _ConnectionState extends State<_Connection>
    with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return ListView(
      children: [
        _Card(title: 'Server', children: [
          _Button('ID/Relay Server', changeServer),
        ]),
        _Card(title: 'Service', children: [
          _OptionCheckBox('Enable Service', 'stop-service', reverse: true),
          // TODO: Not implemented
          // _option_check('Always connected via relay', 'allow-always-relay'),
          // _option_check('Start ID/relay service', 'stop-rendezvous-service',
          //     reverse: true),
        ]),
        _Card(title: 'TCP Tunneling', children: [
          _OptionCheckBox('Enable TCP Tunneling', 'enable-tunnel'),
        ]),
        direct_ip(),
        _Card(title: 'Proxy', children: [
          _Button('Socks5 Proxy', changeSocks5Proxy),
        ]),
      ],
    ).marginOnly(bottom: _kListViewBottomMargin);
  }

  Widget direct_ip() {
    TextEditingController controller = TextEditingController();
    var update = () => setState(() {});
    RxBool apply_enabled = false.obs;
    return _Card(title: 'Direct IP Access', children: [
      _OptionCheckBox('Enable Direct IP Access', 'direct-server',
          update: update),
      _futureBuilder(
        future: () async {
          String enabled = await bind.mainGetOption(key: 'direct-server');
          String port = await bind.mainGetOption(key: 'direct-access-port');
          return {'enabled': enabled, 'port': port};
        }(),
        hasData: (data) {
          bool enabled =
              option2bool('direct-server', data['enabled'].toString());
          if (!enabled) apply_enabled.value = false;
          controller.text = data['port'].toString();
          return Row(children: [
            _SubLabeledWidget(
              'Port',
              Container(
                width: 80,
                child: TextField(
                  controller: controller,
                  enabled: enabled,
                  onChanged: (_) => apply_enabled.value = true,
                  inputFormatters: [
                    FilteringTextInputFormatter.allow(RegExp(
                        '\^([0-9]|[1-9]\\d|[1-9]\\d{2}|[1-9]\\d{3}|[1-5]\\d{4}|6[0-4]\\d{3}|65[0-4]\\d{2}|655[0-2]\\d|6553[0-5])\$')),
                  ],
                  textAlign: TextAlign.end,
                  decoration: InputDecoration(
                    hintText: '21118',
                    border: InputBorder.none,
                    contentPadding: EdgeInsets.only(right: 5),
                    isCollapsed: true,
                  ),
                ),
              ),
              enabled: enabled,
            ),
            Obx(() => ElevatedButton(
                  onPressed: apply_enabled.value && enabled
                      ? () async {
                          apply_enabled.value = false;
                          await bind.mainSetOption(
                              key: 'direct-access-port',
                              value: controller.text);
                        }
                      : null,
                  child: Text(translate('Apply')),
                ).marginOnly(left: 20))
          ]);
        },
      ),
    ]);
  }
}

class _Display extends StatefulWidget {
  const _Display({Key? key}) : super(key: key);

  @override
  State<_Display> createState() => _DisplayState();
}

class _DisplayState extends State<_Display> with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return ListView(
      children: [
        _Card(title: 'Adaptive Bitrate', children: [
          _OptionCheckBox('Adaptive Bitrate', 'enable-abr'),
        ]),
        hwcodec(),
      ],
    ).marginOnly(bottom: _kListViewBottomMargin);
  }

  Widget hwcodec() {
    return _futureBuilder(
        future: bind.mainHasHwcodec(),
        hasData: (data) {
          return Offstage(
            offstage: !(data as bool),
            child: _Card(title: 'Hardware Codec', children: [
              _OptionCheckBox('Enable hardware codec', 'enable-hwcodec'),
            ]),
          );
        });
  }
}

class _Audio extends StatefulWidget {
  const _Audio({Key? key}) : super(key: key);

  @override
  State<_Audio> createState() => _AudioState();
}

enum _AudioInputType {
  Mute,
  Standard,
  Specify,
}

class _AudioState extends State<_Audio> with AutomaticKeepAliveClientMixin {
  @override
  bool get wantKeepAlive => true;

  @override
  Widget build(BuildContext context) {
    super.build(context);
    var update = () => setState(() {});
    var set_enabled = (bool enabled) => bind.mainSetOption(
        key: 'enable-audio', value: bool2option('enable-audio', enabled));
    var set_device = (String device) =>
        bind.mainSetOption(key: 'audio-input', value: device);
    return ListView(children: [
      _Card(
        title: 'Audio Input',
        children: [
          _futureBuilder(future: () async {
            List<String> devices = await bind.mainGetSoundInputs();
            String current = await bind.mainGetOption(key: 'audio-input');
            String enabled = await bind.mainGetOption(key: 'enable-audio');
            return {'devices': devices, 'current': current, 'enabled': enabled};
          }(), hasData: (data) {
            bool mute =
                !option2bool('enable-audio', data['enabled'].toString());
            String currentDevice = data['current'];
            List<String> devices = (data['devices'] as List<String>).toList();
            _AudioInputType groupValue;
            if (mute) {
              groupValue = _AudioInputType.Mute;
            } else if (devices.contains(currentDevice)) {
              groupValue = _AudioInputType.Specify;
            } else {
              groupValue = _AudioInputType.Standard;
            }
            List deviceWidget = [].toList();
            if (devices.isNotEmpty) {
              var combo = _ComboBox(
                keys: devices,
                values: devices,
                initialKey: devices.contains(currentDevice)
                    ? currentDevice
                    : devices[0],
                onChanged: (key) {
                  set_device(key);
                },
                enabled: groupValue == _AudioInputType.Specify,
              );
              deviceWidget.addAll([
                _Radio<_AudioInputType>(
                  value: _AudioInputType.Specify,
                  groupValue: groupValue,
                  label: 'Specify device',
                  onChanged: (value) {
                    set_device(combo.current);
                    set_enabled(true);
                    update();
                  },
                ),
                combo.marginOnly(left: _kContentHSubMargin, top: 5),
              ]);
            }
            return Column(children: [
              _Radio<_AudioInputType>(
                value: _AudioInputType.Mute,
                groupValue: groupValue,
                label: 'Mute',
                onChanged: (value) {
                  set_enabled(false);
                  update();
                },
              ),
              _Radio(
                value: _AudioInputType.Standard,
                groupValue: groupValue,
                label: 'Use standard device',
                onChanged: (value) {
                  set_device('');
                  set_enabled(true);
                  update();
                },
              ),
              ...deviceWidget,
            ]);
          }),
        ],
      )
    ]).marginOnly(bottom: _kListViewBottomMargin);
  }
}

class _About extends StatefulWidget {
  const _About({Key? key}) : super(key: key);

  @override
  State<_About> createState() => _AboutState();
}

class _AboutState extends State<_About> {
  @override
  Widget build(BuildContext context) {
    return _futureBuilder(future: () async {
      final license = await bind.mainGetLicense();
      final version = await bind.mainGetVersion();
      return {'license': license, 'version': version};
    }(), hasData: (data) {
      final license = data['license'].toString();
      final version = data['version'].toString();
      final linkStyle = TextStyle(decoration: TextDecoration.underline);
      return ListView(children: [
        _Card(title: "About Rustdesk", children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                height: 8.0,
              ),
              Text("Version: $version").marginSymmetric(vertical: 4.0),
              InkWell(
                  onTap: () {
                    launchUrlString("https://rustdesk.com/privacy");
                  },
                  child: Text(
                    "Privacy Statement",
                    style: linkStyle,
                  ).marginSymmetric(vertical: 4.0)),
              InkWell(
                  onTap: () {
                    launchUrlString("https://rustdesk.com");
                  },
                  child: Text(
                    "Website",
                    style: linkStyle,
                  ).marginSymmetric(vertical: 4.0)),
              Container(
                decoration: BoxDecoration(color: Color(0xFF2c8cff)),
                padding: EdgeInsets.symmetric(vertical: 24, horizontal: 8),
                child: Row(
                  children: [
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            "Copyright &copy; 2022 Purslane Ltd.\n$license",
                            style: TextStyle(color: Colors.white),
                          ),
                          Text(
                            "Made with heart in this chaotic world!",
                            style: TextStyle(
                                fontWeight: FontWeight.w800,
                                color: Colors.white),
                          )
                        ],
                      ),
                    ),
                  ],
                ),
              ).marginSymmetric(vertical: 4.0)
            ],
          ).marginOnly(left: _kContentHMargin)
        ]),
      ]).marginOnly(left: _kCardLeftMargin);
    });
  }
}

//#endregion

//#region components

Widget _Card({required String title, required List<Widget> children}) {
  return Row(
    children: [
      Container(
        width: _kCardFixedWidth,
        child: Card(
          child: Column(
            children: [
              Row(
                children: [
                  Text(
                    translate(title),
                    textAlign: TextAlign.start,
                    style: TextStyle(
                      fontSize: _kTitleFontSize,
                    ),
                  ),
                  Spacer(),
                ],
              ).marginOnly(left: _kContentHMargin, top: 10, bottom: 10),
              ...children
                  .map((e) => e.marginOnly(top: 4, right: _kContentHMargin)),
            ],
          ).marginOnly(bottom: 10),
        ).marginOnly(left: _kCardLeftMargin, top: 15),
      ),
    ],
  );
}

Widget _OptionCheckBox(String label, String key,
    {Function()? update = null, bool reverse = false}) {
  return _futureBuilder(
      future: bind.mainGetOption(key: key),
      hasData: (data) {
        bool value = option2bool(key, data.toString());
        if (reverse) value = !value;
        var ref = value.obs;
        var onChanged = (option) async {
          if (option != null) {
            ref.value = option;
            if (reverse) option = !option;
            String value = bool2option(key, option);
            bind.mainSetOption(key: key, value: value);
            update?.call();
          }
        };
        return GestureDetector(
          child: Obx(
            () => Row(
              children: [
                Checkbox(value: ref.value, onChanged: onChanged)
                    .marginOnly(right: 10),
                Expanded(child: Text(translate(label)))
              ],
            ),
          ).marginOnly(left: _kCheckBoxLeftMargin),
          onTap: () {
            onChanged(!ref.value);
          },
        );
      });
}

Widget _Radio<T>({
  required T value,
  required T groupValue,
  required String label,
  required Function(T value) onChanged,
}) {
  var on_change = (T? value) {
    if (value != null) {
      onChanged(value);
    }
  };
  return GestureDetector(
    child: Row(
      children: [
        Radio<T>(value: value, groupValue: groupValue, onChanged: on_change),
        Expanded(
          child: Text(translate(label),
                  style: TextStyle(fontSize: _kContentFontSize))
              .marginOnly(left: 5),
        ),
      ],
    ).marginOnly(left: _kRadioLeftMargin),
    onTap: () => on_change(value),
  );
}

Widget _Button(String label, Function() onPressed,
    {bool enabled = true, String? tip}) {
  var button = ElevatedButton(
      onPressed: enabled ? onPressed : null,
      child: Container(
        child: Text(
          translate(label),
        ).marginSymmetric(horizontal: 15),
      ));
  var child;
  if (tip == null) {
    child = button;
  } else {
    child = Tooltip(message: translate(tip), child: button);
  }
  return Row(children: [
    child,
  ]).marginOnly(left: _kContentHMargin);
}

Widget _SubButton(String label, Function() onPressed, [bool enabled = true]) {
  return Row(
    children: [
      ElevatedButton(
          onPressed: enabled ? onPressed : null,
          child: Container(
            child: Text(
              translate(label),
            ).marginSymmetric(horizontal: 15),
          )),
    ],
  ).marginOnly(left: _kContentHSubMargin);
}

Widget _SubLabeledWidget(String label, Widget child, {bool enabled = true}) {
  RxBool hover = false.obs;
  return Row(
    children: [
      MouseRegion(
          onEnter: (_) => hover.value = true,
          onExit: (_) => hover.value = false,
          child: Obx(
            () {
              return Container(
                  height: 32,
                  decoration: BoxDecoration(
                      border: Border.all(
                          color: hover.value && enabled
                              ? Colors.grey.withOpacity(0.8)
                              : Colors.grey.withOpacity(0.5),
                          width: hover.value && enabled ? 2 : 1)),
                  child: Row(
                    children: [
                      Container(
                        height: 28,
                        color: (hover.value && enabled)
                            ? Colors.grey.withOpacity(0.8)
                            : Colors.grey.withOpacity(0.5),
                        child: Text(
                          label + ': ',
                          style: TextStyle(),
                        ),
                        alignment: Alignment.center,
                        padding:
                            EdgeInsets.symmetric(horizontal: 5, vertical: 2),
                      ).paddingAll(2),
                      child,
                    ],
                  ));
            },
          )),
    ],
  ).marginOnly(left: _kContentHSubMargin);
}

Widget _futureBuilder(
    {required Future? future, required Widget Function(dynamic data) hasData}) {
  return FutureBuilder(
      future: future,
      builder: (BuildContext context, AsyncSnapshot snapshot) {
        if (snapshot.hasData) {
          return hasData(snapshot.data!);
        } else {
          if (snapshot.hasError) {
            print(snapshot.error.toString());
          }
          return Container();
        }
      });
}

// ignore: must_be_immutable
class _ComboBox extends StatelessWidget {
  late final List<String> keys;
  late final List<String> values;
  late final String initialKey;
  late final Function(String key) onChanged;
  late final bool enabled;
  late String current;

  _ComboBox({
    Key? key,
    required this.keys,
    required this.values,
    required this.initialKey,
    required this.onChanged,
    this.enabled = true,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var index = keys.indexOf(initialKey);
    if (index < 0) {
      assert(false);
      index = 0;
    }
    var ref = values[index].obs;
    current = keys[index];
    return Container(
      decoration: BoxDecoration(border: Border.all(color: MyTheme.border)),
      height: 30,
      child: Obx(() => DropdownButton<String>(
            isExpanded: true,
            value: ref.value,
            elevation: 16,
            underline: Container(
              height: 25,
            ),
            icon: Icon(
              Icons.expand_more_sharp,
              size: 20,
            ),
            onChanged: enabled
                ? (String? newValue) {
                    if (newValue != null && newValue != ref.value) {
                      ref.value = newValue;
                      current = newValue;
                      onChanged(keys[values.indexOf(newValue)]);
                    }
                  }
                : null,
            items: values.map<DropdownMenuItem<String>>((String value) {
              return DropdownMenuItem<String>(
                value: value,
                child: Text(
                  value,
                  style: TextStyle(fontSize: _kContentFontSize),
                  overflow: TextOverflow.ellipsis,
                ).marginOnly(left: 5),
              );
            }).toList(),
          )),
    );
  }
}

//#endregion

//#region dialogs

void changeServer() async {
  Map<String, dynamic> oldOptions = jsonDecode(await bind.mainGetOptions());
  print("${oldOptions}");
  String idServer = oldOptions['custom-rendezvous-server'] ?? "";
  var idServerMsg = "";
  String relayServer = oldOptions['relay-server'] ?? "";
  var relayServerMsg = "";
  String apiServer = oldOptions['api-server'] ?? "";
  var apiServerMsg = "";
  var key = oldOptions['key'] ?? "";

  var isInProgress = false;
  gFFI.dialogManager.show((setState, close) {
    return CustomAlertDialog(
      title: Text(translate("ID/Relay Server")),
      content: ConstrainedBox(
        constraints: BoxConstraints(minWidth: 500),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('ID Server')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      idServer = s;
                    },
                    decoration: InputDecoration(
                        border: OutlineInputBorder(),
                        errorText: idServerMsg.isNotEmpty ? idServerMsg : null),
                    controller: TextEditingController(text: idServer),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('Relay Server')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      relayServer = s;
                    },
                    decoration: InputDecoration(
                        border: OutlineInputBorder(),
                        errorText:
                            relayServerMsg.isNotEmpty ? relayServerMsg : null),
                    controller: TextEditingController(text: relayServer),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('API Server')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      apiServer = s;
                    },
                    decoration: InputDecoration(
                        border: OutlineInputBorder(),
                        errorText:
                            apiServerMsg.isNotEmpty ? apiServerMsg : null),
                    controller: TextEditingController(text: apiServer),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child:
                        Text("${translate('Key')}:").marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      key = s;
                    },
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(text: key),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 4.0,
            ),
            Offstage(offstage: !isInProgress, child: LinearProgressIndicator())
          ],
        ),
      ),
      actions: [
        TextButton(
            onPressed: () {
              close();
            },
            child: Text(translate("Cancel"))),
        TextButton(
            onPressed: () async {
              setState(() {
                [idServerMsg, relayServerMsg, apiServerMsg].forEach((element) {
                  element = "";
                });
                isInProgress = true;
              });
              final cancel = () {
                setState(() {
                  isInProgress = false;
                });
              };
              idServer = idServer.trim();
              relayServer = relayServer.trim();
              apiServer = apiServer.trim();
              key = key.trim();

              if (idServer.isNotEmpty) {
                idServerMsg = translate(
                    await bind.mainTestIfValidServer(server: idServer));
                if (idServerMsg.isEmpty) {
                  oldOptions['custom-rendezvous-server'] = idServer;
                } else {
                  cancel();
                  return;
                }
              } else {
                oldOptions['custom-rendezvous-server'] = "";
              }

              if (relayServer.isNotEmpty) {
                relayServerMsg = translate(
                    await bind.mainTestIfValidServer(server: relayServer));
                if (relayServerMsg.isEmpty) {
                  oldOptions['relay-server'] = relayServer;
                } else {
                  cancel();
                  return;
                }
              } else {
                oldOptions['relay-server'] = "";
              }

              if (apiServer.isNotEmpty) {
                if (apiServer.startsWith('http://') ||
                    apiServer.startsWith("https://")) {
                  oldOptions['api-server'] = apiServer;
                  return;
                } else {
                  apiServerMsg = translate("invalid_http");
                  cancel();
                  return;
                }
              } else {
                oldOptions['api-server'] = "";
              }
              // ok
              oldOptions['key'] = key;
              await bind.mainSetOptions(json: jsonEncode(oldOptions));
              close();
            },
            child: Text(translate("OK"))),
      ],
    );
  });
}

void changeWhiteList() async {
  Map<String, dynamic> oldOptions = jsonDecode(await bind.mainGetOptions());
  var newWhiteList = ((oldOptions['whitelist'] ?? "") as String).split(',');
  var newWhiteListField = newWhiteList.join('\n');
  var msg = "";
  var isInProgress = false;
  gFFI.dialogManager.show((setState, close) {
    return CustomAlertDialog(
      title: Text(translate("IP Whitelisting")),
      content: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(translate("whitelist_sep")),
          SizedBox(
            height: 8.0,
          ),
          Row(
            children: [
              Expanded(
                child: TextField(
                  onChanged: (s) {
                    newWhiteListField = s;
                  },
                  maxLines: null,
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    errorText: msg.isEmpty ? null : translate(msg),
                  ),
                  controller: TextEditingController(text: newWhiteListField),
                ),
              ),
            ],
          ),
          SizedBox(
            height: 4.0,
          ),
          Offstage(offstage: !isInProgress, child: LinearProgressIndicator())
        ],
      ),
      actions: [
        TextButton(
            onPressed: () {
              close();
            },
            child: Text(translate("Cancel"))),
        TextButton(
            onPressed: () async {
              setState(() {
                msg = "";
                isInProgress = true;
              });
              newWhiteListField = newWhiteListField.trim();
              var newWhiteList = "";
              if (newWhiteListField.isEmpty) {
                // pass
              } else {
                final ips =
                    newWhiteListField.trim().split(RegExp(r"[\s,;\n]+"));
                // test ip
                final ipMatch = RegExp(r"^\d+\.\d+\.\d+\.\d+$");
                for (final ip in ips) {
                  if (!ipMatch.hasMatch(ip)) {
                    msg = translate("Invalid IP") + " $ip";
                    setState(() {
                      isInProgress = false;
                    });
                    return;
                  }
                }
                newWhiteList = ips.join(',');
              }
              oldOptions['whitelist'] = newWhiteList;
              await bind.mainSetOptions(json: jsonEncode(oldOptions));
              close();
            },
            child: Text(translate("OK"))),
      ],
    );
  });
}

void changeSocks5Proxy() async {
  var socks = await bind.mainGetSocks();

  String proxy = "";
  String proxyMsg = "";
  String username = "";
  String password = "";
  if (socks.length == 3) {
    proxy = socks[0];
    username = socks[1];
    password = socks[2];
  }

  var isInProgress = false;
  gFFI.dialogManager.show((setState, close) {
    return CustomAlertDialog(
      title: Text(translate("Socks5 Proxy")),
      content: ConstrainedBox(
        constraints: BoxConstraints(minWidth: 500),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('Hostname')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      proxy = s;
                    },
                    decoration: InputDecoration(
                        border: OutlineInputBorder(),
                        errorText: proxyMsg.isNotEmpty ? proxyMsg : null),
                    controller: TextEditingController(text: proxy),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('Username')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      username = s;
                    },
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(text: username),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Row(
              children: [
                ConstrainedBox(
                    constraints: BoxConstraints(minWidth: 100),
                    child: Text("${translate('Password')}:")
                        .marginOnly(bottom: 16.0)),
                SizedBox(
                  width: 24.0,
                ),
                Expanded(
                  child: TextField(
                    onChanged: (s) {
                      password = s;
                    },
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                    controller: TextEditingController(text: password),
                  ),
                ),
              ],
            ),
            SizedBox(
              height: 8.0,
            ),
            Offstage(offstage: !isInProgress, child: LinearProgressIndicator())
          ],
        ),
      ),
      actions: [
        TextButton(
            onPressed: () {
              close();
            },
            child: Text(translate("Cancel"))),
        TextButton(
            onPressed: () async {
              setState(() {
                proxyMsg = "";
                isInProgress = true;
              });
              final cancel = () {
                setState(() {
                  isInProgress = false;
                });
              };
              proxy = proxy.trim();
              username = username.trim();
              password = password.trim();

              if (proxy.isNotEmpty) {
                proxyMsg =
                    translate(await bind.mainTestIfValidServer(server: proxy));
                if (proxyMsg.isEmpty) {
                  // ignore
                } else {
                  cancel();
                  return;
                }
              }
              await bind.mainSetSocks(
                  proxy: proxy, username: username, password: password);
              close();
            },
            child: Text(translate("OK"))),
      ],
    );
  });
}

//#endregion

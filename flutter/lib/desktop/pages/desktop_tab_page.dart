import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/desktop/pages/desktop_home_page.dart';
import 'package:flutter_hbb/desktop/pages/desktop_setting_page.dart';
import 'package:flutter_hbb/desktop/widgets/tabbar_widget.dart';
import 'package:window_manager/window_manager.dart';

class DesktopTabPage extends StatefulWidget {
  const DesktopTabPage({Key? key}) : super(key: key);

  @override
  State<DesktopTabPage> createState() => _DesktopTabPageState();
}

class _DesktopTabPageState extends State<DesktopTabPage> {
  final tabBarController = DesktopTabBarController();

  @override
  void initState() {
    super.initState();
    tabBarController.state.value.tabs.add(TabInfo(
        key: kTabLabelHomePage,
        label: kTabLabelHomePage,
        selectedIcon: Icons.home_sharp,
        unselectedIcon: Icons.home_outlined,
        closable: false,
        page: DesktopHomePage()));
  }

  @override
  Widget build(BuildContext context) {
    final dark = isDarkTheme();
    return DragToResizeArea(
      child: Container(
        decoration: BoxDecoration(
            border: Border.all(color: MyTheme.color(context).border!)),
        child: Scaffold(
            backgroundColor: MyTheme.color(context).bg,
            body: DesktopTab(
              controller: tabBarController,
              theme: dark ? TarBarTheme.dark() : TarBarTheme.light(),
              isMainWindow: true,
              tail: ActionIcon(
                message: 'Settings',
                icon: IconFont.menu,
                theme: dark ? TarBarTheme.dark() : TarBarTheme.light(),
                onTap: onAddSetting,
                is_close: false,
              ),
            )),
      ),
    );
  }

  void onAddSetting() {
    tabBarController.add(TabInfo(
        key: kTabLabelSettingPage,
        label: kTabLabelSettingPage,
        selectedIcon: Icons.build_sharp,
        unselectedIcon: Icons.build_outlined,
        page: DesktopSettingPage()));
  }
}

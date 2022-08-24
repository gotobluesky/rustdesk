import 'dart:math';

import 'package:desktop_multi_window/desktop_multi_window.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/main.dart';
import 'package:flutter_hbb/utils/multi_window_manager.dart';
import 'package:get/get.dart';
import 'package:window_manager/window_manager.dart';
import 'package:scroll_pos/scroll_pos.dart';

const double _kTabBarHeight = kDesktopRemoteTabBarHeight;
const double _kIconSize = 18;
const double _kDividerIndent = 10;
const double _kActionIconSize = 12;

class TabInfo {
  final String key;
  final String label;
  final IconData? selectedIcon;
  final IconData? unselectedIcon;
  final bool closable;
  final Widget page;

  TabInfo(
      {required this.key,
      required this.label,
      this.selectedIcon,
      this.unselectedIcon,
      this.closable = true,
      required this.page});
}

class DesktopTabBarState {
  final List<TabInfo> tabs = [];
  final ScrollPosController scrollController =
      ScrollPosController(itemCount: 0);
  final PageController pageController = PageController();
  int selected = 0;

  DesktopTabBarState() {
    scrollController.itemCount = tabs.length;
    // TODO test
    // WidgetsBinding.instance.addPostFrameCallback((_) {
    //   scrollController.scrollToItem(selected,
    //       center: true, animate: true);
    // });
  }
}

class DesktopTabBarController {
  final state = DesktopTabBarState().obs;

  void add(TabInfo tab) {
    if (!isDesktop) return;
    final index = state.value.tabs.indexWhere((e) => e.key == tab.key);
    int toIndex;
    if (index >= 0) {
      toIndex = index;
    } else {
      state.update((val) {
        val!.tabs.add(tab);
      });
      toIndex = state.value.tabs.length - 1;
      assert(toIndex >= 0);
    }
    try {
      jumpTo(toIndex);
    } catch (e) {
      // call before binding controller will throw
      debugPrint("Failed to jumpTo: $e");
    }
  }

  void remove(int index) {
    if (!isDesktop) return;
    if (index < 0) return;
    final len = state.value.tabs.length;
    final currentSelected = state.value.selected;
    int toIndex = 0;
    if (index == len - 1) {
      toIndex = max(0, currentSelected - 1);
    } else if (index < len - 1 && index < currentSelected) {
      toIndex = max(0, currentSelected - 1);
    }
    state.value.tabs.removeAt(index);
    state.value.scrollController.itemCount = state.value.tabs.length;
    jumpTo(toIndex);
  }

  void jumpTo(int index) {
    state.update((val) {
      val!.selected = index;
      val.pageController.jumpToPage(index);
      val.scrollController.scrollToItem(index, center: true, animate: true);
    });

    // onSelected callback
  }
}

class DesktopTab extends StatelessWidget {
  final Function(String)? onTabClose;
  final TarBarTheme theme;
  final bool isMainWindow;
  final bool showLogo;
  final bool showTitle;
  final bool showMinimize;
  final bool showMaximize;
  final bool showClose;
  final Widget Function(Widget pageView)? pageViewBuilder;
  final Widget? tail;

  final DesktopTabBarController controller;
  late final state = controller.state;

  DesktopTab(
      {required this.controller,
      required this.isMainWindow,
      this.theme = const TarBarTheme.light(),
      this.onTabClose,
      this.showLogo = true,
      this.showTitle = true,
      this.showMinimize = true,
      this.showMaximize = true,
      this.showClose = true,
      this.pageViewBuilder,
      this.tail});

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      Container(
        height: _kTabBarHeight,
        child: Column(
          children: [
            Container(
              height: _kTabBarHeight - 1,
              child: _buildBar(),
            ),
            Divider(
              height: 1,
              thickness: 1,
            ),
          ],
        ),
      ),
      Expanded(
          child: pageViewBuilder != null
              ? pageViewBuilder!(_buildPageView())
              : _buildPageView())
    ]);
  }

  Widget _buildPageView() {
    debugPrint("_buildPageView: ${state.value.tabs.length}");
    return Obx(() => PageView(
        controller: state.value.pageController,
        children:
            state.value.tabs.map((tab) => tab.page).toList(growable: false)));
  }

  Widget _buildBar() {
    return Row(
      children: [
        Expanded(
          child: Row(
            children: [
              Row(children: [
                Offstage(
                    offstage: !showLogo,
                    child: Image.asset(
                      'assets/logo.ico',
                      width: 20,
                      height: 20,
                    )),
                Offstage(
                    offstage: !showTitle,
                    child: Text(
                      "RustDesk",
                      style: TextStyle(fontSize: 13),
                    ).marginOnly(left: 2))
              ]).marginOnly(
                left: 5,
                right: 10,
              ),
              Expanded(
                child: GestureDetector(
                    onPanStart: (_) {
                      if (isMainWindow) {
                        windowManager.startDragging();
                      } else {
                        WindowController.fromWindowId(windowId!)
                            .startDragging();
                      }
                    },
                    child: _ListView(
                      controller: controller,
                      onTabClose: onTabClose,
                      theme: theme,
                    )),
              ),
              Offstage(
                offstage: isMainWindow,
                child: _AddButton(
                  theme: theme,
                ).paddingOnly(left: 10),
              ),
            ],
          ),
        ),
        Offstage(offstage: tail == null, child: tail),
        WindowActionPanel(
          mainTab: isMainWindow,
          theme: theme,
          showMinimize: showMinimize,
          showMaximize: showMaximize,
          showClose: showClose,
        )
      ],
    );
  }
}

class WindowActionPanel extends StatelessWidget {
  final bool mainTab;
  final TarBarTheme theme;

  final bool showMinimize;
  final bool showMaximize;
  final bool showClose;

  const WindowActionPanel(
      {Key? key,
      required this.mainTab,
      required this.theme,
      this.showMinimize = true,
      this.showMaximize = true,
      this.showClose = true})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Offstage(
            offstage: !showMinimize,
            child: ActionIcon(
              message: 'Minimize',
              icon: IconFont.min,
              theme: theme,
              onTap: () {
                if (mainTab) {
                  windowManager.minimize();
                } else {
                  WindowController.fromWindowId(windowId!).minimize();
                }
              },
              is_close: false,
            )),
        // TODO: drag makes window restore
        Offstage(
            offstage: !showMaximize,
            child: FutureBuilder(builder: (context, snapshot) {
              RxBool is_maximized = false.obs;
              if (mainTab) {
                windowManager.isMaximized().then((maximized) {
                  is_maximized.value = maximized;
                });
              } else {
                final wc = WindowController.fromWindowId(windowId!);
                wc.isMaximized().then((maximized) {
                  is_maximized.value = maximized;
                });
              }
              return Obx(
                () => ActionIcon(
                  message: is_maximized.value ? "Restore" : "Maximize",
                  icon: is_maximized.value ? IconFont.restore : IconFont.max,
                  theme: theme,
                  onTap: () {
                    if (mainTab) {
                      if (is_maximized.value) {
                        windowManager.unmaximize();
                      } else {
                        windowManager.maximize();
                      }
                    } else {
                      // TODO: subwindow is maximized but first query result is not maximized.
                      final wc = WindowController.fromWindowId(windowId!);
                      if (is_maximized.value) {
                        wc.unmaximize();
                      } else {
                        wc.maximize();
                      }
                    }
                    is_maximized.value = !is_maximized.value;
                  },
                  is_close: false,
                ),
              );
            })),
        Offstage(
            offstage: !showClose,
            child: ActionIcon(
              message: 'Close',
              icon: IconFont.close,
              theme: theme,
              onTap: () {
                if (mainTab) {
                  windowManager.close();
                } else {
                  WindowController.fromWindowId(windowId!).close();
                }
              },
              is_close: true,
            )),
      ],
    );
  }
}

// ignore: must_be_immutable
class _ListView extends StatelessWidget {
  final DesktopTabBarController controller;
  late final Rx<DesktopTabBarState> state;
  final Function(String key)? onTabClose;
  final TarBarTheme theme;

  _ListView(
      {required this.controller, required this.onTabClose, required this.theme})
      : this.state = controller.state;

  @override
  Widget build(BuildContext context) {
    return Obx(() => ListView(
        controller: state.value.scrollController,
        scrollDirection: Axis.horizontal,
        shrinkWrap: true,
        physics: BouncingScrollPhysics(),
        children: state.value.tabs.asMap().entries.map((e) {
          final index = e.key;
          final tab = e.value;
          return _Tab(
            index: index,
            label: tab.label,
            selectedIcon: tab.selectedIcon,
            unselectedIcon: tab.unselectedIcon,
            closable: tab.closable,
            selected: state.value.selected,
            onClose: () => controller.remove(index),
            onSelected: () => controller.jumpTo(index),
            theme: theme,
          );
        }).toList()));
  }
}

class _Tab extends StatelessWidget {
  late final int index;
  late final String label;
  late final IconData? selectedIcon;
  late final IconData? unselectedIcon;
  late final bool closable;
  late final int selected;
  late final Function() onClose;
  late final Function() onSelected;
  final RxBool _hover = false.obs;
  late final TarBarTheme theme;

  _Tab(
      {Key? key,
      required this.index,
      required this.label,
      this.selectedIcon,
      this.unselectedIcon,
      required this.closable,
      required this.selected,
      required this.onClose,
      required this.onSelected,
      required this.theme})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    bool show_icon = selectedIcon != null && unselectedIcon != null;
    bool is_selected = index == selected;
    bool show_divider = index != selected - 1 && index != selected;
    return Ink(
      child: InkWell(
        onHover: (hover) => _hover.value = hover,
        onTap: () => onSelected(),
        child: Row(
          children: [
            Container(
                height: _kTabBarHeight,
                child: Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Offstage(
                              offstage: !show_icon,
                              child: Icon(
                                is_selected ? selectedIcon : unselectedIcon,
                                size: _kIconSize,
                                color: is_selected
                                    ? theme.selectedtabIconColor
                                    : theme.unSelectedtabIconColor,
                              ).paddingOnly(right: 5)),
                          Text(
                            translate(label),
                            textAlign: TextAlign.center,
                            style: TextStyle(
                                color: is_selected
                                    ? theme.selectedTextColor
                                    : theme.unSelectedTextColor),
                          ),
                        ],
                      ),
                      Offstage(
                        offstage: !closable,
                        child: Obx((() => _CloseButton(
                              visiable: _hover.value,
                              tabSelected: is_selected,
                              onClose: () => onClose(),
                              theme: theme,
                            ))),
                      )
                    ])).paddingSymmetric(horizontal: 10),
            Offstage(
              offstage: !show_divider,
              child: VerticalDivider(
                width: 1,
                indent: _kDividerIndent,
                endIndent: _kDividerIndent,
                color: theme.dividerColor,
                thickness: 1,
              ),
            )
          ],
        ),
      ),
    );
  }
}

class _AddButton extends StatelessWidget {
  late final TarBarTheme theme;

  _AddButton({
    Key? key,
    required this.theme,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ActionIcon(
        message: 'New Connection',
        icon: IconFont.add,
        theme: theme,
        onTap: () =>
            rustDeskWinManager.call(WindowType.Main, "main_window_on_top", ""),
        is_close: false);
  }
}

class _CloseButton extends StatelessWidget {
  final bool visiable;
  final bool tabSelected;
  final Function onClose;
  late final TarBarTheme theme;

  _CloseButton({
    Key? key,
    required this.visiable,
    required this.tabSelected,
    required this.onClose,
    required this.theme,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
        width: _kIconSize,
        child: Offstage(
          offstage: !visiable,
          child: InkWell(
            customBorder: RoundedRectangleBorder(),
            onTap: () => onClose(),
            child: Icon(
              Icons.close,
              size: _kIconSize,
              color: tabSelected
                  ? theme.selectedIconColor
                  : theme.unSelectedIconColor,
            ),
          ),
        )).paddingOnly(left: 5);
  }
}

class ActionIcon extends StatelessWidget {
  final String message;
  final IconData icon;
  final TarBarTheme theme;
  final Function() onTap;
  final bool is_close;
  const ActionIcon({
    Key? key,
    required this.message,
    required this.icon,
    required this.theme,
    required this.onTap,
    required this.is_close,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    RxBool hover = false.obs;
    return Obx(() => Tooltip(
          message: translate(message),
          child: InkWell(
            hoverColor: is_close ? Colors.red : theme.hoverColor,
            onHover: (value) => hover.value = value,
            child: Container(
              height: _kTabBarHeight - 1,
              width: _kTabBarHeight - 1,
              child: Icon(
                icon,
                color: hover.value && is_close
                    ? Colors.white
                    : theme.unSelectedIconColor,
                size: _kActionIconSize,
              ),
            ),
            onTap: onTap,
          ),
        ));
  }
}

class TarBarTheme {
  final Color unSelectedtabIconColor;
  final Color selectedtabIconColor;
  final Color selectedTextColor;
  final Color unSelectedTextColor;
  final Color selectedIconColor;
  final Color unSelectedIconColor;
  final Color dividerColor;
  final Color hoverColor;

  const TarBarTheme.light()
      : unSelectedtabIconColor = const Color.fromARGB(255, 162, 203, 241),
        selectedtabIconColor = MyTheme.accent,
        selectedTextColor = const Color.fromARGB(255, 26, 26, 26),
        unSelectedTextColor = const Color.fromARGB(255, 96, 96, 96),
        selectedIconColor = const Color.fromARGB(255, 26, 26, 26),
        unSelectedIconColor = const Color.fromARGB(255, 96, 96, 96),
        dividerColor = const Color.fromARGB(255, 238, 238, 238),
        hoverColor = const Color.fromARGB(
            51, 158, 158, 158); // Colors.grey; //0xFF9E9E9E

  const TarBarTheme.dark()
      : unSelectedtabIconColor = const Color.fromARGB(255, 30, 65, 98),
        selectedtabIconColor = MyTheme.accent,
        selectedTextColor = const Color.fromARGB(255, 255, 255, 255),
        unSelectedTextColor = const Color.fromARGB(255, 207, 207, 207),
        selectedIconColor = const Color.fromARGB(255, 215, 215, 215),
        unSelectedIconColor = const Color.fromARGB(255, 255, 255, 255),
        dividerColor = const Color.fromARGB(255, 64, 64, 64),
        hoverColor = Colors.black26;
}

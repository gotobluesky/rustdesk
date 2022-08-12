import 'dart:io';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_hbb/mobile/pages/file_manager_page.dart';
import 'package:flutter_hbb/models/file_model.dart';
import 'package:get/get.dart';
import 'package:provider/provider.dart';
import 'package:wakelock/wakelock.dart';

import '../../common.dart';
import '../../models/model.dart';
import '../../models/platform_model.dart';

class FileManagerPage extends StatefulWidget {
  FileManagerPage({Key? key, required this.id}) : super(key: key);
  final String id;

  @override
  State<StatefulWidget> createState() => _FileManagerPageState();
}

class _FileManagerPageState extends State<FileManagerPage>
    with AutomaticKeepAliveClientMixin {
  final _localSelectedItems = SelectedItems();
  final _remoteSelectedItems = SelectedItems();

  late FFI _ffi;

  FileModel get model => _ffi.fileModel;

  SelectedItems getSelectedItem(bool isLocal) {
    return isLocal ? _localSelectedItems : _remoteSelectedItems;
  }

  @override
  void initState() {
    super.initState();
    _ffi = FFI();
    _ffi.connect(widget.id, isFileTransfer: true);
    Get.put(_ffi, tag: 'ft_${widget.id}');
    // _ffi.ffiModel.updateEventListener(widget.id);
    if (!Platform.isLinux) {
      Wakelock.enable();
    }
    print("init success with id ${widget.id}");
  }

  @override
  void dispose() {
    model.onClose();
    _ffi.close();
    _ffi.dialogManager.dismissAll();
    if (!Platform.isLinux) {
      Wakelock.disable();
    }
    Get.delete<FFI>(tag: 'ft_${widget.id}');
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return ChangeNotifierProvider.value(
        value: _ffi.fileModel,
        child: Consumer<FileModel>(builder: (_context, _model, _child) {
          return WillPopScope(
              onWillPop: () async {
                if (model.selectMode) {
                  model.toggleSelectMode();
                }
                return false;
              },
              child: Scaffold(
                body: Row(
                  children: [
                    Flexible(flex: 3, child: body(isLocal: true)),
                    Flexible(flex: 3, child: body(isLocal: false)),
                    Flexible(flex: 2, child: statusList())
                  ],
                ),
              ));
        }));
  }

  Widget menu({bool isLocal = false}) {
    return PopupMenuButton<String>(
        icon: Icon(Icons.more_vert),
        itemBuilder: (context) {
          return [
            PopupMenuItem(
              child: Row(
                children: [
                  Icon(
                      model.getCurrentShowHidden(isLocal)
                          ? Icons.check_box_outlined
                          : Icons.check_box_outline_blank,
                      color: Colors.black),
                  SizedBox(width: 5),
                  Text(translate("Show Hidden Files"))
                ],
              ),
              value: "hidden",
            )
          ];
        },
        onSelected: (v) {
          if (v == "hidden") {
            model.toggleShowHidden(local: isLocal);
          }
        });
  }

  Widget body({bool isLocal = false}) {
    final fd = isLocal ? model.currentLocalDir : model.currentRemoteDir;
    final entries = fd.entries;
    final sortIndex = (SortBy style) {
      switch (style) {
        case SortBy.Name:
          return 1;
        case SortBy.Type:
          return 0;
        case SortBy.Modified:
          return 2;
        case SortBy.Size:
          return 3;
      }
    }(model.getSortStyle(isLocal));
    final sortAscending =
        isLocal ? model.localSortAscending : model.remoteSortAscending;
    return Container(
      decoration: BoxDecoration(
          color: Colors.white54, border: Border.all(color: Colors.black26)),
      margin: const EdgeInsets.all(16.0),
      padding: const EdgeInsets.all(8.0),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        headTools(isLocal),
        Expanded(
            child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: SingleChildScrollView(
                child: DataTable(
                  showCheckboxColumn: true,
                  dataRowHeight: 25,
                  headingRowHeight: 30,
                  columnSpacing: 8,
                  showBottomBorder: true,
                  sortColumnIndex: sortIndex,
                  sortAscending: sortAscending,
                  columns: [
                    DataColumn(label: Text(translate(" "))), // icon
                    DataColumn(
                        label: Text(
                          translate("Name"),
                        ),
                        onSort: (columnIndex, ascending) {
                          model.changeSortStyle(SortBy.Name,
                              isLocal: isLocal, ascending: ascending);
                        }),
                    DataColumn(
                        label: Text(
                          translate("Modified"),
                        ),
                        onSort: (columnIndex, ascending) {
                          model.changeSortStyle(SortBy.Modified,
                              isLocal: isLocal, ascending: ascending);
                        }),
                    DataColumn(
                        label: Text(translate("Size")),
                        onSort: (columnIndex, ascending) {
                          model.changeSortStyle(SortBy.Size,
                              isLocal: isLocal, ascending: ascending);
                        }),
                  ],
                  rows: entries.map((entry) {
                    final sizeStr = entry.isFile
                        ? readableFileSize(entry.size.toDouble())
                        : "";
                    return DataRow(
                        key: ValueKey(entry.name),
                        onSelectChanged: (s) {
                          if (s != null) {
                            if (s) {
                              getSelectedItem(isLocal).add(isLocal, entry);
                            } else {
                              getSelectedItem(isLocal).remove(entry);
                            }
                            setState(() {});
                          }
                        },
                        selected: getSelectedItem(isLocal).contains(entry),
                        cells: [
                          DataCell(Icon(
                              entry.isFile ? Icons.feed_outlined : Icons.folder,
                              size: 25)),
                          DataCell(
                              ConstrainedBox(
                                  constraints: BoxConstraints(maxWidth: 100),
                                  child: Tooltip(
                                    message: entry.name,
                                    child: Text(entry.name,
                                        overflow: TextOverflow.ellipsis),
                                  )), onTap: () {
                            if (entry.isDirectory) {
                              model.openDirectory(entry.path, isLocal: isLocal);
                              if (isLocal) {
                                _localSelectedItems.clear();
                              } else {
                                _remoteSelectedItems.clear();
                              }
                            } else {
                              // Perform file-related tasks.
                              final _selectedItems = getSelectedItem(isLocal);
                              if (_selectedItems.contains(entry)) {
                                _selectedItems.remove(entry);
                              } else {
                                _selectedItems.add(isLocal, entry);
                              }
                              setState(() {});
                            }
                          }),
                          DataCell(Text(
                            entry
                                    .lastModified()
                                    .toString()
                                    .replaceAll(".000", "") +
                                "   ",
                            style: TextStyle(
                                fontSize: 12, color: MyTheme.darkGray),
                          )),
                          DataCell(Text(
                            sizeStr,
                            style: TextStyle(
                                fontSize: 12, color: MyTheme.darkGray),
                          )),
                        ]);
                  }).toList(),
                ),
              ),
            )
          ],
        )),
        // Center(child: listTail(isLocal: isLocal)),
        // Expanded(
        //     child: ListView.builder(
        //   itemCount: entries.length + 1,
        //   itemBuilder: (context, index) {
        //     if (index >= entries.length) {
        //       return listTail(isLocal: isLocal);
        //     }
        //     var selected = false;
        //     if (model.selectMode) {
        //       selected = _selectedItems.contains(entries[index]);
        //     }
        //
        //     final sizeStr = entries[index].isFile
        //         ? readableFileSize(entries[index].size.toDouble())
        //         : "";
        //     return Card(
        //       child: ListTile(
        //         leading: Icon(
        //             entries[index].isFile ? Icons.feed_outlined : Icons.folder,
        //             size: 40),
        //         title: Text(entries[index].name),
        //         selected: selected,
        //         subtitle: Text(
        //           entries[index]
        //                   .lastModified()
        //                   .toString()
        //                   .replaceAll(".000", "") +
        //               "   " +
        //               sizeStr,
        //           style: TextStyle(fontSize: 12, color: MyTheme.darkGray),
        //         ),
        //         trailing: needShowCheckBox()
        //             ? Checkbox(
        //                 value: selected,
        //                 onChanged: (v) {
        //                   if (v == null) return;
        //                   if (v && !selected) {
        //                     _selectedItems.add(isLocal, entries[index]);
        //                   } else if (!v && selected) {
        //                     _selectedItems.remove(entries[index]);
        //                   }
        //                   setState(() {});
        //                 })
        //             : PopupMenuButton<String>(
        //                 icon: Icon(Icons.more_vert),
        //                 itemBuilder: (context) {
        //                   return [
        //                     PopupMenuItem(
        //                       child: Text(translate("Delete")),
        //                       value: "delete",
        //                     ),
        //                     PopupMenuItem(
        //                       child: Text(translate("Multi Select")),
        //                       value: "multi_select",
        //                     ),
        //                     PopupMenuItem(
        //                       child: Text(translate("Properties")),
        //                       value: "properties",
        //                       enabled: false,
        //                     )
        //                   ];
        //                 },
        //                 onSelected: (v) {
        //                   if (v == "delete") {
        //                     final items = SelectedItems();
        //                     items.add(isLocal, entries[index]);
        //                     model.removeAction(items);
        //                   } else if (v == "multi_select") {
        //                     _selectedItems.clear();
        //                     model.toggleSelectMode();
        //                   }
        //                 }),
        //         onTap: () {
        //           if (model.selectMode && !_selectedItems.isOtherPage(isLocal)) {
        //             if (selected) {
        //               _selectedItems.remove(entries[index]);
        //             } else {
        //               _selectedItems.add(isLocal, entries[index]);
        //             }
        //             setState(() {});
        //             return;
        //           }
        //           if (entries[index].isDirectory) {
        //             model.openDirectory(entries[index].path, isLocal: isLocal);
        //             breadCrumbScrollToEnd(isLocal);
        //           } else {
        //             // Perform file-related tasks.
        //           }
        //         },
        //         onLongPress: () {
        //           _selectedItems.clear();
        //           model.toggleSelectMode();
        //           if (model.selectMode) {
        //             _selectedItems.add(isLocal, entries[index]);
        //           }
        //           setState(() {});
        //         },
        //       ),
        //     );
        //   },
        // ))
      ]),
    );
  }

  /// transfer status list
  /// watch transfer status
  Widget statusList() {
    return PreferredSize(
        child: Container(
          margin: const EdgeInsets.only(top: 16.0, bottom: 16.0, right: 16.0),
          padding: const EdgeInsets.all(8.0),
          decoration: BoxDecoration(
              color: Colors.white70, border: Border.all(color: Colors.grey)),
          child: Obx(
            () => ListView.builder(
              itemBuilder: (BuildContext context, int index) {
                final item = model.jobTable[index];
                return Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Row(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        Transform.rotate(
                            angle: item.isRemote ? pi : 0,
                            child: Icon(Icons.send)),
                        SizedBox(
                          width: 16.0,
                        ),
                        Expanded(
                          child: Column(
                            mainAxisSize: MainAxisSize.min,
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Tooltip(
                                  message: item.jobName,
                                  child: Text(
                                    '${item.jobName}',
                                    maxLines: 1,
                                    style: TextStyle(color: Colors.black45),
                                    overflow: TextOverflow.ellipsis,
                                  )),
                              Wrap(
                                children: [
                                  Text(
                                      '${item.state.display()} ${max(0, item.fileNum)}/${item.fileCount} '),
                                  Text(
                                      '${translate("files")} ${readableFileSize(item.totalSize.toDouble())} '),
                                  Offstage(
                                      offstage:
                                          item.state != JobState.inProgress,
                                      child: Text(
                                          '${readableFileSize(item.speed) + "/s"} ')),
                                  Offstage(
                                    offstage: item.totalSize <= 0,
                                    child: Text(
                                        '${(item.finishedSize.toDouble() * 100 / item.totalSize.toDouble()).toStringAsFixed(2)}%'),
                                  ),
                                ],
                              ),
                            ],
                          ),
                        ),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.end,
                          children: [
                            Offstage(
                              offstage: item.state != JobState.paused,
                              child: IconButton(
                                  onPressed: () {
                                    model.resumeJob(item.id);
                                  },
                                  icon: Icon(Icons.restart_alt_rounded)),
                            ),
                            IconButton(
                              icon: Icon(Icons.delete),
                              onPressed: () {
                                model.jobTable.removeAt(index);
                                model.cancelJob(item.id);
                              },
                            ),
                          ],
                        )
                      ],
                    ),
                    SizedBox(
                      height: 8.0,
                    ),
                    Divider(
                      height: 2.0,
                    )
                  ],
                );
              },
              itemCount: model.jobTable.length,
            ),
          ),
        ),
        preferredSize: Size(200, double.infinity));
  }

  goBack({bool? isLocal}) {
    model.goToParentDirectory(isLocal: isLocal);
  }

  Widget headTools(bool isLocal) => Container(
          child: Column(
        children: [
          // symbols
          PreferredSize(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Container(
                      width: 50,
                      height: 50,
                      decoration: BoxDecoration(color: Colors.blue),
                      padding: EdgeInsets.all(8.0),
                      child: FutureBuilder<String>(
                          future: bind.sessionGetPlatform(
                              id: _ffi.id, isRemote: !isLocal),
                          builder: (context, snapshot) {
                            if (snapshot.hasData && snapshot.data!.isNotEmpty) {
                              return getPlatformImage('${snapshot.data}');
                            } else {
                              return CircularProgressIndicator(
                                color: Colors.white,
                              );
                            }
                          })),
                  Text(isLocal
                          ? translate("Local Computer")
                          : translate("Remote Computer"))
                      .marginOnly(left: 8.0)
                ],
              ),
              preferredSize: Size(double.infinity, 70)),
          // buttons
          Row(
            children: [
              Row(
                children: [
                  IconButton(
                      onPressed: () {
                        model.goHome(isLocal: isLocal);
                      },
                      icon: Icon(Icons.home_outlined)),
                  IconButton(
                    icon: Icon(Icons.arrow_upward),
                    onPressed: () {
                      goBack(isLocal: isLocal);
                    },
                  ),
                  menu(isLocal: isLocal),
                ],
              ),
              Expanded(
                  child: Container(
                      decoration: BoxDecoration(
                          border: Border.all(color: Colors.black12)),
                      child: TextField(
                        decoration: InputDecoration(
                            border: InputBorder.none,
                            isDense: true,
                            prefix:
                                Padding(padding: EdgeInsets.only(left: 4.0)),
                            suffix: DropdownButton<String>(
                                isDense: true,
                                underline: Offstage(),
                                items: [
                                  // TODO: favourite
                                  DropdownMenuItem(
                                    child: Text('/'),
                                    value: '/',
                                  )
                                ],
                                onChanged: (path) {
                                  if (path is String && path.isNotEmpty) {
                                    model.openDirectory(path, isLocal: isLocal);
                                  }
                                })),
                        controller: TextEditingController(
                            text: isLocal
                                ? model.currentLocalDir.path
                                : model.currentRemoteDir.path),
                        onSubmitted: (path) {
                          model.openDirectory(path, isLocal: isLocal);
                        },
                      ))),
              IconButton(
                  onPressed: () {
                    model.refresh(isLocal: isLocal);
                  },
                  icon: Icon(Icons.refresh))
            ],
          ),
          Row(
            textDirection: isLocal ? TextDirection.ltr : TextDirection.rtl,
            children: [
              Expanded(
                child: Row(
                  mainAxisAlignment:
                      isLocal ? MainAxisAlignment.start : MainAxisAlignment.end,
                  children: [
                    IconButton(
                        onPressed: () {
                          final name = TextEditingController();
                          _ffi.dialogManager.show((setState, close) =>
                              CustomAlertDialog(
                                  title: Text(translate("Create Folder")),
                                  content: Column(
                                    mainAxisSize: MainAxisSize.min,
                                    children: [
                                      TextFormField(
                                        decoration: InputDecoration(
                                          labelText: translate(
                                              "Please enter the folder name"),
                                        ),
                                        controller: name,
                                      ),
                                    ],
                                  ),
                                  actions: [
                                    TextButton(
                                        style: flatButtonStyle,
                                        onPressed: () => close(false),
                                        child: Text(translate("Cancel"))),
                                    ElevatedButton(
                                        style: flatButtonStyle,
                                        onPressed: () {
                                          if (name.value.text.isNotEmpty) {
                                            model.createDir(
                                                PathUtil.join(
                                                    model
                                                        .getCurrentDir(isLocal)
                                                        .path,
                                                    name.value.text,
                                                    model.getCurrentIsWindows(
                                                        isLocal)),
                                                isLocal: isLocal);
                                            close();
                                          }
                                        },
                                        child: Text(translate("OK")))
                                  ]));
                        },
                        icon: Icon(Icons.create_new_folder_outlined)),
                    IconButton(
                        onPressed: () async {
                          final items = isLocal
                              ? _localSelectedItems
                              : _remoteSelectedItems;
                          await (model.removeAction(items, isLocal: isLocal));
                          items.clear();
                        },
                        icon: Icon(Icons.delete_forever_outlined)),
                  ],
                ),
              ),
              TextButton.icon(
                  onPressed: () {
                    final items = getSelectedItem(isLocal);
                    model.sendFiles(items, isRemote: !isLocal);
                    items.clear();
                  },
                  icon: Transform.rotate(
                    angle: isLocal ? 0 : pi,
                    child: Icon(
                      Icons.send,
                      color: Colors.black54,
                    ),
                  ),
                  label: Text(
                    isLocal ? translate('Send') : translate('Receive'),
                    style: TextStyle(
                      color: Colors.black54,
                    ),
                  )),
            ],
          ).marginOnly(top: 8.0)
        ],
      ));

  Widget listTail({bool isLocal = false}) {
    final dir = isLocal ? model.currentLocalDir : model.currentRemoteDir;
    return Container(
      height: 100,
      child: Column(
        children: [
          Padding(
            padding: EdgeInsets.fromLTRB(30, 5, 30, 0),
            child: Text(
              dir.path,
              style: TextStyle(color: MyTheme.darkGray),
            ),
          ),
          Padding(
            padding: EdgeInsets.all(2),
            child: Text(
              "${translate("Total")}: ${dir.entries.length} ${translate("items")}",
              style: TextStyle(color: MyTheme.darkGray),
            ),
          )
        ],
      ),
    );
  }

  @override
  bool get wantKeepAlive => true;

  /// Get the image for the current [platform].
  Widget getPlatformImage(String platform) {
    platform = platform.toLowerCase();
    if (platform == 'mac os')
      platform = 'mac';
    else if (platform != 'linux' && platform != 'android') platform = 'win';
    return Image.asset('assets/$platform.png', width: 25, height: 25);
  }
}

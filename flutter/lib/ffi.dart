import 'package:avelio_sdk/lib.dart';
import 'package:flutter/material.dart';

class FFIScope extends InheritedWidget {
  final NativeRust rust;

  @override
  final child;

  const FFIScope({Key? key, required this.rust, required this.child})
      : super(key: key, child: child);

  static FFIScope? of(BuildContext context) {
    return (context.dependOnInheritedWidgetOfExactType<FFIScope>());
  }

  @override
  bool updateShouldNotify(FFIScope oldWidget) {
    //return true;
    return false;
  }
}

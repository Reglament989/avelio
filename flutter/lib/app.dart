import 'package:avelio/ffi.dart';
import 'package:avelio_sdk/lib.dart';
import 'package:flutter/material.dart';

import 'home/home.dart';

/// {@template home_app}
/// A [MaterialApp] which sets the `home` to [homePage].
/// {@endtemplate}
class HomeApp extends MaterialApp {
  /// {@macro home_app}
  HomeApp({Key? key})
      : super(
            key: key,
            home: FFIScope(
              child: const HomePage(),
              rust: NativeRust(),
            ),
            theme: ThemeData.light().copyWith(
                appBarTheme: const AppBarTheme(
                    elevation: 0,
                    color: Colors.transparent,
                    titleTextStyle: TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.bold,
                        color: Colors.black))));
}

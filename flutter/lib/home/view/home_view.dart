import 'package:avelio/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../home.dart';

/// {@template home_view}
/// A [StatelessWidget] which reacts to the provided
/// [homeCubit] state and notifies it in response to user input.
/// {@endtemplate}
class HomeView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final rust = FFIScope.of(context)!.rust;
    return Scaffold(
      appBar: AppBar(title: Text(rust.tr("name-of-the-company"))),
      bottomNavigationBar: BottomNavigationBar(
        items: List.generate(
            3,
            (index) => BottomNavigationBarItem(
                icon: Icon(Icons.home), label: '$index')),
      ),
      body: Column(children: [
        Expanded(flex: 2, child: TopBlock()),
        Expanded(flex: 6, child: TopBlock()),
      ]),
    );
  }
}

class TopBlock extends StatelessWidget {
  const TopBlock({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: Colors.red,
      child: Center(
        child: Text("WHERE"),
      ),
    );
  }
}

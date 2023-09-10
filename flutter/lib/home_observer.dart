import 'package:flutter_bloc/flutter_bloc.dart';

/// {@template Home_observer}
/// [BlocObserver] for the Home application which
/// observes all state changes.
/// {@endtemplate}
class HomeObserver extends BlocObserver {
  @override
  void onChange(BlocBase bloc, Change change) {
    super.onChange(bloc, change);
    print('${bloc.runtimeType} $change');
  }
}

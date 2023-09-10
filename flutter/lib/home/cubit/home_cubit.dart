import 'package:bloc/bloc.dart';

/// {@template home_cubit}
/// A [Cubit] which manages an [int] as its state.
/// {@endtemplate}
class HomeCubit extends Cubit<int> {
  /// {@macro home_cubit}
  HomeCubit() : super(0);

  /// Add 1 to the current state.
  void increment() => emit(state + 1);

  /// Subtract 1 from the current state.
  void decrement() => emit(state - 1);
}

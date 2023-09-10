import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

import 'package:avelio_sdk/gen/proto/general.pb.dart';
import 'package:avelio_sdk/gen/proto/track.pb.dart';
import 'package:avelio_sdk/gen/proto/auth.pb.dart';
import 'package:avelio_sdk/gen/ffi.dart' as native;

class NativeRust {
  late final native.NativeLibrary dll;
  NativeRust() {
    dll = native.NativeLibrary(DynamicLibrary.open("./libavelio_sdk.so"));
    using((arena) {
      dll.init("http://localhost:56750".toNativeUtf8().cast(),
          "".toNativeUtf8().cast());
    });
  }

  AuthorizateResponse _parseSign(Pointer<Int8> response) {
    final list = base64Decode(response.cast<Utf8>().toDartString());
    final tokens = AuthorizateResponse.fromBuffer(list);
    dll.free_char(response);
    return tokens;
  }

  AuthorizateResponse signIn(String login, String password) {
    return using((arena) {
      final loginPtr = login.toNativeUtf8();
      final passwordPtr = password.toNativeUtf8();
      final response = dll.sign_in(loginPtr.cast(), passwordPtr.cast());
      return _parseSign(response);
    });
  }

  AuthorizateResponse refreshToken(String token) {
    return using((arena) {
      final tokenPtr = token.toNativeUtf8();
      final response = dll.refresh_token(tokenPtr.cast());
      return _parseSign(response);
    });
  }

  AuthorizateResponse signUp(String login, String password) {
    return using((arena) {
      final loginPtr = login.toNativeUtf8();
      final passwordPtr = password.toNativeUtf8();
      final response = dll.sign_up(loginPtr.cast(), passwordPtr.cast());
      return _parseSign(response);
    });
  }

  List<Song> tracks([int? limit, int? offset]) {
    return using((arena) {
      final response = dll.tracks(limit ?? 100, offset ?? 0);
      final list = base64Decode(response.cast<Utf8>().toDartString());
      final repo = SongRepository.fromBuffer(list);
      dll.free_char(response);
      return repo.songs;
    });
  }

  String tr(String key) {
    return using((arena) {
      final keyPtr = key.toNativeUtf8();
      final response = dll.tr(keyPtr.cast(), 0, Pointer.fromAddress(0));
      final dartResponse = response.cast<Utf8>().toDartString();
      arena.free(response);
      return dartResponse;
    });
  }
}

void main(List<String> args) async {
  final lib = NativeRust();
  final tokens = lib.signIn("test", "test");
  print(lib.tracks());
  print(lib.refreshToken(tokens.refreshToken));
  // lib.playOnce("/home/h/Music/ОРГАНИЗАЦИЯ.mp3p");
}

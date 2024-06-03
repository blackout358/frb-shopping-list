import 'package:flutter/material.dart';
import 'package:shopping_list_frontend/Screens/main_page.dart';
import 'package:shopping_list_frontend/src/rust/api/communication.dart';
import 'package:shopping_list_frontend/src/rust/api/item_model.dart';
import 'package:shopping_list_frontend/src/rust/api/simple.dart';
import 'package:shopping_list_frontend/src/rust/frb_generated.dart';
// import

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
            centerTitle: true,
            title: const Text(
              'Shopping List',
            )),
        body: const MainPage(),
      ),
    );
  }
}

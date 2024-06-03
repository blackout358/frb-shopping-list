import 'package:flutter/material.dart';
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
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: FutureBuilder<List<Item>>(
              future: getItems(),
              builder: (context, snapshot) {
                // Check if the future has completed
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return CircularProgressIndicator(); // Show loading indicator
                } else if (snapshot.hasError) {
                  return Text('Error: ${snapshot.error}'); // Show error message
                } else {
                  // Show data from the future
                  return Text('Data: ${snapshot.data}');
                }
              }),
        ),
      ),
    );
  }
}

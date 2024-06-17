import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:shopping_list_frontend/Widgets/item_card.dart';
import 'package:shopping_list_frontend/Widgets/refresh_floating_button.dart';
import 'package:shopping_list_frontend/src/rust/api/communication.dart';
import 'package:shopping_list_frontend/src/rust/api/item_model.dart';
import 'package:shopping_list_frontend/Widgets/alert_dialog.dart';

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  void _reloadPage() {
    setState(() {});
  }

  @override
  void initState() {
    // List<Item>> items = getItems();
  }
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      floatingActionButtonLocation: FloatingActionButtonLocation.centerFloat,
      floatingActionButton: FloatingRefreshButton(
        onPressedRefresh: () {
          setState(() {});
        },
        onPressedAdd: () {
          MyAlertDialog.addNewItem(context, _reloadPage);
          // setState(() {});
        },
      ),
      body: Column(
        children: [
          FutureBuilder<List<Item>>(
              future: getItems(),
              builder: (context, snapshot) {
                // Check if the future has completed
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const CircularProgressIndicator(); // Show loading indicator
                } else if (snapshot.hasError) {
                  return Text('Error: ${snapshot.error}'); // Show error message
                } else {
                  return Expanded(
                    child: ListView.builder(
                      itemCount: snapshot.data!.length,
                      itemBuilder: (context, index) {
                        var item = snapshot.data![index];
                        return ItemCard(
                          item: item,
                          onPressed: () {
                            deleteItem(id: item.id.oid);
                            // snapshot.data!.removeAt(index);
                            // setState(() {});
                          },
                          deletePressed: () {
                            deleteItem(id: item.id.oid);
                            _reloadPage();
                          },
                          updatePressed: () {
                            MyAlertDialog.showUpdateDialog(
                                context, item.id.oid, _reloadPage);
                          },
                          // deletePressed: () {},
                        );
                      },
                    ),
                  );
                }
              }),
        ],
      ),
    );
  }
}

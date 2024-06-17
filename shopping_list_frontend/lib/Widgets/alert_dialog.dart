import 'package:flutter/material.dart';
import 'package:shopping_list_frontend/Widgets/text_field.dart';
import 'package:shopping_list_frontend/src/rust/api/communication.dart';

class MyAlertDialog extends StatelessWidget {
  final String dialogTitle;
  final String dialogText;
  final VoidCallback onPressed;
  final TextEditingController textController;
  const MyAlertDialog(
      {super.key,
      required this.dialogText,
      required this.onPressed,
      required this.dialogTitle,
      required this.textController});

  @override
  Widget build(BuildContext context) {
    // TextEditingController nameController = TextEditingController();
    Widget cancelButton = TextButton(
      onPressed: () {
        Navigator.of(context).pop();
      },
      child: const Text("Cancel"),
    );

    Widget confirmButton = TextButton(
      onPressed: () {
        Navigator.of(context).pop();
        onPressed();
      },
      child: const Text("Confirm"),
    );

    return AlertDialog(
      title: Text(dialogTitle),
      content: MyTextField(
        controller: textController,
        hintText: 'Item Name',
        obscureText: false,
      ),
      actions: [cancelButton, confirmButton],
    );
  }

  static Future<void> addNewItem(BuildContext context, VoidCallback onPressed) {
    TextEditingController textCtrl = TextEditingController();
    return showDialog<void>(
        context: context,
        barrierDismissible: true,
        builder: (BuildContext context) {
          return MyAlertDialog(
            dialogText: "Enter Item Name",
            onPressed: () {
              addItem(name: textCtrl.text);
              onPressed();
            },
            dialogTitle: "Add Item",
            textController: textCtrl,
          );
        });
  }

  static Future<void> showUpdateDialog(
      BuildContext context, String id, VoidCallback onPressed) {
    TextEditingController textCtrl = TextEditingController();

    return showDialog<void>(
        context: context,
        barrierDismissible: true,
        builder: (BuildContext context) {
          return MyAlertDialog(
            dialogText: "Enter New Name",
            onPressed: () {
              updateItem(id: id, newName: textCtrl.text);
              onPressed();
            },
            dialogTitle: "Update Item Name",
            textController: textCtrl,
          );
        });
  }
}

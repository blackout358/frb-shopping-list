import 'package:flutter/material.dart';
import 'package:shopping_list_frontend/Widgets/text_field.dart';
import 'package:shopping_list_frontend/src/rust/api/communication.dart';

class MyAlertDialog extends StatelessWidget {
  final String dialogTitle;
  final String dialogText;
  final VoidCallback onPressed;
  const MyAlertDialog(
      {super.key,
      required this.dialogText,
      required this.onPressed,
      required this.dialogTitle});

  @override
  Widget build(BuildContext context) {
    TextEditingController nameController = TextEditingController();
    Widget cancelButton = TextButton(
      onPressed: () {
        Navigator.of(context).pop();
      },
      child: const Text("Cancel"),
    );

    Widget confirmButton = TextButton(
      onPressed: () {
        addItem(name: nameController.text);
        Navigator.of(context).pop();
        onPressed();
      },
      child: const Text("Confirm"),
    );

    return AlertDialog(
      title: Text(dialogTitle),
      content: MyTextField(
        controller: nameController,
        hintText: 'Item Name',
        obscureText: false,
      ),
      actions: [cancelButton, confirmButton],
    );
  }

  static Future<void> showMyDialog(
      BuildContext context, VoidCallback onPressed) {
    // final VoidCallback onPressed;
    return showDialog<void>(
        context: context,
        barrierDismissible: true,
        builder: (BuildContext context) {
          return MyAlertDialog(
              dialogText: "Enter Item Name",
              onPressed: onPressed,
              dialogTitle: "Add Item");
        });
  }
}

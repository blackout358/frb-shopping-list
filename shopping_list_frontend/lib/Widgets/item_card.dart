import 'package:flutter/material.dart';
import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:shopping_list_frontend/src/rust/api/item_model.dart';

class ItemCard extends StatelessWidget {
  final Item item;
  final VoidCallback onPressed;
  final VoidCallback updatePressed;
  final VoidCallback deletePressed;
  // final VoidCallbackAction deletePressed;
  // final VoidCallback(BuildContext) deletePressed;
  const ItemCard(
      {super.key,
      required this.item,
      required this.onPressed,
      required this.deletePressed,
      required this.updatePressed});

  @override
  Widget build(BuildContext context) {
    return Card(
      clipBehavior: Clip.antiAlias,
      elevation: 2.0,
      margin: const EdgeInsets.symmetric(vertical: 08.0, horizontal: 16.0),
      // shape: Rec,
      child: Slidable(
        startActionPane: ActionPane(
          extentRatio: 0.4,
          // A motion is a widget used to control how the pane animates.
          motion: DrawerMotion(),

          // A pane can dismiss the Slidable.
          // dismissible: DismissiblePane(onDismissed: () {}),

          // All actions are defined in the children parameter.
          children: [
            SlidableAction(
              onPressed: (BuildContext context) {
                updatePressed();
              },
              backgroundColor: Color(0xFF21B7CA),
              foregroundColor: Colors.white,
              icon: Icons.edit,
              label: 'Edit',
            ),
          ],
        ),
        endActionPane: ActionPane(
          extentRatio: 0.4,
          // A motion is a widget used to control how the pane animates.
          motion: DrawerMotion(),

          // A pane can dismiss the Slidable.
          // dismissible: DismissiblePane(onDismissed: () {}),

          // All actions are defined in the children parameter.
          children: [
            // A SlidableAction can have an icon and/or a label.
            SlidableAction(
              onPressed: (BuildContext context) {
                deletePressed();
              },
              backgroundColor: Color(0xFFFE4A49),
              foregroundColor: Colors.white,
              icon: Icons.delete,
              label: 'Delete',
            ),
          ],
        ),
        child: ListTile(
          contentPadding: const EdgeInsets.all(8.0),
          title: Text(item.name,
              style: const TextStyle(fontWeight: FontWeight.bold)),
          subtitle: Text(item.id.oid),
          trailing: IconButton(
            icon: const Icon(Icons.delete),
            onPressed: onPressed,
          ),
        ),
      ),
    );
  }
}

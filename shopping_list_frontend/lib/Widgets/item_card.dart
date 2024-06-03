import 'package:flutter/material.dart';
import 'package:shopping_list_frontend/src/rust/api/item_model.dart';

class ItemCard extends StatelessWidget {
  final Item item;
  const ItemCard({super.key, required this.item});

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 2.0,
      margin: const EdgeInsets.symmetric(vertical: 08.0, horizontal: 16.0),
      child: ListTile(
        contentPadding: const EdgeInsets.all(8.0),
        title: Text(item.name,
            style: const TextStyle(fontWeight: FontWeight.bold)),
        subtitle: Text(item.id.oid),
        // subtitle: const Column(
        //   crossAxisAlignment: CrossAxisAlignment.start,
        //   children: <Widget>[
        //     // Text('Muscles Worked: ${muscleWorked.join(', ')}'),
        //     // Text(
        //     //     'Equipment Used: ${equipmentUsed.isNotEmpty ? equipmentUsed.join(', ') : 'None'}'),
        //   ],
        // ),
        trailing: IconButton(
          icon: const Icon(Icons.delete),
          // icon: Icon(Icons.info_outline),
          onPressed: () {},
        ),
      ),
    );
  }
}

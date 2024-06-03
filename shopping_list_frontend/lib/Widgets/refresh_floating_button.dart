import 'package:flutter/material.dart';

class FloatingRefreshButton extends StatelessWidget {
  final VoidCallback onPressedRefresh;
  final VoidCallback onPressedAdd;
  const FloatingRefreshButton(
      {super.key, required this.onPressedRefresh, required this.onPressedAdd});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          FloatingActionButton(
            onPressed: onPressedAdd,
            child: Icon(Icons.add),
          ),
          FloatingActionButton(
            onPressed: onPressedRefresh,
            child: Icon(Icons.refresh_outlined),
          ),
        ],
      ),
    );
  }
}

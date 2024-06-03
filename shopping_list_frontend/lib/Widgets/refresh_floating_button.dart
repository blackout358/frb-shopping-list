import 'package:flutter/material.dart';

class FloatingRefreshButton extends StatelessWidget {
  final VoidCallback onPressed;
  const FloatingRefreshButton({super.key, required this.onPressed});

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: onPressed,
      child: Icon(Icons.refresh_outlined),
    );
  }
}

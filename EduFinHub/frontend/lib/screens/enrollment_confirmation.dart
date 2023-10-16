import 'package:flutter/material.dart';
import 'models/course.dart';

class EnrollmentConfirmationScreen extends StatelessWidget {
  final Course course;

  EnrollmentConfirmationScreen({required this.course});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Enrollment Confirmation'),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Text(
              'Congratulations!',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 10),
            Text(
              'You have successfully enrolled in the following course:',
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 20),
            Text(
              course.name,
              style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
            ),
            SizedBox(height: 10),
            Text(
              course.description,
              style: TextStyle(fontSize: 16),
            ),
            SizedBox(height: 20),
            Text(
              'Price: \$${course.price.toStringAsFixed(2)}',
              style: TextStyle(fontSize: 18),
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: () {
                // Implement navigation to the user's course list or home screen
                Navigator.of(context).pushReplacementNamed('/home');
              },
              child: Text('Back to Home'),
            ),
          ],
        ),
      ),
    );
  }
}

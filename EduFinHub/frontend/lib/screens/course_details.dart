import 'package:flutter/material.dart';

class CourseDetailsScreen extends StatelessWidget {
  final Course course;

  CourseDetailsScreen({required this.course});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(course.name),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Text(
              course.name,
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
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
                // Implement course enrollment logic here
              },
              child: Text('Enroll in Course'),
            ),
          ],
        ),
      ),
    );
  }
}

class Course {
  final String name;
  final String description;
  final double price;

  Course(this.name, this.description, this.price);
}


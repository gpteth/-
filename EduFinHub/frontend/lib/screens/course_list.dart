import 'package:flutter/material.dart';
import 'course_details.dart';

class CourseListScreen extends StatelessWidget {
  final List<Course> courses = [
    Course('Financial Basics', 'Learn financial fundamentals', 100),
    Course('Investment Strategies', 'Master investment techniques', 150),
    // Add more courses here
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Courses'),
      ),
      body: ListView.builder(
        itemCount: courses.length,
        itemBuilder: (context, index) {
          return ListTile(
            title: Text(courses[index].name),
            subtitle: Text(courses[index].description),
            trailing: Text('\$${courses[index].price.toStringAsFixed(2)}'),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => CourseDetailsScreen(course: courses[index]),
                ),
              );
            },
          );
        },
      ),
    );
  }
}

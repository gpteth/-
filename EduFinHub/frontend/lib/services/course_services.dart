import 'models/course.dart';

class CourseService {
  static Future<List<Course>> fetchAllCourses() async {
    // Implement logic to fetch a list of courses from your backend
    // You can make an API call to get the course data
    // For demonstration, return a list of mock courses
    return [
      Course(1, 'Financial Basics', 'Learn financial fundamentals', 100),
      Course(2, 'Investment Strategies', 'Master investment techniques', 150),
      // Add more courses here
    ];
  }

  static Future<Course> enrollInCourse(int courseId) async {
    // Implement logic to enroll the user in a course
    // You can make an API call to enroll the user
    // For demonstration, return the enrolled course
    return Course(courseId, 'Financial Basics', 'Learn financial fundamentals', 100);
  }
}

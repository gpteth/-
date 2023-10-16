import 'models/user.dart';

class UserService {
  static Future<User> fetchUserDetails(int userId) async {
    // Implement logic to fetch user details from your backend
    // You can make an API call to get user information
    // For demonstration, return a mock user object
    return User(userId, 'John Doe', 'john.doe@example.com');
  }
}

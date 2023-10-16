import 'package:flutter/material.dart';
import 'services/auth_service.dart';

class LoginScreen extends StatelessWidget {
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Login'),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: <Widget>[
            TextField(
              controller: _emailController,
              decoration: InputDecoration(labelText: 'Email'),
            ),
            TextField(
              controller: _passwordController,
              obscureText: true,
              decoration: InputDecoration(labelText: 'Password'),
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: () {
                // Call authentication service to log in
                AuthService.login(
                  _emailController.text,
                  _passwordController.text,
                ).then((loggedIn) {
                  if (loggedIn) {
                    Navigator.of(context).pushReplacementNamed('/home');
                  } else {
                    // Show error message
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(
                        content: Text('Invalid credentials. Please try again.'),
                      ),
                    );
                  }
                });
              },
              child: Text('Login'),
            ),
          ],
        ),
      ),
    );
  }
}

# Cipherdial Verification Service

A RESTful API service built with Rust (Axum) following MVC architecture for phone number verification with MySQL database.

Features
1.  MVC Architecture (Model-View-Controller)
2.  RESTful API design
3.  Phone number verification code management
4.  Create or Update verification records (upsert)
5.  Query phone number by username
6.  Data validation
7.  MySQL database integration with SQLx
8.  Comprehensive test coverage
9.  Async/Await support

Tech Stack
1. Framework: Axum 0.7
2. Database: MySQL with SQLx 0.7
3. Runtime: Tokio
4. Validation: Validator
5. UUID: Uuid
6. Time handling: Chrono
7. Configuration: Dotenv
8. Testing: Mockall, Fake, Hyper


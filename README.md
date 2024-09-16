# Rust Chat API

A scalable chat API built with Rust, utilizing WebSockets for real-time communication and MongoDB for persistent message storage.

## Features

- **Real-time chat:** Leverages WebSockets for instant message delivery and updates.
- **Persistent storage:** Stores chat messages in a MongoDB database for future retrieval.
- **Scalable architecture:** Designed with asynchronous programming and modular structure to handle a high volume of concurrent connections.
- **Error handling:** Implements custom error types and centralized error handling for graceful error responses.
- **Logging:** Utilizes structured logging for debugging and monitoring.
- **CI/CD:** Integrates with GitHub Actions for automated build, test, and deployment.

## Prerequisites

- **Rust:** Make sure you have Rust and Cargo installed on your system. You can follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **MongoDB:** You'll need a running MongoDB instance. You can download and install it from [https://www.mongodb.com/try/download/community](https://www.mongodb.com/try/download/community)

## Setup

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd chat-app
   ```

2. **Set up environment variables:**

    - Create a `.env` file in the project's root directory.
    - Add the following environment variables:

        ```bash
        MONGODB_URI="mongodb://localhost:27017"  # Replace with your MongoDB connection URI
        MONGODB_DATABASE="chat_app"              # Replace with your desired database name
        MONGODB_COLLECTION="messages"            # Replace with your desired collection name
        SERVER_URL="127.0.0.1"                   # Replace with your server's URL
        SERVER_PORT="8080"                       # Replace with your desired port number
        RUST_LOG="info"                          # Set the desired log level (info, debug, etc.)
        ```

3. **Build and run:**

    ```bash
    make build  # Build the project
    make run    # Run the server
    ```

## Usage

1. **Connect to the WebSocket server:**

    - Use a WebSocket client library or tool to connect to `ws://<SERVER_URL>:<SERVER_PORT>` (replace `<SERVER_URL>` and `<SERVER_PORT>` with the values from your `.env` file).

2. **Send and receive messages:**

    - The server expects messages in the following JSON format:

        ```json
        {
            "username": "your_username",
            "content": "your_message",
            "timestamp": 1694844257
        }
        ```

    - When a client sends a message, the server will broadcast the updated list of all messages to all connected clients.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the Apache-2.0 License.

**Remember:**

- Replace placeholders like `<repository-url>`, `your_username`, and `your_message` with actual values.
- Customize the `README.md` further to include any additional information relevant to your project, such as specific features, configuration options, or deployment instructions.

Feel free to ask if you have any more questions or modifications!

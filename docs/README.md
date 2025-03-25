# MCP Command Server Documentation

## Postman Collection

This folder contains a complete Postman collection for testing and interacting with the MCP Command Server API. The collection includes examples, test cases, and validation scripts to ensure the API works as expected.

### Collection Overview

The Postman collection (`mcp_command_server.postman_collection.json`) is organized into the following sections:

1. **API Documentation** - Access the API context documentation in markdown format.
2. **Basic Commands** - Simple system commands like listing files, checking current directory, and system information.
3. **File Operations** - Creating, reading, and deleting files through the API.
4. **Error Cases** - Testing error handling for invalid methods, missing parameters, and execution errors.
5. **Advanced Commands** - More complex commands for monitoring system resources.

### How to Use the Collection

1. **Import the Collection**:
   - Open Postman
   - Click "Import" button
   - Browse to the `mcp_command_server.postman_collection.json` file
   - Click "Import"

2. **Set up the Environment**:
   - The collection uses a variable `{{base_url}}` which is set to `http://localhost:3030` by default
   - You can create a new environment or modify this variable if your server runs on a different host/port

3. **Run Individual Requests**:
   - Navigate through the folders to find the request you want to test
   - Each request has an example response and description of what it does
   - Click "Send" to execute the request

4. **Run Test Suite**:
   - You can run all tests by clicking the "Run" button on the collection
   - The tests validate error handling, correct responses, and edge cases

### Tests Included

The collection contains automated tests that verify:

- Response status codes
- JSON-RPC 2.0 format compliance
- Error handling for invalid methods and parameters
- Command execution and output validation
- File operation success verification

### Example Use Cases

The collection demonstrates how to:

1. Execute basic commands (`ls`, `pwd`, `uname`)
2. Create, read, and delete files
3. Check system resources (processes, disk usage, memory)
4. Handle various error conditions

### API Context Endpoint

The `/context` endpoint provides API context documentation in markdown format. This endpoint can be used to access information about the API, including available endpoints, methods, and parameters.

## Security Considerations

The current implementation executes commands directly without restrictions, which could be a security risk in production environments. Before deploying to production, consider:

1. Implementing authentication/authorization
2. Restricting commands that can be executed
3. Setting up proper input validation and sanitization

## Docker Integration

If you're using the Docker setup provided with the project, the API will be accessible at `http://localhost:3030` when the container is running.

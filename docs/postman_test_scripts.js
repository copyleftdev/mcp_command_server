/**
 * MCP Command Server Postman Test Scripts
 * 
 * This file contains reusable test scripts that can be copied into Postman requests
 * for more advanced testing scenarios.
 */

// Basic JSON-RPC Response Structure Test
pm.test("Response has valid JSON-RPC structure", function () {
    const jsonData = pm.response.json();
    pm.expect(jsonData).to.have.property('jsonrpc');
    pm.expect(jsonData.jsonrpc).to.eql('2.0');
    pm.expect(jsonData).to.have.property('id');
    pm.expect(jsonData).to.satisfy((data) => {
        return data.hasOwnProperty('result') || data.hasOwnProperty('error');
    });
});

// Success Response Test
pm.test("Command executed successfully", function () {
    const jsonData = pm.response.json();
    pm.expect(jsonData).to.have.property('result');
    pm.expect(jsonData.result).to.have.property('stdout');
    pm.expect(jsonData.error).to.be.null;
});

// Error Response Test
pm.test("Error response is properly formatted", function () {
    const jsonData = pm.response.json();
    pm.expect(jsonData).to.have.property('error');
    pm.expect(jsonData.error).to.have.property('code');
    pm.expect(jsonData.error).to.have.property('message');
    pm.expect(jsonData.result).to.be.null;
});

// Performance Test
pm.test("Response time is acceptable", function () {
    pm.expect(pm.response.responseTime).to.be.below(500);
});

// Content Type Test
pm.test("Content-Type header is application/json", function () {
    pm.expect(pm.response.headers.get('Content-Type')).to.include('application/json');
});

// Specific Command Output Tests
pm.test("Command output contains expected text", function () {
    const jsonData = pm.response.json();
    // Replace 'expected text' with the actual text you expect to find
    pm.expect(jsonData.result.stdout).to.include('expected text');
});

// Environment Variable Setting
// This can be used to save values from one request to use in subsequent requests
if (pm.response.code === 200) {
    const jsonData = pm.response.json();
    if (jsonData.result && jsonData.result.stdout) {
        // Extract some value from stdout and save it as an environment variable
        const value = jsonData.result.stdout.trim();
        pm.environment.set("extracted_value", value);
    }
}

// Pre-request Script for dynamic command generation
// This can be used in the Pre-request Script tab to generate dynamic commands
const timestamp = new Date().toISOString();
const dynamicCommand = `echo "Test run at ${timestamp}" > /tmp/postman-dynamic-test.txt`;
pm.variables.set("dynamic_command", dynamicCommand);

// Advanced Test: JSON Schema Validation
const schema = {
    "type": "object",
    "required": ["jsonrpc", "id", "result"],
    "properties": {
        "jsonrpc": { "type": "string", "enum": ["2.0"] },
        "id": { "type": "integer" },
        "result": {
            "type": "object",
            "required": ["stdout"],
            "properties": {
                "stdout": { "type": "string" }
            }
        },
        "error": { "type": "null" }
    }
};

pm.test("Response matches JSON schema", function () {
    const jsonData = pm.response.json();
    pm.expect(tv4.validate(jsonData, schema)).to.be.true;
});

// Test for idempotent commands (should return same result when run multiple times)
pm.test("Command is idempotent", function () {
    const previousOutput = pm.variables.get("previous_command_output");
    const currentOutput = pm.response.json().result.stdout;
    
    if (previousOutput) {
        pm.expect(currentOutput).to.eql(previousOutput);
    }
    
    pm.variables.set("previous_command_output", currentOutput);
});

// Transaction flow test - to be used across multiple requests
// First request sets a value
// if (pm.info.requestName === "Create File (echo)") {
//     pm.variables.set("test_file_created", true);
// }

// Second request checks if the file was created
// if (pm.info.requestName === "Read File (cat)" && pm.variables.get("test_file_created")) {
//     pm.test("File was successfully created in previous step", function() {
//         const jsonData = pm.response.json();
//         pm.expect(jsonData.error).to.be.null;
//     });
// }

/**
 * Collection-level test scripts
 * These can be added to the entire collection to run before/after all requests
 */

// Pre-request script for the collection
// console.log("Starting test run at " + new Date().toISOString());
// pm.environment.set("test_run_id", Date.now());

// Test script for the collection
// console.log("Test run completed at " + new Date().toISOString());
// console.log("Test run ID: " + pm.environment.get("test_run_id"));

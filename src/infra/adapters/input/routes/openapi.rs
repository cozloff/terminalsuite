use axum::Json;
use serde_json::{json, Value};

pub async fn api_openapi() -> Json<Value> {
    Json(json!({
        "openapi": "3.1.0",
        "info": {
            "title": "TerminalSuite API",
            "version": "0.1.0",
            "description": "TerminalSuite API documentation."
        },
        "paths": {
            "/api/test": {
                "get": {
                    "summary": "Test Endpoint",
                    "description": "Just a test endpoint.",
                    "responses": {
                        "200": {
                            "description": "Successful response.",
                            "content": {
                                "application/json": {
                                    "schema": { 
                                        "$ref": "#/components/schemas/TestResponse" 
                                    }
                                }
                            }
                        }
                    }
                }
            },
        },
        "components": {
            "schemas": {
                "TestResponse": {
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "example": "Hello, World!"
                        }
                    }
                }
            }
        }
    }))
}
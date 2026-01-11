// go のwebサーバー
package main

import (
	"log"
	"os"
	"path/filepath"
	"github.com/gofiber/fiber/v2"
)

func main() {
	// Create a new Fiber instance
	app := fiber.New()

	// Get the directory path for ../appvue/dist
	// This will work when the app is run from the project root
	distPath, err := filepath.Abs("../vueapp/dist")
	if err != nil {
		log.Fatal("Failed to resolve dist path:", err)
	}

	// Check if the directory exists
	if _, err := os.Stat(distPath); os.IsNotExist(err) {
		log.Printf("Warning: dist directory does not exist at %s. This is fine if you're building the frontend separately.", distPath)
	}

	// Serve static files from ../appvue/dist
	// Note: Using a simple route to serve files from a specific directory
	app.Static("/", distPath)

	// Define a simple route for testing
	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World! Static files are being served from ../appvue/dist")
	})

	// Define API test endpoint
	app.Get("/api/test", func(c *fiber.Ctx) error {
		return c.SendString("OK")
	})

	// Start the server on port 3000
	log.Println("Server starting on http://localhost:3000")
	log.Println("Serving static files from:", distPath)
	if err := app.Listen(":3000"); err != nil {
		log.Fatal("Error starting server:", err)
	}
}

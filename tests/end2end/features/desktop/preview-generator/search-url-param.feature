Feature: Search a brand via URL parameter
	As a user
	I want to load a specific brand in the preview generator via URL parameter
	So that I can quickly see the preview of that brand

	Background:
		Given I see the preview generator page with the url params q=rust

	Scenario: Load "Rust" brand via URL parameter
		Then the brand input value is "Rust"
		Then the color input value is "000000"
		Then the path input value starts with "M23.83"
		Then the title in the preview is "Rust Preview"
		Then the filename in the preview is "rust.svg"
		Then the brand in the preview is "Brand: Rust"
		Then the color in the preview is "Color: #000000"
		Then the background color of the preview is #000000
		Then the SVG paths of the preview start with "M23.83"
